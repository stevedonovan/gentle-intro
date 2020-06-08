# Structs, Enums and Matching

## Rust likes to Move It, Move It

I'd like to move back a little, and show you something surprising:

```rust
// move1.rs
fn main() {
    let s1 = "hello dolly".to_string();
    let s2 = s1;
    println!("s1 {}", s1);
}
```
And we get the following error:

```
error[E0382]: use of moved value: `s1`
 --> move1.rs:5:22
  |
4 |     let s2 = s1;
  |         -- value moved here
5 |     println!("s1 {}", s1);
  |                      ^^ value used here after move
  |
  = note: move occurs because `s1` has type `std::string::String`,
  which does not implement the `Copy` trait
```
Rust has different behaviour than other languages. In a language where variables are
always references (like Java or Python), `s2` becomes yet another reference to the
string object referenced by `s1`. In C++, `s1` is a value, and it is _copied_ to `s2`.
But Rust moves the value.  It doesn't see strings as copyable
("does not implement the Copy trait").

We would not see this with 'primitive' types like numbers, since they are just values;
they are allowed to be copyable because they are cheap to copy. But `String` has allocated
memory containing "Hello dolly", and copying will involve allocating some more memory
and copying the characters. Rust will not do this silently.

Consider a `String` containing the whole text of 'Moby-Dick'. It's not a big struct,
just has the address in memory of the text, its size, and how big the allocated block is.
Copying this is going to be expensive, because that memory is allocated on the heap and
the copy will need its own allocated block.

```
    String
    | addr | ---------> Call me Ishmael.....
    | size |                    |
    | cap  |                    |
                                |
    &str                        |
    | addr | -------------------|
    | size |

    f64
    | 8 bytes |
```
The second value is a string slice (`&str`) which refers to the same memory as the string,
with a size - just the guy's name. Cheap to copy!

The third value is an `f64` - just 8 bytes. It does not refer to any other memory, so
it's just as cheap to copy as to move.

`Copy` values are only defined by their representation in memory, and when
Rust copies, it just copies those bytes elsewhere. Similarly, a non-`Copy` value
is also _just moved_.  There is no cleverness in copying and moving, unlike in C++.

Re-writing with a function call reveals exactly the same error:

```rust
// move2.rs

fn dump(s: String) {
    println!("{}", s);
}

fn main() {
    let s1 = "hello dolly".to_string();
    dump(s1);
    println!("s1 {}", s1); // <---error: 'value used here after move'
}
```
Here, you have a choice. You may pass a reference to that string, or
explicitly copy it using its `clone` method.  Generally, the first is the better way
to go.

```rust
fn dump(s: &String) {
    println!("{}", s);
}

fn main() {
    let s1 = "hello dolly".to_string();
    dump(&s1);
    println!("s1 {}", s1);
}
```
The error goes away. But you'll rarely see a plain
`String` reference like this, since to pass a string literal is really ugly _and_ involves
creating a temporary string.

```rust
    dump(&"hello world".to_string());
```
So altogether the best way to declare that function is:

```rust
fn dump(s: &str) {
    println!("{}", s);
}
```

And then both `dump(&s1)` and `dump("hello world")` work properly. (Here `Deref`
coercion kicks in and Rust will convert `&String` to `&str` for you.)

To summarise, assignment of a non-Copy value moves the value from one location
to another. Otherwise, Rust would be forced to _implicitly_ do a copy and break its
promise to make allocations explicit.

## Scope of Variables

So, the rule of thumb is to prefer to keep references to the original data - to 'borrow'
it.

But a reference must _not_ outlive the owner!

First, Rust is a _block-scoped_ language. Variables only exist for the duration of their
block:

```rust
{
    let a = 10;
    let b = "hello";
    {
        let c = "hello".to_string();
        // a, b and c are visible
    }
    // the string c is dropped
    // a, b are visible
    for i in 0..a {
        let b = &b[1..];
        // original b is no longer visible - it is shadowed.
    }
    // the slice b is dropped, original b is visible again
    // i is _not_ visible!
}
```
Loop variables (like `i`) are a little different, they are only visible in the loop
block.  It is not an error to create a new variable using the same name ('shadowing')
but it can be confusing.

When a variable 'goes out of scope' then it is _dropped_. Any memory used is reclaimed,
and any other _resources_ owned by that variable are given back to the system - for
instance, dropping a `File` closes it.  This is a Good Thing. Unused resources are
reclaimed immediately when not needed.

(A further Rust-specific issue is that a variable may appear to be in scope, but its
value has moved.)

Here a reference `rs1` is made to a value `tmp` which only lives for the duration
of its block:

```rust
// ref1.rs
fn main() {
    let s1 = "hello dolly".to_string();
    let mut rs1 = &s1;
    {
        let tmp = "hello world".to_string();
        rs1 = &tmp;
    }
    println!("ref {}", rs1);
}
```
We borrow the value of `s1` and then borrow the value of `tmp`. But `tmp`'s value
does not exist outside that block!

```
error: `tmp` does not live long enough
  --> ref1.rs:8:5
   |
7  |         rs1 = &tmp;
   |                --- borrow occurs here
8  |     }
   |     ^ `tmp` dropped here while still borrowed
9  |     println!("ref {}", rs1);
10 | }
   | - borrowed value needs to live until here
```
Where is `tmp`? Gone, dead, gone back to the Big Heap in the Sky: _dropped_.
Rust is here saving you from the dreaded 'dangling pointer' problem of C -
a reference that points to stale data.

## Tuples

It's sometimes very useful to return multiple values from a function. Tuples are
a convenient solution:

```rust
// tuple1.rs

fn add_mul(x: f64, y: f64) -> (f64,f64) {
    (x + y, x * y)
}

fn main() {
    let t = add_mul(2.0,10.0);

    // can debug print
    println!("t {:?}", t);

    // can 'index' the values
    println!("add {} mul {}", t.0,t.1);

    // can _extract_ values
    let (add,mul) = t;
    println!("add {} mul {}", add,mul);
}
// t (12, 20)
// add 12 mul 20
// add 12 mul 20
```
Tuples may contain _different_ types, which is the main difference from arrays.

