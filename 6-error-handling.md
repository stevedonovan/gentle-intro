## Error Handling

Error handling in Rust can be clumsy if you can't use the question-mark operator.
To achieve happiness, we need to create our own error type. The basic requirements
are straightforward:

  - May implement `Debug`
  - Must implement `Display`
  - Must implement `Error`

Otherwise, your error can do pretty much what it likes.

```rust
// error1.rs
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct MyError {
    details: String
}

impl MyError {
    fn new(msg: &str) -> MyError {
        MyError{details: msg.to_string()}
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}


impl Error for MyError {
    fn description(&self) -> &str {
        &self.details
    }
}

// a test function that returns our error result
fn raises_my_error(yes: bool) -> Result<(),MyError> {
    if yes {
        Err(MyError::new("borked"))
    } else {
        Ok(())
    }
}
```

In this example we need to handle the specific error when a string can't be parsed
as a floating-point number. It implements `Error` so `description()` is defined.

 Now the way that `?` works
is to look for a conversion from the error of the expression to the error that must
be returned. And this conversion is expressed by the `From` trait:

```rust
impl From<std::num::ParseFloatError> for MyError {
    fn from(err: std::num::ParseFloatError) -> Self {
        MyError::new(err.description())
    }
}

// and test!
fn parse_f64(s: &str, yes: bool) -> Result<f64,MyError> {
    raises_my_error(yes)?;
    let x: f64 = s.parse()?;
    Ok(x)
}
```

The first `?` is fine (a type always converts to itself with `From`) and the
second `?` needs to convert from `ParseFloatError` to `MyError`.

And the results:

```rust
fn main() {
    println!(" {:?}",parse_f64("42",false));
    println!(" {:?}",parse_f64("42",true));
    println!(" {:?}",parse_f64("?42",false));
}
//  Ok(42)
//  Err(MyError { details: "borked" })
//  Err(MyError { details: "invalid float literal" })
```

Not too complicated, although a little long-winded. The tedious bit is having to
write `From` conversions for all the other error types that need to play nice
with `MyError`.  But once the mechanism is in place, your error handling looks
much cleaner!

Typing `Result<T,MyError>` gets tedious and many Rust modules define their own
`Result` - e.g. `io::Result<T>` is short for `io::Result<T,io::Error>`.

Currently, the question-mark operator only works for `Result`, not `Option`, and this can be seen
as a feature, not a limitation.  `Option` has a `ok_or_else` which converts itself into a `Result`.
For example, say we had a `HashMap` and must fail if a key isn't defined:


```rust
    let val = map.get("my_key").ok_or_else(|| MyError::new("my_key not defined"))?;
```

Now here the error returned is completely clear! (This form uses a closure, so the error value
is only created if the lookup fails.)

