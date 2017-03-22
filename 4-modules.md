# Modules and Cargo

## Modules

As programs get larger, it's necessary to spread them over more than one file
and put functions and types in different _namespaces_.
The Rust solution for both of these is _modules_.

C does the first, and not
the second, so you end up with awful names like `primitive_display_set_width` and
so forth. In Rust the full name would look like `primitive::display::set_width`,
and after saying `use primitive::display` you can then refer to it as `display::set_width`.
You can even say `use primitive::display::set_width` and then just say `set_width`, but
it's not a good idea to get carried away with this. `rustc` will not be confused, but _you_
may get confused later.

A new keyword `mod` is used to define a module as a block
where Rust types or functions can be written:

```rust
mod foo {
    #[derive(Debug)]
    struct Foo {
        s: &'static str
    }
}

fn main() {
    let f = foo::Foo{s: "hello"};
    println!("{:?}", f);
}
```

But it's still not quite right - we get 'struct Foo is private'. To solve this, we need the `pub` keyword
to export `Foo`. The error then changes to 'field s of struct foo::Foo is private', so put `pub`
before the field `s` to export `Foo::s`. Then things will work.

```rust
    pub struct Foo {
        pub s: &'static str
    }
```
Needing an explicit `pub` means that you must _choose_ what items to make public from a module.
The set of functions and types exported from a module is called its _interface_.

It is usually better to hide the insides of a struct, and only allow access through methods:

```rust
mod foo {
    #[derive(Debug)]
    pub struct Foo {
        s: &'static str
    }

    impl Foo {
        pub fn new(s: &'static str) -> Foo {
            Foo{s: s}
        }
    }
}

fn main() {
    let f = foo::Foo::new("hello");
    println!("{:?}", f);
}
```

Why is hiding the implementation a good thing?  Because it means you may change it later
without breaking the interface, without consumers of a module getting too dependent on its details.
The great enemy of large-scale programing is a tendency for code to get entangled, so that understanding
a piece of code is impossible in isolation.

In a perfect world a module does one thing, does it well, and keeps its own secrets.

When not to hide? As Stroustrup says, when the interface _is_ the implementation, like
`struct Point{x: f32, y: f32}`.

_Within_ a module, all items are visible to all other items. It's a cozy place where
everyone can be friends and know intimate details about each other.

Everyone gets to a point where they want to break a program up into separate files,
depending on taste. I start getting uncomfortable around 500 lines, but we all agree
that more than 2000 lines is pushing it.

So how to break this program into separate files?

We put the `foo` code into `foo.rs`:

```rust
// foo.rs
#[derive(Debug)]
pub struct Foo {
    s: &'static str
}

impl Foo {
    pub fn new(s: &'static str) -> Foo {
        Foo{s: s}
    }
}
```
And use a `mod foo` statement _without_ a block in the main program:

```rust
// mod3.rs
mod foo;

fn main() {
    let f = foo::Foo::new("hello");
    println!("{:?}", f);
}
```
Now `rustc mod3.rs` will cause `foo.rs` to be compiled as well. There is no need to fool around
with makefiles!

The compiler will also look at `MODNAME/mod.rs`, so this will work
if I create a directory `boo` containing a file `mod.rs`:

```rust
// boo/mod.rs
pub fn answer()->u32 {
    42
}
```
And now the main program can use both modules as separate files:

```
// mod3.rs
mod foo;
mod boo;

fn main() {
    let f = foo::Foo::new("hello");
    let res = boo::answer();
    println!("{:?} {}", f,res);
}
```

So far, there's `mod3.rs`, containing `main`, a module `foo.rs` and a directory `boo` containing
`mod.rs`.  The usual convention is that the file containing `main` is just called `main.rs`.

Why two ways to do the same thing? Because `boo/mod.rs` can refer to other modules defined in `boo`,
Update `boo/mod.rs` and add a new module - note that this is explicitly exported.

```rust
pub fn answer()->u32 {
	42
}

pub mod bar {
    pub fn question() -> &'static str {
        "the meaning of everything"
    }
}
```
and then we have the question corresponding to the answer:

```rust
let q = boo::bar::question();
```

That module block can be pulled out as `boo/bar.rs` and so on.

In summary, modules are about organization and visibility,
and this may or may not involve separate files.

Please note that `use` has nothing to do with importing, and simply specifies visibility
of module names. For example:

```rust
{
    use boo::bar;
    let q = bar::question();
    ...
}
{
    use boo::bar::question();
    let q = question();
    ...
}

```
An important point to note is there is no _separate compilation_ here. The main program and its
module files will be recompiled each time. Larger programs will take a fair amount of time to build,
although `rustc` is getting better at incremental compilation.

## Crates

The 'compilation unit' for Rust is the _crate_, which is either an executable or a library.