```rust
let tuple = ("hello", 5, 'c');

assert_eq!(tuple.0, "hello");
assert_eq!(tuple.1, 5);
assert_eq!(tuple.2, 'c');
```
They appear in some `Iterator` methods. `enumerate` is like the Python generator
of the same name:

```rust
    for t in ["zero","one","two"].iter().enumerate() {
        print!(" {} {};",t.0,t.1);
    }
    //  0 zero; 1 one; 2 two;
```
`zip` combines two iterators into a single iterator of
tuples containing the values from both:

```rust
    let names = ["ten","hundred","thousand"];
    let nums = [10,100,1000];
    for p in names.iter().zip(nums.iter()) {
        print!(" {} {};", p.0,p.1);
    }
    //  ten 10; hundred 100; thousand 1000;
```

## Structs

Tuples are convenient, but saying `t.1` and keeping track of the meaning of each part
is tedious for anything that isn't straightforward.

Rust _structs_ contain named _fields_:

```rust
// struct1.rs

struct Person {
    first_name: String,
    last_name: String
}

fn main() {
    let p = Person {
        first_name: "John".to_string(),
        last_name: "Smith".to_string()
    };
    println!("person {} {}", p.first_name,p.last_name);
}
```

The values of a struct will be placed next to each other in memory, although you should
not assume any particular memory layout, since the compiler will organize the memory for
efficiency, not size, and there may be padding.

Initializing this struct is a bit clumsy, so we want to move the construction of a `Person`
into its own function. This function can be made into an _associated function_ of `Person` by putting
it into a `impl` block:

```rust
// struct2.rs

struct Person {
    first_name: String,
    last_name: String
}

impl Person {

    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

}

fn main() {
    let p = Person::new("John","Smith");
    println!("person {} {}", p.first_name,p.last_name);
}
```
There is nothing magic or reserved about the name `new` here. Note that it's accessed
using a C++-like notation using double-colon `::`.

Here's a `Person` _method_ , that takes a _reference self_ argument:

```rust
impl Person {
    ...

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

}
...
    println!("fullname {}", p.full_name());
// fullname John Smith
```
The `self` is used explicitly and is passed as a reference.
(You can think of `&self` as short for `self: &Person`.)

The keyword `Self` refers to the struct type - you can mentally substitute `Person`
for `Self` here:

```rust
    fn copy(&self) -> Self {
        Self::new(&self.first_name,&self.last_name)
    }
```

Methods may allow the data to be modified using a _mutable self_ argument:

```rust
    fn set_first_name(&mut self, name: &str) {
        self.first_name = name.to_string();
    }
```
And the data will _move_ into the method when a plain self argument is used:

```rust
    fn to_tuple(self) -> (String,String) {
        (self.first_name, self.last_name)
    }
```
(Try that with `&self` - structs will not let go of their data without a fight!)

Note that after `v.to_tuple()` is called, then `v` has moved and is no longer
available.

To summarize:
  -  no `self` argument: you can associate functions with structs, like the `new` "constructor".
  - `&self` argument: can use the values of the struct, but not change them
  - `&mut self` argument: can modify the values
  - `self` argument: will consume the value, which will move.

If you try to do a debug dump of a `Person`, you will get an informative error:

```
error[E0277]: the trait bound `Person: std::fmt::Debug` is not satisfied
  --> struct2.rs:23:21
   |
23 |     println!("{:?}", p);
   |                     ^ the trait `std::fmt::Debug` is not implemented for `Person`
   |
   = note: `Person` cannot be formatted using `:?`; if it is defined in your crate,
    add `#[derive(Debug)]` or manually implement it
   = note: required by `std::fmt::Debug::fmt`
```
The compiler is giving advice, so we put `#[derive(Debug)]` in front of `Person`, and now
there is sensible output:

```
Person { first_name: "John", last_name: "Smith" }
```

The _directive_ makes the compiler generate a `Debug` implementation, which is very
helpful. It's good practice to do this for your structs, so they can be
printed out (or written as a string using `format!`).  (Doing so _by default_ would be
very un-Rustlike.)

Here is the final little program:

```rust
// struct4.rs
use std::fmt;

#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String
}

impl Person {

    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}",self.first_name, self.last_name)
    }

    fn set_first_name(&mut self, name: &str) {
        self.first_name = name.to_string();
    }

    fn to_tuple(self) -> (String,String) {
        (self.first_name, self.last_name)
    }
}

fn main() {
    let mut p = Person::new("John","Smith");

    println!("{:?}", p);

    p.set_first_name("Jane");

    println!("{:?}", p);

    println!("{:?}", p.to_tuple());
    // p has now moved.

}
// Person { first_name: "John", last_name: "Smith" }
// Person { first_name: "Jane", last_name: "Smith" }
// ("Jane", "Smith")
```

## Lifetimes Start to Bite

Usually structs contain values, but often they also need to contain references.
Say we want to put a string slice, not a string value, in a struct.

```rust
// life1.rs

#[derive(Debug)]
struct A {
    s: &str
}

fn main() {
    let a = A { s: "hello dammit" };

    println!("{:?}", a);
}
```

```
error[E0106]: missing lifetime specifier
 --> life1.rs:5:8
  |
5 |     s: &str
  |        ^ expected lifetime parameter
```
To understand the complaint, you have to see the problem from the point of view of Rust.
It will not allow a reference to be stored without knowing its lifetime. All
references are borrowed from some value, and all values have lifetimes. The lifetime of
a reference cannot be longer than the lifetime of that value.
Rust cannot allow
a situation where that reference could suddenly become invalid.

Now, string slices borrow from _string literals_
like "hello" or from `String` values. String literals exist for the duration
of the whole program, which is called the 'static' lifetime.

So this works - we assure Rust that the string slice always refers to such static strings:

```rust
// life2.rs

#[derive(Debug)]
struct A {
    s: &'static str
}

fn main() {
    let a = A { s: "hello dammit" };

    println!("{:?}", a);
}
// A { s: "hello dammit" }
```
It is not the most _pretty_ notation, but sometimes ugliness is the necessary
price of being precise.

This can also be used to specify a string slice that is returned from a function:

```rust
fn how(i: u32) -> &'static str {
    match i {
    0 => "none",
    1 => "one",
    _ => "many"
    }
}
```
That works for the special case of static strings, but this is very restrictive.

However we can specify that the lifetime of the reference is _at least as long_ as that of
the struct itself.

