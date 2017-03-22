# Filesystem and Processes

## Another look at Reading Files

At the end of Part 1, I showed how to read a whole file into a string. Naturally
this isn't always such a good idea, so here is how to read a file line-by-line.

`fs::File` implements `io::Read`, which is the trait for anything readable.
This trait defines a `read` method which will fill a slice of `u8` with bytes -
this is the only _required_ method of the trait, and you get some _provided_ methods
for free, much like with `Iterator`.  You can use `read_to_end` to fill a vector of
bytes with contents from the readable, and `read_to_string` to fill a string - which
must be UTF-8 encoded.

This is a 'raw' read, with no buffering. For buffered reading there is the
`io::BufRead` trait which gives us `read_line` and a `lines` iterator.
`io::BufReader` will provide an implementation of `io::BufRead` for _any_ readable.

`fs::File` _also_ implements `io::Write`.

The easiest way to make sure all these traits are visible is `use std::io::prelude::*`.


```rust
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn read_all_lines(filename: &str) -> io::Result<()> {
    let file = File::open(&filename)?;

    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        println!("{}", line);
    }
    Ok(())
}
```

The `let line = line?` may look a bit strange. The `line` returned by the
iterator is actually an `io::Result<String>` which we unwrap with `?`.
Because things _can_ go wrong during this iteration - I/O errors, swallowing
a chunk of bytes that aren't UTF-8, and so forth.

`lines` being an iterator, it is straightforward to read a file into a vector
of strings using `collect`, or print out the line with line numbers using the
`enumerate` iterator.

It isn't the most efficient way to read all the lines, however, because a new
string is allocated for each line. It is more efficient to use `read_line`, although
more awkward. Note that the returned line includes the linefeed, which
can be removed using `trim_right`.

```rust
    let mut reader = io::BufReader::new(file);
    let mut buf = String::new();
    while reader.read_line(&mut buf)? > 0 {
        {
            let line = buf.trim_right();
            println!("{}", line);
        }
        buf.clear();
    }
```

This results in far less allocations, because _clearing_ that string does not free its
allocated memory; once the string has enough capacity, no more allocations will take
place.

This is one of those cases where we use a block to control a borrow. `line` is
borrowed from `buf`, and this borrow must finish before we modify `buf`.  Again,
Rust is trying to stop us doing something stupid, which is to access `line` _after_
we've cleared the buffer. (The borrow checker can be restrictive sometimes.
Rust is due to get 'non-lexical lifetimes' this year, where
it will analyze the code and see that `line` isn't used after `buf.clear()`.)

This isn't very pretty. I cannot give you a proper iterator that returns references
to a buffer, but I can give you something that _looks_ like an iterator.

First define a generic struct;
the type parameter `R` is 'any type that implements Read'. It contains the reader
and the buffer which we are going to borrow from.

```rust
// file5.rs
use std::fs::File;
use std::io;
use std::io::prelude::*;

struct Lines<R> {
    reader: io::BufReader<R>,
    buf: String
}

impl <R: Read> Lines<R> {
    fn new(r: R) -> Lines<R> {
        Lines{reader: io::BufReader::new(r), buf: String::new()}
    }
    ...
}
```

Then the `next` method. It returns an `Option` - just like an iterator, when it
returns `None` the iterator finishes. The returned type is a `Result` because
`read_line` might fail, and we _never throw errors away_. So if fails, we
wrap up its error in a `Some<Result>`.  Otherwise, it may have read zero bytes,
which is the natural end of the file - not an error, just a `None`.

At this point, the buffer contains the line with a linefeed (`\n') appended.
Trim this away, and package up the string slice.

```rust
    fn next<'a>(&'a mut self) -> Option<io::Result<&'a str>>{
        self.buf.clear();
        match self.reader.read_line(&mut self.buf) {
            Ok(nbytes) => if nbytes == 0 {
                None // no more lines!
            } else {
                let line = self.buf.trim_right();
                Some(Ok(line))
            },
            Err(e) => Some(Err(e))
        }
    }
```
Now, note how the lifetimes work. We need an explicit lifetime because Rust will never
allow us to hand out borrowed string slices without knowing their lifetime. And here
we say that the lifetime of this borrowed string is within the lifetime of `self`.

And this signature, with the lifetime, is incompatible with the interface of `Iterator`.
But it's easy to see problems if it were compatible; consider `collect` trying to make
a vector of these string slices. There's no way this could work, since they're all
borrowed from the same mutable string! (If you had read _all_ the file into a string, then
the string's `lines` iterator works fine because the string slices are all borrowed from
the original string.)

The resulting loop is much cleaner, and the file buffering is invisible to the user.

```
fn read_all_lines(filename: &str) -> io::Result<()> {
    let file = File::open(&filename)?;

    let mut lines = Lines::new(file);
    while let Some(line) = lines.next() {
        let line = line?;
        println!("{}", line)?;
    }

    Ok(())
}
```

You can even write the loop like this, since the explicit match can pull out the
string slice:

```
    while let Some(Ok(line)) = lines.next() {
        println!"{}", line)?;
    }