To separately compile the files from the last section,
first build `foo.rs` as a Rust _static library_ crate:

```
src$ rustc foo.rs --crate-type=lib
src$ ls -l libfoo.rlib
-rw-rw-r-- 1 steve steve 7888 Jan  5 13:35 libfoo.rlib
```
We can now _link_ this into our main program:

```
src$ rustc mod4.rs --extern foo=libfoo.rlib
```
But the main program must now look like this, where the `extern` name is the same
as the one used when linking. There is an implicit top-level module `foo` associated
with the library crate:

```
// mod4.rs
extern crate foo;

fn main() {
    let f = foo::Foo::new("hello");
    println!("{:?}", f);
}
```
Before people start chanting 'Cargo! Cargo!' let me justify this lower-level look at building Rust.
I'm a great believer in 'Know Thy Toolchain', and this will reduce the amount of new magic you need
to learn when we look at managing projects with Cargo. Modules are basic language features and can be
used outside Cargo projects.

It's time to understand why Rust binaries are so large:

```
src$ ls -lh mod4
-rwxrwxr-x 1 steve steve 3,4M Jan  5 13:39 mod4
```
That's rather fat! There is a _lot_ of debug information in that executable. This is not a Bad Thing,
if you want to use a debugger and actually want meaningful backtraces when your program panics.

So let's strip that debug information and see:

```
src$ strip mod4
src$ ls -lh mod4
-rwxrwxr-x 1 steve steve 300K Jan  5 13:49 mod4
```
Still feels a little large for something so simple, but this program links _statically_ against
the Rust standard library. This is a Good Thing, since you can hand this executable to anyone
with the right operating system - they will not need a 'Rust runtime'. (And `rustup` will even let
you cross-compile for other operating systems and platforms as well.)

We can link dynamically against the Rust runtime and get truly tiny exes:

```
src$ rustc -C prefer-dynamic mod4.rs --extern foo=libfoo.rlib
src$ ls -lh mod4
-rwxrwxr-x 1 steve steve 14K Jan  5 13:53 mod4
src$ ldd mod4
	linux-vdso.so.1 =>  (0x00007fffa8746000)
	libstd-b4054fae3db32020.so => not found
	libc.so.6 => /lib/x86_64-linux-gnu/libc.so.6 (0x00007f3cd47aa000)
	/lib64/ld-linux-x86-64.so.2 (0x00007f3cd4d72000)
```
That 'not found' is because `rustup` doesn't install the dynamic libraries globally. We
can hack our way to happiness, at least on Unix (yes, I know the best solution is a symlink.)

```
src$ export LD_LIBRARY_PATH=~/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib
src$ ./mod4
Foo { s: "hello" }
```

Rust does not have a _philosophical_ problem with dynamic linking, in the same way as Go does. It's
just that when there's a stable release every 6 weeks it becomes inconvenient to have to recompile
everything. If you have a stable version that Works For You, then cool. As stable versions of Rust
get increasingly delivered by the OS package manager, dynamic linking will become more popular.

## Cargo

The Rust standard library is not very large, compared to Java or Python; although much more fully
featured than C or C++, which lean heavily on operating system provided libraries.