```rust
// life3.rs

#[derive(Debug)]
struct A <'a> {
    s: &'a str
}

fn main() {
    let s = "I'm a little string".to_string();
    let a = A { s: &s };

    println!("{:?}", a);
}
```
Lifetimes are conventionally called 'a', 'b', etc but you could just as well call it
'me' here.

After this point, our `a` struct and the `s` string are bound by a strict contract:
`a` borrows from `s`, and cannot outlive it.

With this struct definition, we would like to write a function that returns an `A` value:

```rust
fn makes_a() -> A {
    let string = "I'm a little string".to_string();
    A { s: &string }
}
```

But `A` needs a lifetime - "expected lifetime parameter":

```
  = help: this function's return type contains a borrowed value,
   but there is no value for it to be borrowed from
  = help: consider giving it a 'static lifetime
```
`rustc` is giving advice, so we follow it:

```rust
fn makes_a() -> A<'static> {
    let string = "I'm a little string".to_string();
    A { s: &string }
}
```
And now the error is

```
8 |      A { s: &string }
  |              ^^^^^^ does not live long enough
9 | }
  | - borrowed value only lives until here
```

There is no way that this could safely work, because `string` will be dropped when the
function ends, and no reference to `string` can outlast it.

You can usefully think of lifetime parameters as being part of the type of a value.

Sometimes it seems like a good idea for a struct to contain a value _and_ a reference
that borrows from that value.
It's basically impossible because structs must be _moveable_, and any move will
invalidate the reference.  It isn't necessary to do this - for instance, if your
struct has a string field, and needs to provide slices, then it could keep indices
and have a method to generate the actual slices.

## Traits

Please note that Rust does not spell `struct` _class_. The keyword `class` in other
languages is so overloaded with meaning that it effectively shuts down original thinking.

Let's put it like this: Rust structs cannot _inherit_ from other structs; they are
all unique types. There is no _sub-typing_. They are dumb data.

So how _does_ one establish relationships between types? This is where _traits_ come in.

`rustc` often talks about `implementing X trait` and so it's time to talk about traits
properly.

Here's a little example of defining a trait and _implementing_ it for a particular type.

```rust
// trait1.rs

trait Show {
    fn show(&self) -> String;
}

impl Show for i32 {
    fn show(&self) -> String {
        format!("four-byte signed {}", self)
    }
}

impl Show for f64 {
    fn show(&self) -> String {
        format!("eight-byte float {}", self)
    }
}

fn main() {
    let answer = 42;
    let maybe_pi = 3.14;
    let s1 = answer.show();
    let s2 = maybe_pi.show();
    println!("show {}", s1);
    println!("show {}", s2);
}
// show four-byte signed 42
// show eight-byte float 3.14
```
It's pretty cool; we have _added a new method_ to both `i32` and `f64`!

Getting comfortable with Rust involves learning the basic traits of the
standard library (they tend to hunt in packs.)

`Debug` is very common.
We gave `Person` a default implementation with the
convenient `#[derive(Debug)]`, but say we want a `Person` to display as its full name:

```rust
use std::fmt;

impl fmt::Debug for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.full_name())
    }
}
...
    println!("{:?}", p);
    // John Smith
```
`write!` is a very useful macro - here `f` is anything that implements `Write`.
(This would also work with a `File` - or even a `String`.)

`Display` controls how values are printed out with "{}" and is implemented
just like `Debug`. As a useful side-effect, `ToString` is automatically
implemented for anything implementing `Display`. So if we implement
`Display` for `Person`, then `p.to_string()` also works.

`Clone` defines the method `clone`, and can simply be defined with
"#[derive(Clone)]" if all the fields themselves implement `Clone`.

## Example: iterator over floating-point range

We have met ranges before (`0..n`) but they don't work for floating-point values. (You
can _force_ this but you'll end up with a step of 1.0 which is uninteresting.)

Recall the informal definition of an iterator; it is an struct with a `next` method
which may return `Some`-thing or `None`. In the process, the iterator itself gets modified,
it keeps the state for the iteration (like next index and so forth.) The data that
is being iterated over doesn't change usually, (But see `Vec::drain` for an
interesting iterator that does modify its data.)

And here is the formal definition: the [Iterator trait](https://doc.rust-lang.org/std/iter/trait.Iterator.html).

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
    ...
}
```
Here we meet an [associated type](https://doc.rust-lang.org/stable/book/associated-types.html) of the `Iterator` trait.
This trait must work for any type, so you must specify that return type somehow.
The method `next` can then be written without using a
particular type - instead it refers to that type parameter's `Item` via `Self`.

The iterator trait for `f64` is written `Iterator<Item=f64>`, which can be read as
"an Iterator with its associated type Item set to f64".

The `...` refers to the _provided methods_ of `Iterator`. You only need to define `Item`
and `next`, and the provided methods are defined for you.

```rust
// trait3.rs

struct FRange {
    val: f64,
    end: f64,
    incr: f64
}

fn range(x1: f64, x2: f64, skip: f64) -> FRange {
    FRange {val: x1, end: x2, incr: skip}
}

impl Iterator for FRange {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.val;
        if res >= self.end {
            None
        } else {
            self.val += self.incr;
            Some(res)
        }
    }
}


fn main() {
    for x in range(0.0, 1.0, 0.1) {
        println!("{} ", x);
    }
}
```
And the rather messy looking result is

```
0
0.1
0.2
0.30000000000000004
0.4
0.5
0.6
0.7
0.7999999999999999
0.8999999999999999
0.9999999999999999
```
This is because 0.1 is not precisely representable as a float, so a little formatting
help is needed. Replace the `println!` with this

```rust
println!("{:.1} ", x);
```
And we get cleaner output (this [format](https://doc.rust-lang.org/std/fmt/index.html)
 means 'one decimal after dot'.)

All of the default iterator methods are available, so we can collect these values into
a vector, map them, and so forth.

```rust
    let v: Vec<f64> = range(0.0, 1.0, 0.1).map(|x| x.sin()).collect();
```

## Generic Functions

We want a function which will dump out any value that implements `Debug`. Here is
a first attempt at a generic function, where we can pass a reference to _any_ type
of value. `T` is a type parameter, which needs to be declared just after the
function name:

```rust
fn dump<T> (value: &T) {
    println!("value is {:?}",value);
}