```

It's tempting, but you are throwing away a possible error here; this loop will
silently stop whenever an error occurs. In particular, it will stop at the first place
where Rust can't convert a line to UTF-8.  Fine for casual code, bad for production code!

## Writing To Files

We met the `write!` macro when implementing `Display` - it also works with anything
that implements `Write`. So here's a another way of saying `print!`:

```rust
    let mut stdout = io::stdout();
    ...
    write!(stdout,"answer is {}\n", 42).expect("write failed");
```

If an error is _possible_, you must handle it. It may not be
very _likely_ but it can happen. It's usually fine, because if you
are doing file i/o you should be in a context where `?` works.

`print!` works fine as it is, but for arbitrary files we need `write!`. The
file is closed when `out` is dropped at the end of `write_out`, which is
convenient and often important.

```rust
// file6.rs
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn write_out(f: &str) -> io::Result<()> {
    let mut out = File::create(f)?;
    write!(out,"answer is {}\n", 42)?;
    Ok(())
}

fn main() {
  write_out("test.txt").expect("write failed");
}
```

Another place we need it is writing to 'standard error'. There is one input stream
`io::stdin()` and _two_ output streams `io::stderr()` and `io::stdout()`. If a program
needs to complain bitterly, the convention is that this output should go to the
error stream. If the program wants then to stop running, the convention is also to
return a non-zero exit code.

```rust
fn quit(msg: &str) -> ! {
    write!(io::stderr(),"error: {}\n", msg).expect("write?");
    std::process::exit(1);
}
```
(The `!` means that function never returns, so you can call `quit` anywhere
and there will be no type mismatch complaints)

## Files, Paths and Directories

Here is a little program for printing out the Cargo directory on a machine. The
simplest case is that it's '~/.cargo'. This is a Unix shell expansion,
so we use `env::home_dir` because it's cross-platform. (It might fail, but a
computer without a home directory isn't going to be hosting Rust tools anyway.)

We then create a [PathBuf](https://doc.rust-lang.org/std/ops/trait.Mul.html)
and use its `push` method to build up the full file path from its _components_.
(This is much easier than fooling around with '/','\' or whatever, depending on
the system.)

```rust
// file7.rs
use std::env;
use std::path::PathBuf;

fn main() {
    let home = env::home_dir().expect("no home!");
    let mut path = PathBuf::new();
    path.push(home);
    path.push(".cargo");

    if path.is_dir() {
        println!("{}", path.display());
    }
}
```
A `PathBuf` is like `String` - it owns a growable set of characters, but with methods
specialized to building up paths.  Most of its functionality however comes from
the borrowed version `Path`, which is like `&str`.  So, for instance, `is_dir` is
a `Path` method.

This might sound suspiciously like a form of inheritance, but the magic [Deref](https://doc.rust-lang.org/book/deref-coercions.html)
trait works differently. It works just like it does with `String/&str` -
a reference to `PathBuf` can be _coerced_ into a reference to `Path`.
('Coerce' is a strong word, but this really
is one of the few places where Rust does conversions for you.)

```rust
fn foo(p: &Path) {...}
...
let path = PathBuf::from(home);
foo(&path);
```

`PathBuf` has an intimate relationship with `OsString`, which represents strings we get
directly from the system. (There is a corresponding `OsString/&OsStr` relationship.)

Such strings are not _guaranteed_ to be representable as UTF-8!
Real life is a [complicated matter](https://news.ycombinator.com/item?id=10519932),
particularly see the answer to 'Why are they so hard?'.  To summarize, first there are
years of ASCII legacy coding, and multiple special encodings for other languages. Second,
human languages are complicated. For instance 'noël' is _five_ Unicode code points!

It's true that _most_ of the time
with modern operating systems file names will be Unicode (UTF-8 on the Unix side, UTF-16
for Windows), except when they're not. And Rust must handle that possibility
rigorously. For instance,
`Path` has a method `as_os_str` which returns a `&OsStr`, but the `to_str` method
returns an `Option<&str>`. Not always possible!

People have trouble at this point because they have become too attached to 'string' and
'character' as the only necessary abstractions.  As Einstein could have said, a programming language
has to be as simple as possible, but no simpler. A systems language _needs_ a
`String/&str` distinction (owned versus borrowed: this is also very _convenient_)
and if it wishes to standardize on Unicode strings then it needs another type to handle
text which isn't valid Unicode - hence `OsString/&OsStr`.

People are also very used to processing filenames as if they were text, which is why
Rust makes it easier to manipulate file paths using `PathBuf` methods.

You can `pop` to successively remove path components. Here we start with the
current directory of the program:

```
// file8.rs
use std::env;

