# Error Handling

## Basic Error Handling

Error handling in Rust can be clumsy if you can't use the question-mark operator.
To achieve happiness, we need to return a `Result` which can accept any error.
All errors implement the trait `std::error::Error`, and
so _any_ error can convert into a `Box<Error>`.

Say we needed to handle _both_ i/o errors and errors from converting
strings into numbers:

```rust
// box-error.rs
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

fn run(file: &str) -> Result<i32,Box<Error>> {
    let mut file = File::open(file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents.trim().parse()?)
}
```
So that's two question-marks for the i/o errors (can't open file, or can't read as string)
and one question-mark for the conversion error. Finally, we wrap the result in `Ok`.
Rust can work out from the return type that `parse` should convert to `i32`.

It's easy to create a shortcut for this `Result` type:

```rust
type BoxResult<T> = Result<T,Box<Error>>;
```

However, our programs will have application-specific error conditions, and so
we need to create our own error type. The basic requirements
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

Typing `Result<T,MyError>` gets tedious and many Rust modules define their own
`Result` - e.g. `io::Result<T>` is short for `Result<T,io::Error>`.

In this next example we need to handle the specific error when a string can't be parsed
as a floating-point number.

Now the way that `?` works
is to look for a conversion from the error of the _expression_ to the error that must
be _returned_.  And this conversion is expressed by the `From` trait. `Box<Error>`
works as it does because it implements `From` for all types implementing `Error`.

At this point you can continue to use the convenient alias `BoxResult` and catch everything
as before; there will be a conversion from our error into `Box<Error>`.
This is a good option for smaller applications. But I want to show other errors can
be explicitly made to cooperate with our error type.

`ParseFloatError` implements `Error` so `description()` is defined.

```rust
use std::num::ParseFloatError;

impl From<ParseFloatError> for MyError {
    fn from(err: ParseFloatError) -> Self {
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
second `?` will convert the `ParseFloatError` to `MyError`.

And the results:

```rust
fn main() {
    println!(" {:?}", parse_f64("42",false));
    println!(" {:?}", parse_f64("42",true));
    println!(" {:?}", parse_f64("?42",false));
}
//  Ok(42)
//  Err(MyError { details: "borked" })
//  Err(MyError { details: "invalid float literal" })
```

Not too complicated, although a little long-winded. The tedious bit is having to
write `From` conversions for all the other error types that need to play nice
with `MyError` - or you simply lean on `Box<Error>`. Newcomers get confused
by the multitude of ways to do the same thing in Rust; there is always another
way to peel the avocado (or skin the cat, if you're feeling bloodthirsty). The price
of flexibility is having many options. Error-handling for a 200 line program can afford
to be simpler than for a large application. And if you ever want to package your precious
droppings as a Cargo crate, then error handling becomes crucial.

Currently, the question-mark operator only works for `Result`, not `Option`, and this is
a feature, not a limitation.  `Option` has a `ok_or_else` which converts itself into a `Result`.
For example, say we had a `HashMap` and must fail if a key isn't defined:

```rust
    let val = map.get("my_key").ok_or_else(|| MyError::new("my_key not defined"))?;
```

Now here the error returned is completely clear! (This form uses a closure, so the error value
is only created if the lookup fails.)

## simple-error for Simple Errors

The [simple-error](https://docs.rs/simple-error/0.1.9/simple_error/) crate provides you with
a basic error type based on a string, as we have defined it here, and a few convenient macros.
Like any error, it works fine with `Box<Error>`:

```rust
use simple_error::bail;

use std::error::Error;

type BoxResult<T> = Result<T,Box<dyn Error>>;

fn run(s: &str) -> BoxResult<i32> {
    if s.len() == 0 {
        bail!("empty string");
    }
    Ok(s.trim().parse()?)
}

fn main() {
    println!("{:?}", run("23"));
    println!("{:?}", run("2x"));
    println!("{:?}", run(""));
}
// Ok(23)
// Err(ParseIntError { kind: InvalidDigit })
// Err(StringError("empty string"))