let n = 42;
dump(&n);
```
However, Rust clearly knows nothing about this generic type `T`:

```
error[E0277]: the trait bound `T: std::fmt::Debug` is not satisfied
...
   = help: the trait `std::fmt::Debug` is not implemented for `T`
   = help: consider adding a `where T: std::fmt::Debug` bound
```
For this to work, Rust needs to be told that `T` does in fact implement `Debug`!

```rust
fn dump<T> (value: &T)
where T: std::fmt::Debug {
    println!("value is {:?}",value);
}

let n = 42;
dump(&n);
// value is 42
```
Rust generic functions need _trait bounds_ on types - we are saying here that
"T is any type that implements Debug". `rustc` is being very helpful, and
suggests exactly what bound needs to be provided.

Now that Rust knows the trait bounds for `T`, it can give you sensible compiler messages:

```rust
struct Foo {
    name: String
}

let foo = Foo{name: "hello".to_string()};

dump(&foo)
```
And the error is "the trait `std::fmt::Debug` is not implemented for `Foo`".

Functions are already generic in dynamic languages because values carry their actual type around,
and the type checking happens at run-time - or fails miserably. For larger programs, we really
do want to know about problems at compile-time rather! Rather than sitting down calmly with
compiler errors, a programmer in these languages has to deal with problems that only
show up when the program is running. Murphy's Law then implies that these problems
will tend to happen at the most inconvenient/disastrous time.

The operation of squaring a number is generic:  `x*x` will work for integers,
floats and generally for anything that knows about the multiplication operator `*`.
But what are the type bounds?

```rust
// gen1.rs

fn sqr<T> (x: T) -> T {
    x * x
}

fn main() {
    let res = sqr(10.0);
    println!("res {}",res);
}
```
The first problem is that Rust does not know that `T` can be multiplied:

```
error[E0369]: binary operation `*` cannot be applied to type `T`
 --> gen1.rs:4:5
  |
4 |     x * x
  |     ^
  |
note: an implementation of `std::ops::Mul` might be missing for `T`
 --> gen1.rs:4:5
  |
4 |     x * x
  |     ^
```
Following the advice of the compiler, let's constrain that type parameter using
[that trait](https://doc.rust-lang.org/std/ops/trait.Mul.html), which is used to implement the multiplication operator `*`:

```rust
fn sqr<T> (x: T) -> T
where T: std::ops::Mul {
    x * x
}
```

Which still doesn't work:

```
rror[E0308]: mismatched types
 --> gen2.rs:6:5
  |
6 |     x * x
  |     ^^^ expected type parameter, found associated type
  |
  = note: expected type `T`
  = note:    found type `<T as std::ops::Mul>::Output`
```
What `rustc` is saying that the type of `x*x` is the associated type `T::Output`, not `T`.
There's actually no reason that the type of `x*x` is the same as the type of `x`, e.g. the dot product
of two vectors is a scalar.

```rust
fn sqr<T> (x: T) -> T::Output
where T: std::ops::Mul {
    x * x
}
```

and now the error is:

```
error[E0382]: use of moved value: `x`
 --> gen2.rs:6:7
  |
6 |     x * x
  |     - ^ value used here after move
  |     |
  |     value moved here
  |
  = note: move occurs because `x` has type `T`, which does not implement the `Copy` trait
```

So, we need to constrain the type even further!

```rust
fn sqr<T> (x: T) -> T::Output
where T: std::ops::Mul + Copy {
    x * x
}
```
And that (finally) works. Calmly listening to the compiler will often get you closer
to the magic point when ... things compile cleanly.

It _is_ a bit simpler in C++:

```cpp
template <typename T>
T sqr(x: T) {
    return x * x;
}
```
but (to be honest) C++ is adopting cowboy tactics here. C++ template errors are famously
bad, because all the compiler knows (ultimately) is that some operator or method is
not defined. The C++ committee knows this is a problem and so they are working
toward [concepts](https://en.wikipedia.org/wiki/Concepts_(C%2B%2B)), which are pretty
much like trait-constrained type parameters in Rust.

Rust generic functions may look a bit overwhelming at first, but being explicit means
you will know exactly what kind of values you can safely feed it, just by looking at the
definition.

These functions are called _monomorphic_, in constrast to _polymorphic_. The body of
the function is compiled separately for each unique type.  With polymorphic functions,
the same machine code works with each matching type, dynamically _dispatching_
the correct method.

 Monomorphic produces faster code,
specialized for the particular type, and can often be _inlined_.  So when `sqr(x)` is
seen, it's effectively replaced with `x*x`.  The downside is that large generic
functions produce a lot of code, for each type used, which can result in _code bloat_.
As always, there are trade-offs; an experienced person learns to make the right choice
for the job.

## Simple Enums

Enums are types which have a few definite values. For instance, a direction has
only four possible values.

```rust
enum Direction {
    Up,
    Down,
    Left,
    Right
}
...
    // `start` is type `Direction`
    let start = Direction::Left;
```
They can have methods defined on them, just like structs.
The  `match` expression is the basic way to handle `enum` values.

```rust
impl Direction {
    fn as_str(&self) -> &'static str {
        match *self { // *self has type Direction
            Direction::Up => "Up",
            Direction::Down => "Down",
            Direction::Left => "Left",
            Direction::Right => "Right"
        }
    }
}
```

Punctuation matters. Note that `*` before `self`. It's easy to forget, because often
Rust will assume it (we said `self.first_name`, not `(*self).first_name`). However,
matching is a more exact business. Leaving it out would give a whole spew of messages,
which boil down to this type mismatch:

```
   = note: expected type `&Direction`
   = note:    found type `Direction`
```
This is because `self` has type `&Direction`, so we have to throw in the `*` to
_deference_ the type.

Like structs, enums can implement traits, and our friend `#[derive(Debug)]` can
be added to `Direction`:

```rust
        println!("start {:?}",start);
        // start Left
```
So that `as_str` method isn't really necessary, since we can always get the name from `Debug`.
(But `as_str` does _not allocate_, which may be important.)

You should not assume any particular ordering here - there's no implied integer
'ordinal' value.

Here's a method which defines the 'successor' of each `Direction` value. The
very handy _wildcard use_ temporarily puts the enum names into the method context:

