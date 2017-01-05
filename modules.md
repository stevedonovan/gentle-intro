## Modules

As programs get larger, it's necessary to spread them over more than one file
and put functions and types in different _namespaces_.

C does the first, and not
the second, so you end up with awful names like `primitive_display_set_width` and
so forth. In Rust the full name would look like `primitive::display::set_width`,
and after saying `use primitive::display` you can then refer to it as `display::set_width`.
You can even say `use primitive::display::set_width` and then just say `set_width`, but
it's not a good idea to get carried away with this. `rustc` may not be confused, but _you_ 
may get confused later.

The Rust solution is _modules_.

A new keyword `mod` is used to define a module as a block 
where Rust types or functions can be written

```rust
mod foo {
    #[derive(Debug)]
    struct Foo {
        s: &'static str
    }  
}

fn main() {
    let f = foo::Foo{s: "hello"};
    println!("{:?}",f);    
}
```

But not quite right - we get `struct Foo is private`. To solve this, we need the `pub` keyword
to export `Foo`. The error then changes to `field s of struct foo::Foo is private', so `pub`
again to export `Foo::s`.

```rust
    pub struct Foo {
        pub s: &'static str
    }
```
Needing an explicit `pub` means that you must _choose_ what items to make public from a module,
often called its _interface_.

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
    println!("{:?}",f);    
}
```

Why is hiding the implementation a good thing?  Because it means you may change it later 
without breaking the interface, without consumers of a module getting too dependent on its details.
The great enemy of large programs is a tendency for code to get too entangled, so that understanding
a piece of code is impossible in isolation.

When not to hide? As Stroustrup says, when the interface _is_ the implementation, like
`struct Point{x: f32, y: f32}`.

_Within a module_, all items are visible to all other items. It's a cozy place where
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
    println!("{:?}",f);    
}
```
`rustc` does this automatically - `rustc mod3.rs` will cause `foo.rs` to be compiled as well.

The compiler will also look at `MODNAME/mod.rs`, so this will work
if I create a directory `boo` containing a file `mod.rs`:

```rust
// boo/mod.rs
pub fn answer()->u32 {
    42
}

// mod3.rs
mod foo;
mod boo;

fn main() {
    let f = foo::Foo::new("hello");
    let res = boo::answer();
    println!("{:?} {}",f,res);    
}
```

Let's keep going. Update `boo/mod.rs` - note this module is explicitly exported!

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

Now that module block can be pulled out as `boo/bar.rs` and so on.

In summary, modules are about organization and visibility,
and this may or may not involve separate files.

Please note that `use` has nothing to do with importing, and simply specifies visibility
of module names. For example:

```rust
use boo::bar;
...
let q = bar::question();

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
    println!("{:?}",f);    
}
```
Before people start chanting 'Cargo! Cargo!' let me justify this lower-level look at building Rust.
I'm a great believer in 'Know Thy Toolchain', and this will reduce the amount of new magic you need
to learn when we look at managing projects with Cargo.

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
with the right operating system - they will not need a 'Rust install`. (And `rustup` will even let
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
can hack our way to happiness, at least on Linux (yes, I know the best solution is a symlink.)

```
src$ export LD_LIBRARY_PATH=~/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib
src$ ./mod4
Foo { s: "hello" }
```

Rust does not have a _philosophical_ problem with dynamic linking, in the same way as Go does. It's
just that when there's a stable release every 6 weeks it becomes inconvenient to have to recompile
everything. If you have a stable version that Works For You, then cool. As stable versions of Rust
get increasingly delivered by the OS package manager, dynamic linking will become more popular.