But it is straightforward to access community-provided libraries in [crates.io](https://crates.io)
using __Cargo__.  Cargo will look up the correct version and download the source for you, and
ensures that any other needed crates are downloaded as well.

Let's create a simple program which needs to read JSON. This data format is very widely used,
but is too specialized for inclusion in the standard library. So we initialize a Cargo project,
using '--bin' because the default is to create a library project.

```
test$ cargo init --bin test-json
     Created binary (application) project
test$ cd test-json
test$ cat Cargo.toml
[package]
name = "test-json"
version = "0.1.0"
authors = ["Your Name <you@example.org>"]

[dependencies]
```
To make the project depend on the [JSON crate](http://json.rs/doc/json/), edit the
'Cargo.toml' file so:

```
[dependencies]
json="0.11.4"
```

Then do a first build with Cargo:

```
test-json$ cargo build
    Updating registry `https://github.com/rust-lang/crates.io-index`
 Downloading json v0.11.4
   Compiling json v0.11.4
   Compiling test-json v0.1.0 (file:///home/steve/c/rust/test/test-json)
    Finished debug [unoptimized + debuginfo] target(s) in 1.75 secs
```
The main file of this project has already been created - it's 'main.rs' in the 'src'
directory. It starts out just as a 'hello world' app, so let's edit it to be a proper test program.

Mote the very convenient 'raw' string literal - otherwise we would need to escape those double quotes
and end up with ugliness:

```rust
// test-json/src/main.rs
extern crate json;

fn main() {
    let doc = json::parse(r#"
    {
        "code": 200,
        "success": true,
        "payload": {
            "features": [
                "awesome",
                "easyAPI",
                "lowLearningCurve"
            ]
        }
    }
    "#).expect("parse failed");

    println!("debug {:?}", doc);
    println!("display {}", doc);
}
```

You can now build and run this project - only `main.rs` has changed.

```
test-json$ cargo run
   Compiling test-json v0.1.0 (file:///home/steve/c/rust/test/test-json)
    Finished debug [unoptimized + debuginfo] target(s) in 0.21 secs
     Running `target/debug/test-json`
debug Object(Object { store: [("code", Number(Number { category: 1, exponent: 0, mantissa: 200 }),
 0, 1), ("success", Boolean(true), 0, 2), ("payload", Object(Object { store: [("features",
 Array([Short("awesome"), Short("easyAPI"), Short("lowLearningCurve")]), 0, 0)] }), 0, 0)] })
display {"code":200,"success":true,"payload":{"features":["awesome","easyAPI","lowLearningCurve"]}}
```
The debug output shows some internal details of the JSON document, but a
plain '{}', using the `Display` trait, regenerates JSON from the parsed document.

Let's explore the JSON API.
It would not be useful if we could not extract values. The `as_TYPE` methods
return `Option<TYPE>` since we cannot be sure that the field exists or is of the correct type.
(see the [docs for JsonValue](http://json.rs/doc/json/enum.JsonValue.html))

```rust
    let code = doc["code"].as_u32().unwrap_or(0);
    let success = doc["success"].as_bool().unwrap_or(false);

    assert_eq!(code, 200);
    assert_eq!(success, true);

    let features = &doc["payload"]["features"];
    for v in features.members() {
        println!("{}", v.as_str().unwrap()); // MIGHT explode
    }
    // awesome
    // easyAPI
    // lowLearningCurve
```
`features` here is a reference to `JsonValue` - it has to be a reference because otherwise
we would be trying to move a _value_ out of the JSON document.  Here we know it's an array,
so `members()` will return a non-empty iterator over `&JsonValue`.

What if the 'payload' object didn't have a 'features' key? Then `features` would be set to `Null`.
There will be no explosion. This convenience expresses the free-form, anything-goes nature of JSON
very well. It is up to you to examine the structure of any document you receive and create your own
errors if the structure does not match.

You can modify these structures. If we had `let mut doc` then this would do what you expect:

```rust
    let features = &mut doc["payload"]["features"];
    features.push("cargo!").expect("couldn't push");
```
The `push` will fail if `features` wasn't an array, hence it returns `Result<()>`.

Here's a truly beautiful use of macros to generate _JSON literals_:

```rust
    let data = object!{
        "name"    => "John Doe",
        "age"     => 30,
        "numbers" => array![10,53,553]
    };
    assert_eq!(
        data.dump(),
        r#"{"name":"John Doe","age":30,"numbers":[10,53,553]}"#
    );
```

For this to work, you need to explicitly import macros from the JSON crate thus:

```rust
#[macro_use]
extern crate json;
```

There is a downside to using this crate, because of the mismatch between the amorphous, dynamically-typed
nature of JSON and the structured, static nature of Rust. (The readme explicitly speaks of 'friction')
So if you _did_ want to map JSON to Rust data structures, you would end up doing a lot of checking,
because you can not assume that the received structure matches your structs! For that, a better
solution is [serde-json](https://github.com/serde-rs/json) where you _serialize_ Rust data structures
into JSON and _deserialize_ JSON into Rust.

For this, create a another Cargo binary project with `cargo new --bin test-serde-json`, go into
the `test-serde-json` directory and edit `Cargo.toml`. Edit it like so:

```
[dependencies]
serde="0.9"
serde_derive="0.9"
serde_json="0.9"
```

And edit `src/main.rs` to be this:

```rust
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u8,
    address: Address,
    phones: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Address {
    street: String,
    city: String,
}

fn main() {
    let data = r#" {
     "name": "John Doe", "age": 43,
     "address": {"street": "main", "city":"Downtown"},
     "phones":["27726550023"]
    } "#;
    let p: Person = serde_json::from_str(data).expect("deserialize error");
    println!("Please call {} at the number {}", p.name, p.phones[0]);

    println!("{:#?}",p);
}
```

You have seen the `derive` attribute before, but the `serde_derive` crate defines _custom derives_
for the special `Serialize` and `Deserialize` traits. And the result shows the generated Rust struct:

```
Please call John Doe at the number 27726550023
Person {
    name: "John Doe",
    age: 43,
    address: Address {
        street: "main",
        city: "Downtown"
    },
    phones: [
        "27726550023"
    ]
}
```

Now, if you did this using the `json` crate, you would need a few hundred lines of custom conversion
code, mostly error handling. Tedious, easy to mess up, and not where you want to put effort anyway.

This is clearly the best solution if you are processing well-structured JSON from outside sources (it's
possible to remap field names if needed) and provides a robust way for Rust programs to share data
with other programs over the network (since everything understands JSON these days.)

Serialization is an important technique and similar solutions exist for Java and Go - but with a big
difference. In those languages the structure of the data is found at _run-time_ using _reflection_, but
in this case the serialization code is generated at _compile-time_ - altogether more efficient!

Cargo is considered to be one of the great strengths of the Rust ecosystem, because it does
a lot of work for us. Otherwise we would have had to download these libraries from Github,
build as static library crates, and link them against the program. It's painful to do this for
C++ projects, and would be nearly as painful for Rust projects if Cargo did not exist.
C++ is somewhat unique in its painfullness here, so we should compare this with
other languages' package managers. npm (for JavaScript) and pip (for Python) manage dependencies
and downloads for you, but the distribution story is harder, since the user of your program
needs NodeJS or Python installed.
But these programs are statically linked against their dependencies, so again it can be handed
out to your buddies without external dependencies.

## More Gems

When processing anything except simple text, regular expressions make your life much easier.
These are commonly available for most languages and I'll here assume a basic familiarity with
regex notation.  To use the [regex](https://github.com/rust-lang/regex) crate, put 'regex = "0.2.1"'
after "[dependencies]" in your Cargo.toml.

We'll use 'raw strings' again so that the backslashes don't have to be escaped. In English, this
regular expression is "match exactly two digits, the character ':', and then any number of digits.
Capture both sets of digits":

```rust
extern crate regex;
use regex::Regex;

let re = Regex::new(r"(\d{2}):(\d+)").unwrap();
println!("{:?}", re.captures("  10:230"));
println!("{:?}", re.captures("[22:2]"));
println!("{:?}", re.captures("10:x23"));
// Some(Captures({0: Some("10:230"), 1: Some("10"), 2: Some("230")}))
// Some(Captures({0: Some("22:2"), 1: Some("22"), 2: Some("2")}))
// None
```

The successful output actually has three _captures_ - the whole match, and the two sets of digits.
These regular expressions are not _anchored_ by default, so __regex__ will hunt for the first
occurring match, skipping anything that doesn't match.  (If you left out the '()' it would just
give us the whole match.)

It's possible to _name_ those captures, and spread the regular expression over several lines,
even including comments!  Compiling the regex might fail (the first _expect_) or the match
might fail (the second _expect_). Here we can use the result as an associative array and look
up captures by name.

```rust
let re = Regex::new(r"(?x)
(?P<year>\d{4})  # the year
-
(?P<month>\d{2}) # the month
-
(?P<day>\d{2})   # the day
").expect("bad regex");
let caps = re.captures("2010-03-14").expect("match failed");

assert_eq!("2010", &caps["year"]);
assert_eq!("03", &caps["month"]);
assert_eq!("14", &caps["day"]);
```

Regular expressions can break up strings that match a pattern, but won't check whether they make sense.
That is, you can specify and match the _syntax_ of ISO-style dates, but _semantically_ they may be nonsense,
like "2014-24-52".

For this, you need dedicated date-time processing, which is provided by [chrono](https://github.com/lifthrasiir/rust-chrono).
You need to decide on a time zone when doing dates:

```rust
extern crate chrono;
use chrono::*;

fn main() {
    let date = Local.ymd(2010,3,14);
    println!("date was {}", date);
}
// date was 2010-03-14+02:00
```

However, this isn't recommended because feeding it bad dates will cause a panic! (try the bogus date
to see this.) The method you need here is `ymd_opt` which returns `LocalResult<Date>`

```rust
    let date = Local.ymd_opt(2010,3,14);
    println!("date was {:?}", date);
    // date was Single(2010-03-14+02:00)

    let date = Local.ymd_opt(2014,24,52);
    println!("date was {:?}", date);
    // date was None
```

You can also directly parse date-times, either in standard UTC format or using custom [formats](https://lifthrasiir.github.io/rust-chrono/chrono/format/strftime/index.html#specifiers).
These self-same formats allow you to print out dates in exactly the format you want.

I specifically highlighted these two useful crates because they would be part of the standard
library in most other languages. And, in fact, the embryonic form of these crates was
once part of the Rust stdlib, but were cut loose. This was a deliberate decision: the Rust team takes
stdlib stability very seriously so features only arrive in stable once they have gone through
incubation in unstable nightly versions, and only then beta and stable.  For libraries that need
experimentation and refinement, it's much better that they remain independent and get tracked
with Cargo. For all practical purposes, these two crates _are_ standard - they are not going away and
may be folded back into the stdlib at some point.