```rust
    fn next(&self) -> Direction {
        use Direction::*;
        match *self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up
        }
    }
    ...

    let mut d = start;
    for _ in 0..8 {
        println!("d {:?}", d);
        d = d.next();
    }
    // d Left
    // d Up
    // d Right
    // d Down
    // d Left
    // d Up
    // d Right
    // d Down
```
So this will cycle endlessly through the various directions in this particular, arbitrary,
order. It is (in fact) a very simple _state machine_.

These enum values can't be compared:

```
assert_eq!(start, Direction::Left);

error[E0369]: binary operation `==` cannot be applied to type `Direction`
  --> enum1.rs:42:5
   |
42 |     assert_eq!(start, Direction::Left);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
note: an implementation of `std::cmp::PartialEq` might be missing for `Direction`
  --> enum1.rs:42:5
```

The solution is to say `#[derive(Debug,PartialEq)]` in front of `enum Direction`.

This is an important point - Rust user-defined types start out fresh and unadorned.
You give them sensible default behaviours by implementing the common traits. This
applies also to structs - if you ask for Rust to derive `PartialEq` for a struct it
will do the sensible thing, assume that all fields implement it and build up
a comparison. If this isn't so, or you want to redefine equality, then you are free
to define `PartialEq` explicitly.

Rust does 'C style enums' as well:

```rust
// enum2.rs

enum Speed {
    Slow = 10,
    Medium = 20,
    Fast = 50
}

fn main() {
    let s = Speed::Slow;
    let speed = s as u32;
    println!("speed {}", speed);
}
```
They are initialized with an integer value, and can be converted into that integer
with a type cast.

You only need to give the first name a value, and thereafter the
value goes up by one each time:

```rust
enum Difficulty {
    Easy = 1,
    Medium,  // is 2
    Hard   // is 3
}
```

By the way, 'name' is too vague, like saying 'thingy' all the time. The proper term here
is _variant_ - `Speed` has variants `Slow`,`Medium` and `Fast`.

These enums _do_ have a natural ordering, but you have to ask nicely.
After placing `#[derive(PartialEq,PartialOrd)]` in front of `enum Speed`, then it's indeed
true that `Speed::Fast > Speed::Slow` and `Speed::Medium != Speed::Slow`.

## Enums in their Full Glory

Rust enums in their full form are like C unions on steroids, like a Ferrari compared
to a Fiat Uno. Consider the problem of storing different values in a type-safe way.

```rust
// enum3.rs

#[derive(Debug)]
enum Value {
    Number(f64),
    Str(String),
    Bool(bool)
}

fn main() {
    use Value::*;
    let n = Number(2.3);
    let s = Str("hello".to_string());
    let b = Bool(true);

    println!("n {:?} s {:?} b {:?}", n,s,b);
}
// n Number(2.3) s Str("hello") b Bool(true)
```
Again, this enum can only contain _one_ of these values; its size will be the size of
the largest variant.

So far, not really a supercar, although it's cool that enums know how to print themselves
out. But they also know how _what kind_ of value they contain, and _that_ is the
superpower of `match`:

```rust
fn eat_and_dump(v: Value) {
    use Value::*;
    match v {
        Number(n) => println!("number is {}", n),
        Str(s) => println!("string is '{}'", s),
        Bool(b) => println!("boolean is {}", b)
    }
}
....
eat_and_dump(n);
eat_and_dump(s);
eat_and_dump(b);
//number is 2.3
//string is 'hello'
//boolean is true
```