fn main() {
    let mut path = env::current_dir().expect("can't access current dir");
    loop {
        println!("{}", path.display());
        if ! path.pop() {
            break;
        }
    }
}
// /home/steve/rust/gentle-intro/src
// /home/steve/rust/gentle-intro
// /home/steve/rust
// /home/steve
// /home
// /
```

Here's a useful variation. I have a program which searches for a configuration file,
and the rule is that it may appear in any subdirectory of the current directory.
So I create `/home/steve/rust/config.txt` and start this program up in `/home/steve/rust/gentle-intro/src`:

```rust
// file9.rs
use std::env;

fn main() {
    let mut path = env::current_dir().expect("can't access current dir");
    loop {
        path.push("config.txt");
        if path.is_file() {
            println!("gotcha {}", path.display());
            break;
        } else {
            path.pop();
        }
        if ! path.pop() {
            break;
        }
    }
}
// gotcha /home/steve/rust/config.txt
```

This is pretty much how __git__ works when it wants to know what the current repo is.

The details about a file (its size, type, etc) are called its _metadata_. As always,
there may be an error - not just 'not found' but also if we don't have permission
to read this file.

```rust
// file10.rs
use std::env;
use std::path::Path;

fn main() {
    let file = env::args().skip(1).next().unwrap_or("file10.rs".to_string());
    let path = Path::new(&file);
    match path.metadata() {
        Ok(data) => {
            println!("type {:?}", data.file_type());
            println!("len {}", data.len());
            println!("perm {:?}", data.permissions());
            println!("modified {:?}", data.modified());
        },
        Err(e) => println!("error {:?}", e)
    }
}
// type FileType(FileType { mode: 33204 })
// len 488
// perm Permissions(FilePermissions { mode: 436 })
// modified Ok(SystemTime { tv_sec: 1483866529, tv_nsec: 600495644 })
```

The length of the file (in bytes) and modified time are straightforward to interpret.
(Note we may not be able to get this time!)  The file type has methods `is_dir`,
`is_file` and `is_symlink`.

`permissions` is an interesting one. Rust strives to be cross-platform, and so it's
a case of the 'lowest common denominator'. In general, all you can query is whether
the file is read-only - the 'permissions' concept is extended in Unix and encodes
read/write/executable for user/group/others.

But, if you are not interested in Windows, then bringing in a platform-specific trait will give
us at least the permission mode bits. (As usual, a trait only kicks in when it is
visible.) Then, applying the program to its own executable gives:

```rust
use std::os::unix::fs::PermissionsExt;
...
println!("perm {:o}",data.permissions().mode());
// perm 755
```
(Note '{:o}' for printing out in _octal_)

(Whether a file is executable on Windows is determined by its extension. The executable
extensions are found in the `PATHEXT` environment variable - '.exe','.bat' and so forth).

`std::fs` contains a number of useful functions for working with files, such as copying or
moving files, making symbolic links and creating directories.

To find the contents of a directory, `std::fs::read_dir` provides an iterator.
Here are all files with extension '.rs' and size greater than 1024 bytes:

```rust
fn dump_dir(dir: &str) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let data = entry.metadata()?;
        let path = entry.path();
        if data.is_file() {
            if let Some(ex) = path.extension() {
                if ex == "rs" && data.len() > 1024 {
                    println!("{} length {}", path.display(),data.len());
                }
            }
        }
    }
    Ok(())
}
// ./enum4.rs length 2401
// ./struct7.rs length 1151
// ./sexpr.rs length 7483
// ./struct6.rs length 1359
// ./new-sexpr.rs length 7719
```

Obviously `read_dir` might fail (usually 'not found' or 'no permission'), but
also getting each new entry might fail (it's like the `lines` iterator over a buffered
reader's contents).  Plus, we might not be able to get the metadata corresponding to
the entry.  A file might have no extension, so we have to check for that as well.

Why not just an iterator over paths? On Unix this is the way the `opendir` system call works,
but on Windows you cannot iterate over a directory's contents without getting the
metadata. So this is a reasonably elegant compromise that allows cross-platform
code to be as efficient as possible.

You can be forgiven for feeling 'error fatigue' at this point. But please note that
the _errors always existed_ - it's not that Rust is inventing new ones. It's just
trying its darnedest to make it hard for you to ignore them.  Any call to an
operating system may fail, but C makes getting errors awkward. You check the return
code, and if it's not zero you then have to inspect `errno`
to find out what the actual error was.
Quite apart from memory safety, this makes robust C programming harder than most
C programmers think!

Languages like Java and Python throw exceptions; languages like Go and Lua return two
values, where the first is the result and the second is the error: like Rust it is
considered bad manners for library functions to raise errors. So there is a lot
of error checking and early-returns from functions.

Rust uses `Result` because it's either-or: you cannot get a result _and_ an error.
And the question-mark operator makes handling errors much cleaner.

## Processes

A fundamental need is for programs to run programs, or to _launch processes_.
Your program can _spawn_ as many child processes it likes, and as the name
suggests they have a special relationship with their parent.

To run a program is straightforward using the `Command` struct, which _builds_ up
arguments to pass to the program:

```rust
use std::process::Command;