Thinking about error handling is important, and for serious applications have a look
at the [error_chain](http://brson.github.io/2016/11/30/starting-with-error-chain) crate.
A little macro magic can go a long way in Rust...

Create a binary crate with `cargo new --bin test-error-chain` and
change to this directory. Edit `Cargo.toml` and add `error-chain="*"` to the end.

What __error-chain__ does for you is create all the definitions we needed for manually implementing
an error type; creating a struct, and implementing the necessary traits: `Display`, `Debug` and `Error`.
It also by default implements `From` so strings can be converted into errors. Here we also ask for
`From` to be implemented so that `std::io::Error` will also convert into our error type.

Our first `src/main.rs` file looks like this. All the main program does is call `run`, print out any
errors, and end the program with a non-zero exit code.  The macro `error_chain` generates all the
definitions needed, within an `error` module - in a larger problem you would put this in its own file.
We need to bring everything in `error` back into global scope because our code will need to see
the generated traits. By default, there will be an `Error` struct and a `Result` defined with that
error:

```rust
#[macro_use]
extern crate error_chain;

mod errors {
    error_chain!{
        foreign_links {
            Io(::std::io::Error);
        }
    }
}
use errors::*;

fn run() -> Result<()> {
    use std::fs::File;

    File::open("file")?;

    Ok(())
}


fn main() {
    if let Err(ref e) = run() {
        println!("error: {}", e);

        std::process::exit(1);
    }
}
// error: No such file or directory (os error 2)
```

The 'foreign_links' has made our life easier, since the question mark operator now knows how to
convert `std::io::Error` into our `error::Error`.

All the action happens in `run`; let's make it print out the first 10 lines of a file given as the
first program argument.  There may or may not be such an argument, which isn't necessarily an
error. Here we want to convert an `Option<String>` into a `Result<String>`. There are two `Option`
methods for doing this conversion, and I've picked the simplest one.  Our `Error` type implements
`From` for `&str`, so it's straightforward to make an error with a simple text message.

```rust
fn run() -> Result<()> {
    use std::env::args;
    use std::fs::File;
    use std::io::BufReader;
    use std::io::prelude::*;

    let file = args().skip(1).next()
        .ok_or(Error::from("provide a file"))?;

    let f = File::open(&file)?;
    let mut l = 0;
    for line in BufReader::new(f).lines() {
        let line = line?;
        println!("{}",line);
        l += 1;
        if l == 10 {
            break;
        }
    }

    Ok(())
}
```

We can freely use `?` for the I/O errors, which is the state of happiness we desired. There is a useful
little macro `bail!` for 'throwing' errors. An alternative to the `ok_or` method here could be:

```rust
    let file = match args().skip(1).next() {
        Some(s) => s,
        None => bail!("provide a file")
    };
```

Like `?` it does an _early return_.

The returned error contains an enum `ErrorKind`, which allows us to distinguish between various
kinds of errors. There's always a variant `Msg` (when you say `Error::from(str)`) and the `foreign_links`
macro has declared `Io` which wraps I/O errors:

```rust
fn main() {
    if let Err(e) = run() {
        match e.kind() {
            &ErrorKind::Msg(ref s) => println!("msg {}",s),
            &ErrorKind::Io(ref s) => println!("io {}",s),
        }
        std::process::exit(1);
    }
}
// $ cargo run
// msg provide a file
// $ cargo run foo
// io No such file or directory (os error 2)
```

It's straightforward to add new kinds of errors. Add an `errors` section to the `error_chain!` macro:

```rust
    error_chain!{
        foreign_links {
            Io(::std::io::Error);
        }

        errors {
            NoArgument(t: String) {
                display("no argument provided: '{}'", t)
            }
        }

    }
```

This defines how `Display` works for this new kind of error. And now we can handle
'no argument' errors more specifically, feeding `ErrorKind::NoArgument` a `String` value:

```rust
    let file = args().skip(1).next()
        .ok_or(ErrorKind::NoArgument("filename needed".to_string()))?;

```

There's now an extra `ErrorKind` variant that you must match:

```rust
fn main() {
    if let Err(e) = run() {
        println!("error {}",e);
        match e.kind() {
            &ErrorKind::Msg(ref s) => println!("msg {}",s),
            &ErrorKind::Io(ref s) => println!("io {}",s),
            &ErrorKind::NoArgument(ref s) => println!("no argument {:?}",s),
        }
        std::process::exit(1);
    }
}
// cargo run
// error no argument provided: 'filename needed'
// no argument "filename needed"
```

Generally, it's useful to make errors as specific as possible, _particularly_ if this is a library
function! This match-on-kind technique is pretty much the equivalent of traditional exception handling,
where you match on exception types.

As a _library user_, it's irritating when a method simply just 'throws' a generic I/O error. OK, it
could not open a file, fine, but what file? Basically, what use is this information to me?

`error_chain` does _error chaining_ which helps solve this problem of over-generic errors. When we
try to open the file, we just lazily leant on the conversion to `io::Error` using `?`.

```rust
// non-specific error
let f = File::open(&file)?;

// a specific chained error
let f = File::open(&file).chain_err(|| "unable to read the damn file")?;
```

Here's a new version of the program, with _no_ imported 'foreign' errors, just the defaults:

```rust
#[macro_use]
extern crate error_chain;

mod errors {
    error_chain!{
    }

}
use errors::*;

fn run() -> Result<()> {
    use std::env::args;
    use std::fs::File;
    use std::io::BufReader;
    use std::io::prelude::*;

    let file = args().skip(1).next()
        .ok_or(Error::from("filename needed"))?;

    ///////// chain explicitly! ///////////
    let f = File::open(&file).chain_err(|| "unable to read the damn file")?;

    let mut l = 0;
    for line in BufReader::new(f).lines() {
        let line = line.chain_err(|| "cannot read a line")?;
        println!("{}",line);
        l += 1;
        if l == 10 {
            break;
        }
    }

    Ok(())
}


fn main() {
    if let Err(e) = run() {
        println!("error {}",e);

        /////// look at the chain of errors... ///////
        for e in e.iter().skip(1) {
            println!("caused by: {}",e);
        }

        std::process::exit(1);
    }
}
// $ cargo run foo
// error unable to read the damn file
// caused by: No such file or directory (os error 2)
```

So the `chain_err` method takes the original error, and creates a new error which contains the
original error - this can be continued indefinitely.  The closure is expected to return any
value which can be _converted_ into an error.

Rust macros can clearly save you a lot of typing.  `error-chain` even provides a shortcut that
replaces the whole main program:

```rust
quick_main!(run);
```

So `run` is where all the action takes place.