(And that's what `Option` and `Result` are - enums.)

We like this `eat_and_dump` function, but we want to pass the value as a reference, because currently
a move takes place and the value is 'eaten':

```rust
fn dump(v: &Value) {
    use Value::*;
    match *v {  // type of *v is Value
        Number(n) => println!("number is {}", n),
        Str(s) => println!("string is '{}'", s),
        Bool(b) => println!("boolean is {}", b)
    }
}

error[E0507]: cannot move out of borrowed content
  --> enum3.rs:12:11
   |
12 |     match *v {
   |           ^^ cannot move out of borrowed content
13 |     Number(n) => println!("number is {}",n),
14 |     Str(s) => println!("string is '{}'",s),
   |         - hint: to prevent move, use `ref s` or `ref mut s`
```
There are things you cannot do with borrowed references. Rust is not letting
you _extract_ the string contained in the original value. It did not complain about `Number`
because it's happy to copy `f64`, but `String` does not implement `Copy`.

I mentioned earlier that `match` is picky about _exact_ types;
here we follow the hint and things will work; now we are just borrowing a reference
to that contained string.

```rust
fn dump(v: &Value) {
    use Value::*;
    match *v {
        Number(n) => println!("number is {}", n),
        Str(ref s) => println!("string is '{}'", s),
        Bool(b) => println!("boolean is {}", b)
    }
}
    ....

    dump(&s);
    // string is 'hello'
```
Before we move on, filled with the euphoria of a successful Rust compilation, let's
pause a little. `rustc` is unusually good at generating errors that have enough
context for a human to _fix_ the error without necessarily _understanding_ the error.

The issue is a combination of the exactness of matching, with the determination of the
borrow checker to foil any attempt to break the Rules.  One of those Rules is that
you cannot yank out a value which belongs to some owning type. Some knowledge of
C++ is a hindrance here, since C++ will copy its way out of the problem, whether that
copy even _makes sense_.  You will get exactly the same error if you try to pull out
a string from a vector, say with `*v.get(0).unwrap()` (`*` because indexing returns references.)
It will simply not let you do this. (Sometimes `clone` isn't such a bad solution to this.)

(By the way, `v[0]` does not work for non-copyable values like strings for precisely this reason.
You must either borrow with `&v[0]` or clone with `v[0].clone()`)

As for `match`, you can see `Str(s) =>` as short for `Str(s: String) =>`. A local variable
(often called a _binding_) is created.  Often that inferred type is cool, when you
eat up a value and extract its contents. But here we really needed is `s: &String`, and the
`ref` is a hint that ensures this: we just want to borrow that string.

Here we do want to extract that string, and don't care about
the enum value afterwards. `_` as usual will match anything.

```rust
impl Value {
    fn to_str(self) -> Option<String> {
        match self {
        Value::Str(s) => Some(s),
        _ => None
        }
    }
}
    ...
    println!("s? {:?}", s.to_str());
    // s? Some("hello")
    // println!("{:?}", s) // error! s has moved...
```
Naming matters - this is called `to_str`, not `as_str`. You can write a
method that just borrows that string as an `Option<&String>` (The reference will need
the same lifetime as the enum value.)  But you would not call it `to_str`.

You can write `to_str` like this - it is completely equivalent:

```rust
    fn to_str(self) -> Option<String> {
        if let Value::Str(s) = self {
            Some(s)
        } else {
            None
        }
    }
```

## More about Matching

Recall that the values of a tuple can be extracted with '()':

```rust
    let t = (10,"hello".to_string());
    ...
    let (n,s) = t;
    // t has been moved. It is No More
    // n is i32, s is String
```

This is a special case of _destructuring_; we have some
data and wish to either pull it apart (like here) or just borrow its values.
Either way, we get the parts of a structure.

The syntax is like that used in `match`. Here
we are explicitly borrowing the values.

```rust
    let (ref n,ref s) = t;
    // n and s are borrowed from t. It still lives!
    // n is &i32, s is &String
```

Destructuring works with structs as well:

```rust
    struct Point {
        x: f32,
        y: f32
    }

    let p = Point{x:1.0,y:2.0};
    ...
    let Point{x,y} = p;
    // p still lives, since x and y can and will be copied
    // both x and y are f32
```

Time to revisit `match` with some new patterns. The first two patterns are exactly like `let`
destructuring - it only matches tuples with first element zero, but _any_ string;
the second adds an `if` so that it only matches `(1,"hello")`.
Finally, just a variable matches _anything_. This is useful if the `match` applies
to an expression and you don't want to bind a variable to that expression. `_` works
like a variable but is ignored. It's a common
way to finish off a `match`.

```rust
fn match_tuple(t: (i32,String)) {
    let text = match t {
        (0, s) => format!("zero {}", s),
        (1, ref s) if s == "hello" => format!("hello one!"),
        tt => format!("no match {:?}", tt),
        // or say _ => format!("no match") if you're not interested in the value
     };
    println!("{}", text);
}
```

Why not just match against `(1,"hello")`? Matching is an exact business, and the compiler
will complain:

```
  = note: expected type `std::string::String`
  = note:    found type `&'static str`
```

Why do we need `ref s`? It's a slightly obscure gotcha (look up the E00008 error) where
if you have an _if guard_ you need to borrow, since the if guard happens in a different
context, a move will take place otherwise. It's a case of the implementation leaking
ever so slightly.

If the type _was_ `&str` then we match it directly:

```rust
    match (42,"answer") {
        (42,"answer") => println!("yes"),
        _ => println!("no")
    };
```

What applies to `match` applies to `if let`. This is a cool example, since if we
get a `Some`, we can match inside it and only extract the string from the tuple. So it
isn't necessary to have nested `if let` statements here. We use `_` because we aren't interested
in the first part of the tuple.

```rust
    let ot = Some((2,"hello".to_string());

    if let Some((_,ref s)) = ot {
        assert_eq!(s, "hello");
    }
    // we just borrowed the string, no 'destructive destructuring'
```

An interesting problem happens when using `parse` (or any function which needs to work
out its return type from context)

```rust
    if let Ok(n) = "42".parse() {
        ...
    }
```

So what's the type of `n`? You have to give a hint somehow - what kind of integer? Is it
even an integer?

```rust
    if let Ok(n) = "42".parse::<i32>() {
        ...
    }
```

This somewhat non-elegant syntax is called the 'turbofish operator'.

If you are in a function returning `Result`, then the question-mark operator provides a much
more elegant solution:

```rust
    let n: i32 = "42".parse()?;
```
However, the parse error needs to be convertible to the error type of the `Result`, which is a topic
we'll take up later when discussing [error handling](6-error-handling.html).

## Closures

A great deal of Rust's power comes from _closures_. In their simplest form, they
act like shortcut functions:

```rust
    let f = |x| x * x;

    let res = f(10);

    println!("res {}", res);
    // res 100
```

There are no explicit types in this example - everything is deduced, starting with the
integer literal 10.

We get an error if we call `f` on different types - Rust has already decided that
`f` must be called on an integer type:

```
    let res = f(10);

    let resf = f(1.2);
  |
8 |     let resf = f(1.2);
  |                  ^^^ expected integral variable, found floating-point variable
  |
  = note: expected type `{integer}`
  = note:    found type `{float}`

```

So, the first call fixes the type of the argument `x`. It's equivalent to this function:

```rust
    fn f (x: i32) -> i32 {
        x * x
    }
```

But there's a big difference between functions and closures, _apart_ from the need for explicit typing.
Here we evaluate a linear function:

```rust
    let m = 2.0;
    let c = 1.0;

    let lin = |x| m*x + c;

    println!("res {} {}", lin(1.0), lin(2.0));
    // res 3 5
```

You cannot do this with the explicit `fn` form - it does not know about variables
in the enclosing scope. The closure has _borrowed_ `m` and `c` from its context.

Now, what's the type of `lin`? Only `rustc` knows.
Under the hood, a closure is a _struct_ that is callable ('implements the call operator').
It behaves as if it was written out like this:

```rust
struct MyAnonymousClosure1<'a> {
    m: &'a f64,
    c: &'a f64
}

impl <'a>MyAnonymousClosure1<'a> {
    fn call(&self, x: f64) -> f64 {
        self.m * x  + self.c
    }
}
```
The compiler is certainly being helpful, turning simple closure syntax into all
that code! You do need to know that a closure is a _struct_ and it _borrows_ values
from its environment. And that therefore it has a _lifetime_.

All closures are unique types, but they have traits in common.
So even though we don't know the exact type, we know the generic constraint:

```rust
fn apply<F>(x: f64, f: F) -> f64
where F: Fn(f64)->f64  {
    f(x)
}
...
    let res1 = apply(3.0,lin);
    let res2 = apply(3.14, |x| x.sin());
```

In English: `apply` works for _any_ type `T` such that `T` implements `Fn(f64)->f64` - that
is, is a function which takes `f64` and returns `f64`.

After the call to `apply(3.0,lin)`, trying to access `lin` gives an interesting error:

```
    let l = lin;
error[E0382]: use of moved value: `lin`
  --> closure2.rs:22:9
   |
16 |     let res = apply(3.0,lin);
   |                         --- value moved here
...
22 |     let l = lin;
   |         ^ value used here after move
   |
   = note: move occurs because `lin` has type
    `[closure@closure2.rs:12:15: 12:26 m:&f64, c:&f64]`,
     which does not implement the `Copy` trait

```

That's it, `apply` ate our closure. And there's the actual type of the struct that
`rustc` made up to implement it. Always thinking of closures as structs is helpful.

Calling a closure is a _method call_:  the three kinds of function traits
correspond to the three kinds of methods:

  - `Fn` struct passed as `&self`
  - `FnMut` struct passed as `&mut self`
  - `FnOnce` struct passed as `self`

So it's possible for a closure to mutate its _captured_ references:

```rust
    fn mutate<F>(mut f: F)
    where F: FnMut() {
        f()
    }
    let mut s = "world";
    mutate(|| s = "hello");
    assert_eq!(s, "hello");
```

Note that `mut` - `f` needs to be mutable for this to work.

[#71: NLL makes this work]

However, you cannot escape the rules for borrowing. Consider this:

```rust
let mut s = "world";

// closure does a mutable borrow of s
let mut changer = || s = "world";

changer();
// does an immutable borrow of s
assert_eq!(s, "world");
```

Can't be done! The error is that we cannot borrow `s`
in the assert statement, because it has been previously borrowed by the
closure `changer` as mutable. As long as that closure lives, no other
code can access `s`, so the solution is to control that lifetime by
putting the closure in a limited scope:

```rust
let mut s = "world";
{
    let mut changer = || s = "world";
    changer();
}
assert_eq!(s, "world");
```

At this point, if you are used to languages like JavaScript or Lua, you may wonder at the
complexity of Rust closures compared with how straightforward they are in those languages.
This is the necessary cost of Rust's promise to not sneakily make any allocations. In JavaScript,
the equivalent `mutate(function() {s = "hello";})` will always result in a dynamically
allocated closure.

Sometimes you don't want a closure to borrow those variables, but instead _move_ them.

```rust
    let name = "dolly".to_string();
    let age = 42;

    let c = move || {
        println!("name {} age {}", name,age);
    };

    c();

    println!("name {}",name);
```

And the error at the last `println` is: "use of moved value: `name`". So one solution
here - if we _did_ want to keep `name` alive - is to move a cloned copy into the closure:

```rust
    let cname = name.to_string();
    let c = move || {
        println!("name {} age {}",cname,age);
    };
```
Why are moved closures needed? Because we might need to call them at a point where
the original context no longer exists.
A classic case is when creating a _thread_.
A moved closure does not borrow, so does not have a lifetime.

A major use of closures is within iterator methods. Recall the `range` iterator we
defined to go over a range of floating-point numbers. It's straightforward to operate
on this (or any other iterator) using closures:

```rust
    let sine: Vec<f64> = range(0.0,1.0,0.1).map(|x| x.sin()).collect();
```

`map` isn't defined on vectors (although it's easy enough to create a trait that does this),
because then _every_ map  will create a new vector.  This way, we have a choice. In this
sum, no temporary objects are created:

```rust
 let sum: f64 = range(0.0,1.0,0.1).map(|x| x.sin()).sum();
```

It will (in fact) be as fast as writing it out as an explicit loop! That performance
guarantee would be impossible if Rust closures were as 'frictionless'
as Javascript closures.

`filter` is another useful iterator method - it only lets through values that match
a condition:

```rust
    let tuples = [(10,"ten"),(20,"twenty"),(30,"thirty"),(40,"forty")];
    let iter = tuples.iter().filter(|t| t.0 > 20).map(|t| t.1);

    for name in iter {
        println!("{} ", name);
    }
    // thirty
    // forty
```

## The Three Kinds of Iterators

The three kinds correspond (again) to the three basic argument types. Assume we
have a vector of `String` values. Here are the iterator types explicitly, and
then _implicitly_, together with the actual type returned by the iterator.

```rust
for s in vec.iter() {...} // &String
for s in vec.iter_mut() {...} // &mut String
for s in vec.into_iter() {...} // String

// implicit!
for s in &vec {...} // &String
for s in &mut vec {...} // &mut String
for s in vec {...} // String
```
Personally I prefer being explicit, but it's important to understand both forms,
and their implications.

`into_iter` _consumes_ the vector and extracts its strings,
and so afterwards the vector is no longer available - it has been moved. It's
a definite gotcha for Pythonistas used to saying `for s in vec`!

 So the
implicit form `for s in &vec` is usually the one you want, just as `&T` is a good
default in passing arguments to functions.

It's important to understand how the three kinds works because Rust relies heavily
on type deduction - you won't often see explicit types in closure arguments. And this
is a Good Thing, because it would be noisy if all those types were explicitly
_typed out_. However, the price of this compact code is that you need to know
what the implicit types actually are!

`map` takes whatever value the iterator returns and converts it into something else,
but `filter` takes a _reference_ to that value. In this case, we're using `iter` so
the iterator item type is `&String`. Note that `filter` receives a reference to this type.

```rust
for n in vec.iter().map(|x: &String| x.len()) {...} // n is usize
....
}

for s in vec.iter().filter(|x: &&String| x.len() > 2) { // s is &String
...
}
```

When calling methods, Rust will derefence automatically, so the problem isn't obvious.
But `|x: &&String| x == "one"|` will _not_ work, because operators are more strict
about type matching. `rustc` will complain that there is no such operator that
compares `&&String` and `&str`. So you need an explicit deference to make that `&&String`
into a `&String` which _does_ match.

```rust
for s in vec.iter().filter(|x: &&String| *x == "one") {...}
// same as implicit form:
for s in vec.iter().filter(|x| *x == "one") {...}
```

If you leave out the explicit type, you can modify the argument so that the type of `s`
is now `&String`:

```rust
for s in vec.iter().filter(|&x| x == "one")
```

And that's usually how you will see it written.

## Structs with Dynamic Data

A most powerful technique is _a struct that contain references to itself_.

Here is the basic building block of a _binary tree_, expressed in C (everyone's
favourite old relative with a frightening fondness for using power tools without
protection.)

```rust
    struct Node {
        const char *payload;
        struct Node *left;
        struct Node *right;
    };
```

You can not do this by _directly_ including `Node` fields, because then the size of
`Node` depends on the size of `Node`... it just doesn't compute. So we use pointers
to `Node` structs, since the size of a pointer is always known.

If `left` isn't `NULL`, the `Node` will have a left pointing to another node, and so
moreorless indefinitely.

Rust does not do `NULL` (at least not _safely_) so it's clearly a job for `Option`.
But you cannot just put a `Node` in that `Option`, because we don't know the size
of `Node` (and so forth.)  This is a job for `Box`, since it contains an allocated
pointer to the data, and always has a fixed size.

So here's the Rust equivalent, using `type` to create an alias:

```rust
type NodeBox = Option<Box<Node>>;

#[derive(Debug)]
struct Node {
    payload: String,
    left: NodeBox,
    right: NodeBox
}
```
(Rust is forgiving in this way - no need for forward declarations.)

And a first test program:

```rust
impl Node {
    fn new(s: &str) -> Node {
        Node{payload: s.to_string(), left: None, right: None}
    }

    fn boxer(node: Node) -> NodeBox {
        Some(Box::new(node))
    }

    fn set_left(&mut self, node: Node) {
        self.left = Self::boxer(node);
    }

    fn set_right(&mut self, node: Node) {
        self.right = Self::boxer(node);
    }

}


fn main() {
    let mut root = Node::new("root");
    root.set_left(Node::new("left"));
    root.set_right(Node::new("right"));

    println!("arr {:#?}", root);
}
```
The output is surprisingly pretty, thanks to "{:#?}" ('#' means 'extended'.)

```
root Node {
    payload: "root",
    left: Some(
        Node {
            payload: "left",
            left: None,
            right: None
        }
    ),
    right: Some(
        Node {
            payload: "right",
            left: None,
            right: None
        }
    )
}
```
Now, what happens when `root` is dropped? All fields are dropped; if the 'branches' of
the tree are dropped, they drop _their_ fields and so on. `Box::new` may be the
closest you will get to a `new` keyword, but we have no need for `delete` or `free`.

We must now work out a use for this tree. Note that strings can be ordered:
'bar' < 'foo', 'abba' > 'aardvark'; so-called 'alphabetical order'. (Strictly speaking, this
is _lexical order_, since human languages are very diverse and have strange rules.)

Here is a method which inserts nodes in lexical order of the strings. We compare the new data
to the current node - if it's less, then we try to insert on the left, otherwise try to insert
on the right. There may be no node on the left, so then `set_left` and so forth.

```rust
    fn insert(&mut self, data: &str) {
        if data < &self.payload {
            match self.left {
                Some(ref mut n) => n.insert(data),
                None => self.set_left(Self::new(data)),
            }
        } else {
            match self.right {
                Some(ref mut n) => n.insert(data),
                None => self.set_right(Self::new(data)),
            }
        }
    }

    ...
    fn main() {
        let mut root = Node::new("root");
        root.insert("one");
        root.insert("two");
        root.insert("four");

        println!("root {:#?}", root);
    }
```

Note the `match` - we're pulling out a mutable reference to the box, if the `Option`
is `Some`, and applying the `insert` method. Otherwise, we need to create a new `Node`
for the left side and so forth. `Box` is a _smart_ pointer; note that no 'unboxing' was
needed to call `Node` methods on it!

And here's the output tree:

```
root Node {
    payload: "root",
    left: Some(
        Node {
            payload: "one",
            left: Some(
                Node {
                    payload: "four",
                    left: None,
                    right: None
                }
            ),
            right: None
        }
    ),
    right: Some(
        Node {
            payload: "two",
            left: None,
            right: None
        }
    )
}
```
The strings that are 'less' than other strings get put down the left side, otherwise
the right side.

Time for a visit. This is _in-order traversal_ - we visit the left, do something on
the node, and then visit the right.

```rust
    fn visit(&self) {
        if let Some(ref left) = self.left {
            left.visit();
        }
        println!("'{}'", self.payload);
        if let Some(ref right) = self.right {
            right.visit();
        }
    }
    ...
    ...
    root.visit();
    // 'four'
    // 'one'
    // 'root'
    // 'two'
```
So we're visiting the strings in order! Please note the reappearance of `ref` - `if let`
uses exactly the same rules as `match`.


## Generic Structs

Consider the previous example of a binary tree. It would be _seriously irritating_ to
have to rewrite it for all possible kinds of payload.
So here's our generic `Node` with its type parameter `T`.

```rust
type NodeBox<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    payload: T,
    left: NodeBox<T>,
    right: NodeBox<T>
}
```

The implementation shows the difference between the languages. The fundamental operation
on the payload is comparison, so T must be comparable with `<`, i.e. implements `PartialOrd`.
The type parameter must be declared in the `impl` block with its constraints:


```rust
impl <T: PartialOrd> Node<T> {
    fn new(s: T) -> Node<T> {
        Node{payload: s, left: None, right: None}
    }

    fn boxer(node: Node<T>) -> NodeBox<T> {
        Some(Box::new(node))
    }

    fn set_left(&mut self, node: Node<T>) {
        self.left = Self::boxer(node);
    }

    fn set_right(&mut self, node: Node<T>) {
        self.right = Self::boxer(node);
    }

    fn insert(&mut self, data: T) {
        if data < self.payload {
            match self.left {
                Some(ref mut n) => n.insert(data),
                None => self.set_left(Self::new(data)),
            }
        } else {
            match self.right {
                Some(ref mut n) => n.insert(data),
                None => self.set_right(Self::new(data)),
            }
        }
    }
}


fn main() {
    let mut root = Node::new("root".to_string());
    root.insert("one".to_string());
    root.insert("two".to_string());
    root.insert("four".to_string());

    println!("root {:#?}", root);
}
```

So generic structs need their type parameter(s) specified
in angle brackets, like C++. Rust is usually smart enough to work out
that type parameter from context - it knows it has a `Node<T>`, and knows
that its `insert` method is passed `T`. The first call of `insert` nails
down `T` to be `String`. If any further calls are inconsistent it will complain.

But you do need to constrain that type appropriately!