fn main() {
    let status = Command::new("rustc")
        .arg("-V")
        .status()
        .expect("no rustc?");

    println!("cool {} code {}", status.success(), status.code().unwrap());
}
// rustc 1.15.0-nightly (8f02c429a 2016-12-15)
// cool true code 0
```
So `new` receives the name of the program (it will be looked up on `PATH` if not
an absolute filename), `arg` adds a new argument, and `status` causes it to be run.
This returns a `Result`, which is `Ok` if the program actually run, containing an
`ExitStatus`. In this case, the program succeeded, and returned an exit code 0. (The
`unwrap` is because we can't always get the code if the program was killed by
a signal).

If we change the `-V` to `-v` (an easy mistake) then `rustc` fails:

```
error: no input filename given

cool false code 101
```

So there are three possibilities:

  - program didn't exist, was bad, or we were not allowed to run it
  - program ran, but was not successful - non-zero exit code
  - program ran, with zero exit code. Success!

By default, we get the program's output, since its standard output and standard error is
still going to the terminal.

Often we are very interested in capturing that output, so there's the `output`
method.

```rust
// process2.rs
use std::process::Command;

fn main() {
    let output = Command::new("rustc")
        .arg("-V")
        .output()
        .expect("no rustc?");

    if output.status.success() {
        println!("ok!");
    }
    println!("len stdout {} stderr {}", output.stdout.len(), output.stderr.len());
}
// ok!
// len stdout 44 stderr 0
```

As with `status` our program blocks until the child process is finished, and we get
back three things - the status (as before), the contents of stdout and the contents
of stderr.

The captured output is simply `Vec<u8>` - just bytes.  Recall we have no guarantee
that data we receive from the operating system is a properly encoded UTF-8 string. In
fact, we have no guarantee that it _even_ is a string - programs may return arbitrary
binary data.

If we are pretty sure the output is UTF-8, then `String::from_utf8` will convert those
vectors or bytes - it returns a `Result` because this conversion may not succeed.
A more sloppy function is `String::from_utf8_lossy` which will make a good attempt at
conversion and insert the invalid Unicode mark � where it failed.

Here is a useful function which runs a program using the shell. This uses the usual
shell mechanism for joining stderr to stdout:

```rust
fn shell(cmd: &str) -> (String,bool) {
    let cmd = format!("{} 2>&1",cmd);
    let output = Command::new("/bin/sh")
        .arg("-c")
        .arg(&cmd)
        .output()
        .expect("no shell?");
    (
        String::from_utf8_lossy(&output.stdout).trim_right().to_string(),
        output.status.success()
    )
}

fn shell_success(cmd: &str) -> Option<String> {
    let (output,success) = shell(cmd);
    if success {Some(output)} else {None}
}
```

I'm trimming any whitespace from the right so that if you said `shell("which rustc")`
you will get the path without any extra linefeed.

You can control the execution of a program by specifying the directory it will run
in using the `current_dir` method and the environment variables it sees using `env`.

Up to now, our program simply waits for the child process to finish. If you use
the `spawn` method then we return immediately, and must explicitly wait for it to
finish - or go off and do something else in the meantime!  This example also
shows how to suppress both standard out and standard error:

```rust
// process5.rs
use std::process::{Command,Stdio};

fn main() {
    let mut child = Command::new("rustc")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("no rustc?");

    let res = child.wait();
    println!("res {:?}", res);
}
```

By default, the child 'inherits' the standard input and output of the parent. In this case,
we redirect the child's output handles into 'nowhere'. It's equivalent to saying
`> /dev/null > /dev/null 2> /dev/null` in the Unix shell.

Now, it's possible to do these things using the shell (`sh` or `cmd`) in Rust, although not in a portable way.
But this way you get full programmatic control of process creation.

For example, if we just had `.stdout(Stdio::piped())` then the child's standard output
is redirected to a pipe. If we then use `wait_with_output` instead of `wait` then
it returns a `Result<Output>` and the child's output is captured into the `stdout`
field of that `Output` just as before. Alternatively, the _child_'s `stdout` field is readable and
you can manually read the child's output in the usual way.

The `Child` struct also gives you an explicit `kill` method.