```

`bail!(s)` expands to `return SimpleError::new(s).into();` - return early with a conversion _into_
the receiving type.

You need to use `BoxResult` for mixing the `SimpleError` type with other errors, since
we can't implement `From` for it, since both the trait and the type come from other crates.

## error-chain for Serious Errors

For non-trivial applications have a look
at the [error_chain](http://brson.github.io/2016/11/30/starting-with-error-chain) crate.
A little macro magic can go a long way in Rust...

Create a binary crate with `cargo new --bin test-error-chain` and
change to this directory. Edit `Cargo.toml` and add `error-chain="0.8.1"` to the end.

What __error-chain__ does for you is create all the definitions we needed for manually implementing
an error type; creating a struct, and implementing the necessary traits: `Display`, `Debug` and `Error`.
It also by default implements `From` so strings can be converted into errors.

Our first `src/main.rs` file looks like this. All the main program does is call `run`, print out any
errors, and end the program with a non-zero exit code.  The macro `error_chain` generates all the
definitions needed, within an `error` module - in a larger program you would put this in its own file.
We need to bring everything in `error` back into global scope because our code will need to see
the generated traits. By default, there will be an `Error` struct and a `Result` defined with that
error.

Here we also ask for `From` to be implemented so that `std::io::Error` will convert into
our error type using `foreign_links`:

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
    if let Err(e) = run() {
        println!("error: {}", e);

        std::process::exit(1);
    }
}
// error: No such file or directory (os error 2)
```

The 'foreign_links' has made our life easier, since the question mark operator now knows how to
convert `std::io::Error` into our `error::Error`.  (Under the hood, the macro is creating a
`From<std::io::Error>` conversion, exactly as spelt out earlier.)

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
        println!("{}", line);
        l += 1;
        if l == 10 {
            break;
        }
    }

    Ok(())
}
```

There is (again) a useful little macro `bail!` for 'throwing' errors.
An alternative to the `ok_or` method here could be:

```rust
    let file = match args().skip(1).next() {
        Some(s) => s,
        None => bail!("provide a file")
    };
```

Like `?` it does an _early return_.

The returned error contains an enum `ErrorKind`, which allows us to distinguish between various
kinds of errors. There's always a variant `Msg` (when you say `Error::from(str)`) and the `foreign_links`
has declared `Io` which wraps I/O errors:

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
            &ErrorKind::Msg(ref s) => println!("msg {}", s),
            &ErrorKind::Io(ref s) => println!("io {}", s),
            &ErrorKind::NoArgument(ref s) => println!("no argument {:?}", s),
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
where you match on exception types in a `catch` or `except` block.

In summary, __error-chain__ creates a type `Error` for you, and defines `Result<T>` to be `std::result::Result<T,Error>`.
`Error` contains an enum `ErrorKind` and by default there is one variant `Msg` for errors created from
strings. You define external errors with `foreign_links` which does two things. First, it creates a new
`ErrorKind` variant. Second, it defines `From` on these external errors so they can be converted to our
error.  New error variants can be easily added.  A lot of irritating boilerplate code is eliminated.

## Chaining Errors

But the really cool thing that this crate provides is _error chaining_.

As a _library user_, it's irritating when a method simply just 'throws' a generic I/O error. OK, it
could not open a file, fine, but what file? Basically, what use is this information to me?

`error_chain` does _error chaining_ which helps solve this problem of over-generic errors. When we
try to open the file, we can lazily lean on the conversion to `io::Error` using `?`, or _chain_ the error.

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
        println!("{}", line);
        l += 1;
        if l == 10 {
            break;
        }
    }

    Ok(())
}


fn main() {
    if let Err(e) = run() {
        println!("error {}", e);

        /////// look at the chain of errors... ///////
        for e in e.iter().skip(1) {
            println!("caused by: {}", e);
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

(`run` is where all the action takes place, anyway.)

