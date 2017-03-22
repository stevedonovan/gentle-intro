# Basics

## Hello, World!

The original purpose of "hello world", ever since the first C version was written,
was to test the compiler and run an actual program.

```rust
// hello.rs
fn main() {
    println!("Hello, World!");
}
```

```
$ rustc hello.rs
$ ./hello
Hello, World!
```

Rust is a curly-braces language with semicolons, C++-style comments and a `main`
function - so far, so familiar.  The exclamation mark indicates that this is a
_macro_ call. For C++ programmers, this can be a turn-off, since they are used to
seriously stupid C macros - but I can ensure you that these are altogether more
capable and sane.

For anybody else,
it's probably "Great, now I have to remember when to say bang!".  However, the
compiler is unusually helpful; if you leave out that exclamation, you get:

```
error[E0425]: unresolved name `println`
 --> hello2.rs:2:5
  |
2 |     println("Hello, World!");
  |     ^^^^^^^ did you mean the macro `println!`?

```

Learning a language means getting comfortable with its errors. Try to see the compiler
as a strict but friendly helper rather than a computer _shouting_ at you, because you are
going to see a lot of red ink in the beginning.  It's much better for the compiler
to catch you out than for your program to blow up in front of actual humans.

The next step is to introduce a _variable_:

```rust
// let1.rs
fn main() {
    let answer = 42;
    println!("Hello {}", answer);
}

```

Spelling mistakes are _compile_ errors, not runtime errors like with dynamic languages
like Python or JavaScript.  This will save you a lot of stress later! And if I wrote
 'answr' instead of 'answer', the compiler is actually _nice_ about it:

```
4 |     println!("Hello {}", answr);
  |                         ^^^^^ did you mean `answer`?

```

The `println!` macro takes a [format string](https://doc.rust-lang.org/std/fmt/index.html)
and some values; it's very similar to the formatting used by Python 3.

Another very useful macro is `assert_eq!`. This is the workhorse of testing
in Rust; you _assert_ that two things must be equal, and if not, _panic_.

```rust
// let2.rs
fn main() {
    let answer = 42;
    assert_eq!(answer,42);
}
```

Which won't produce any output. But change 42 to 40:

```
thread 'main' panicked at
'assertion failed: `(left == right)` (left: `42`, right: `40`)',
let2.rs:4
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```
And that's our first _runtime error_ in Rust.

## Looping and Ifing

Anything interesting can be done more than once:

```rust
// for1.rs
fn main() {
    for i in 0..5 {
        println!("Hello {}", i);
    }
}
```

The _range_ is not inclusive, so `i` goes from 0 to 4. This is convenient in a
language which _indexes_ things like arrays from 0.

And interesting things have to be done _conditionally_:

```rust
// for2.rs
fn main() {
    for i in 0..5 {
        if i % 2 == 0 {
            println!("even {}", i);
        } else {
            println!("odd {}", i);
        }
    }
}
```

```
even 0
odd 1
even 2
odd 3
even 4
```

`i % 2` is zero if 2 can divide into `i` cleanly; Rust uses C-style operators.
There are _no_ brackets around the condition, just like in Go, but
you _must_ use curly brackets around the block.

This does the same, written in a more interesting way:

```rust
// for3.rs
fn main() {
    for i in 0..5 {
        let even_odd = if i % 2 == 0 {"even"} else {"odd"};
        println!("{} {}", even_odd, i);
    }
}
```

Traditionally, programming languages have _statements_ (like `if`) and
_expressions_ (like `1+i`). In Rust, nearly everything has a value and can
be an expression.  The seriously ugly C 'ternary operator' `i % 2 == 0 ? "even" : "odd"`
is not needed.

Note that there aren't any semi-colons in those blocks!

## Adding Things Up

Computers are very good at arithmetic. Here is a first attempt at adding all
the numbers from 0 to 4:

```rust
// add1.rs
fn main() {
    let sum = 0;
    for i in 0..5 {
        sum += i;
    }
    println!("sum is {}", sum);
}
```

But it fails to compile:

```
error[E0384]: re-assignment of immutable variable `sum`
 --> add1.rs:5:9
3 |     let sum = 0;
  |         --- first assignment to `sum`
4 |     for i in 0..5 {
5 |         sum += i;
  |         ^^^^^^^^ re-assignment of immutable variable

```

'Immutable'? A variable that cannot _vary_?  `let` variables by default can only
be assigned a value when declared. Adding the magic word `mut` (_please_ make
this variable mutable) does the trick:

```rust
// add2.rs
fn main() {
    let mut sum = 0;
    for i in 0..5 {
        sum += i;
    }
    println!("sum is {}", sum);
}
```

This can be puzzling when coming from other languages, where variables can be
re-written by default. What makes something a 'variable' is that it gets assigned
a computed value at run-time - it is not a _constant_.
It is pretty much how the word is used in mathematics, like when we say
'let n be the largest number in set S'.

There is a reason for declaring variables to be _read-only_ by default. In a larger
program, it gets hard to track where writes are taking place. So Rust makes things
like mutability ('write-ability') explicit. There's a lot of cleverness in the
language, but it never hides anything.

Rust is both statically-typed and strongly-typed - these are often confused, but
think of C (statically but weakly typed) and Python (dynamically but strongly typed).
In static types the type is known at compile time, and dynamic types are only known
at run time.

At the moment, it feels like Rust is _hiding_ those types from you. What
exactly is the type of `i`?  The compiler can work it out, starting with 0,
with _type inference_, and comes up with `i32` (four byte signed integer.)

Let's make exactly one change - turn that `0` into `0.0`. Then we get errors:

```
error[E0277]: the trait bound `{float}: std::ops::AddAssign<{integer}>` is not satisfied
 --> add3.rs:5:9
  |
5 |         sum += i;
  |         ^^^^^^^^ the trait `std::ops::AddAssign<{integer}>` is not implemented for `{float}`
  |

```

Ok, so the honeymoon is over: what does this mean? Each operator (like `+=`) corresponds
to a _trait_, which is like an abstract interface that must be implemented for each concrete type.
We'll deal with traits in detail later, but here all you need to know is that
`AddAssign` is the name of the trait implementing the `+=` operator, and the error is saying
that floating point numbers do not implement this operator for a integer. (The full list of
operator traits is [here](https://doc.rust-lang.org/std/ops/index.html).


Again, Rust likes to be explicit - it will not silently convert that integer into a float for you.

We have to _cast_ that value to a floating-point value explicitly.

```rust
// add3.rs
fn main() {
    let mut sum = 0.0;
    for i in 0..5 {
        sum += i as f64;
    }
    println!("sum is {}", sum);
}
```

## Get Explicit

_Functions_ are one place where the compiler will not work out types for you.
And this in fact was a deliberate decision, since languages like Haskell have
such powerful type inference that there are hardly any explicit type names. It's
a case of the language being somewhat more intelligent than its users, since
it's good Haskell style to put in a comment giving the function argument types. This feels
self-defeating.

Here is a simple user-defined function:

```rust
// fun1.rs

fn sqr(x: f64) -> f64 {
    return x * x;
}

fn main() {
    let res = sqr(2.0);
    println!("square is {}", res);
}
```

Rust (like Go, which it superficially resembles in many ways) goes back to an
older style of argument declaration, where the type follows the name. This is
how it was done in Algol-derived languages like Pascal.

Again, no integer-to-float conversions - if you replace the `0.0` with `0` then we
get a clear error:

```
8 |     let res = sqr(0);
  |                   ^ expected f64, found integral variable
  |
```

You will actually rarely see functions written using a `return` statement. More
often, it will look like this:

```rust
fn sqr(x: f64) -> f64 {
    x * x
}
```

This is because the body of the function (inside `{}`) has the value of its
last expression, just like with if-as-an-expression.

Since semicolons are inserted semi-automatically by human fingers, you might add it
here and get the following error:

```
  |
3 | fn sqr(x: f64) -> f64 {
  |                       ^ expected f64, found ()
  |
  = note: expected type `f64`
  = note:    found type `()`
help: consider removing this semicolon:
 --> fun2.rs:4:8
  |
4 |     x * x;
  |       ^

```

The `()` type is the empty type, nada, `void`, zilch, nothing. Everything in Rust
has a value, but sometimes it's just nothing.  The compiler knows this is
a common mistake, and actually _helps_ you.  (Anybody who has spent time with a
C++ compiler will know how _damn unusual_ this is.)

A few more examples of this no-return expression style:

```rust
// absolute value of a floating-point number
fn abs(x: f64) -> f64 {
    if x > 0.0 {
        x
    } else {
        -x
    }
}

// ensure the number always falls in the given range
fn clamp(x: f64, x1: f64, x2: f64) -> f64 {
    if x < x1 {
        x1
    } else if x > x2 {
        x2
    } else {
        x
    }
}
```

It's not wrong to use `return`, but code is cleaner without it. You will still
use `return` for _returning early_ from a function.

Some operations can be elegantly expressed _recursively_:

```rust
fn factorial(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorial(n-1)
    }
}
```
This can be a little strange at first, and the best thing is then to use pencil and paper
and work out some examples. It isn't usually the most _efficient_ way to do that
operation however.

What if you want a function to modify one of its arguments?  Enter _mutable references_:

```rust
// fun4.rs

fn modifies(x: &mut f64) {
    *x = 1.0;
}

fn main() {
    let mut res = 0.0;
    modifies(&mut res);
    println!("res is {}", res);
}
```
This is more how C would do it than C++. You have to explicitly pass the
reference (with `&`) and explicitly _dereference_ with `*`. And then throw in `mut`
because it's not the default. (I've always felt that C++ references are
too easy to miss compared to C.)

Basically, Rust is introducing some _friction_ here, and not-so-subtly pushing
you towards returning values from functions directly.  Fortunately, Rust has
powerful ways to express things like "operation succeeded and here's the result"
so `&mut` isn't needed that often.

The type-after-variable style applies to `let` as well, when you really want to nail
down the type of a variable:

```rust
let bigint: i64 = 0;
```

## Learning Where to Find the Ropes

It's time to start using the documentation. This will be installed on your machine,
and you can use `rustup doc --std` to open it in a browser.

Note the _search_ field at the top, since this
is going to be your friend; it operates completely offline.

Let's say we want to see where the mathematical
functions are, so search for 'cos'. The first two hits show it defined for both
single and double-precision floating point numbers.  It is defined on the
_value itself_ as a method, like so:

```rust
let pi = 3.1416;
let x = pi/2.0;
let cosine = x.cos();
```
And the result will be sort-of zero; we obviously need a more authoritative source
of pi-ness!

Let me quote the example given for `cos`, but written as a complete program
( `assert!` is a cousin of `assert_eq!`; the expression must be true):

```rust
fn main() {
    let x = 2.0 * std::f64::consts::PI;

    let abs_difference = (x.cos() - 1.0).abs();

    assert!(abs_difference < 1e-10);
}
```
`std::f64::consts::PI` is a mouthful! `::` means much the same as it does in C++,
(often written using '.' in other languages) - it is a _fully qualified name_. We get
this full name from the second hit on searching for `PI`.

Up to now, our little Rust programs have been free of all that `import` and
`include` stuff that tends to slow down the discussion of 'Hello World' programs.
Let's make this program more readable with a `use` statement:

```rust
use std::f64::consts;

fn main() {
    let x = 2.0 * consts::PI;

    let abs_difference = (x.cos() - 1.0).abs();

    assert!(abs_difference < 1e-10);
}
```
Why haven't we needed to do this up to now?
This is because Rust helpfully makes a lot of basic functionality visible without
explicit `use` statements.

## Arrays and Slices

All statically-typed languages have _arrays_, which are values packed nose to tail
in memory. Arrays are _indexed_ from zero:

```rust
// array1.rs
fn main() {
    let arr = [10, 20, 30, 40];
    let first = arr[0];
    println!("first {}", first);

    for i in 0..4 {
        println!("[{}] = {}", i,arr[i]);
    }
    println!("length {}", arr.len());
}
```

And the output is:

```
first 10
[0] = 10
[1] = 20
[2] = 30
[3] = 40
length 4
```

In this case, Rust knows _exactly_ how big the array is and if you try to
access `arr[4]` it will be a _compile error_.

Learning a new language often involves _unlearning_ mental habits from languages
you already know; if you are a Pythonista, then those brackets say `List`. We will
come to the Rust equivalent of `List` soon, but arrays are not the droids you're looking
for; they are _fixed in size_. They can be _mutable_ (if we ask nicely) but you
cannot add new elements.

Arrays are not used that often in Rust, because the type of an array includes its
size.  The type of the array in the example is
`[i32; 4]`; the type of `[10, 20]` would be `[i32; 2]` and so forth: they
have _different types_.  So they are basically bastards to pass around as
function arguments.

What _are_ used often are _slices_. You can think of these as _views_ into
an underlying array of values. They otherwise behave very much like an array, and
_know their size_, unlike those dangerous animals C pointers.

Note two important things here - how to write a slice's type, and that
you have to use `&` to pass it to the function.

```rust
// array2.rs
// read as: slice of i32 array
fn sum(values: &[i32]) -> i32 {
    let mut res = 0;
    for i in 0..values.len() {
        res += values[i]
    }
    res
}

fn main() {
    let arr = [10,20,30,40];
    // look at that &
    let res = sum(&arr);
    println!("sum {}", res);
}
```

Ignore the code of `sum` for a while, and look at `&[i32]`. The relationship between
Rust arrays and slices is similar to that between C arrays and pointers, except for
two important differences - Rust slices keep track of their size (and will
panic if you try to access outside that size) and you have to explicitly say that
you want to pass an array as a slice using the `&` operator.

A C programmer pronounces `&` as 'address of'; a Rust programmer pronounces it
'borrow'. This is going to be the key word when learning Rust. Borrowing is the name
given to a common pattern in programming; whenever you pass something by reference
(as nearly always happens in dynamic languages) or pass a pointer in C. Anything
borrowed remains the 'property' of the original owner.

## Slicing and Dicing

You cannot print out an array in the usual way with `{}` but you can do a _debug_
print with `{:?}`.

```rust
// array3.rs
fn main() {
    let ints = [1, 2, 3];
    let floats = [1.1, 2.1, 3.1];
    let strings = ["hello", "world"];
    let ints_ints = [[1, 2], [10, 20]];
    println!("ints {:?}", ints);
    println!("floats {:?}", floats);
    println!("strings {:?}", strings);
    println!("ints_ints {:?}", ints_ints);
}
```

Which gives:

```
ints [1, 2, 3]
floats [1.1, 2.1, 3.1]
strings ["hello", "world"]
ints_ints [[1, 2], [10, 20]]
```

So, arrays of arrays are no problem, but the important thing is that an array contains
values of _only one type_.  The values in an array are arranged next to each other
in memory so that `[10, 20]` takes 8 bytes (two 4-byte signed integers). This makes
them _very_ efficient to access.

If you are curious about the actual types of these variables, here is a useful trick.
Just declare a variable with an explicit type which you know will be wrong:

```rust
let var: () = [1.1, 1.2];
```
Here is the informative error:

```
3 |     let var: () = [1.1, 1.2];
  |                   ^^^^^^^^^^ expected (), found array of 2 elements
  |
  = note: expected type `()`
  = note:    found type `[{float}; 2]`
```

Slices give you different _views_ of the _same_ array:

```rust
// slice1.rs
fn main() {
    let ints = [1, 2, 3, 4, 5];
    let slice1 = &ints[0..2];
    let slice2 = &ints[1..];  // open range!

    println!("ints {:?}", ints);
    println!("slice1 {:?}", slice1);
    println!("slice2 {:?}", slice2);
}
```

```
ints [1, 2, 3, 4, 5]
slice1 [1, 2]
slice2 [2, 3, 4, 5]
```

This is a neat notation which looks similar to Python slices but with a big difference:
a copy of the data is never made.  These slices all _borrow_ their data from their
arrays. They have a very intimate relationship with that array, and Rust spends a lot
of effort to make sure that relationship does not break down.

## Optional Values

Slices, like arrays, can be _indexed_. Rust knows the size of an array at
compile-time, but the size of a slice is only known at run-time. So `s[i]` can
cause an out-of-bounds error when running and will _panic_.  This is really not
what you want to happen - it can be the difference between a safe launch abort and
scattering pieces of a very expensive satellite all over Florida. And there are
_no exceptions_.

Let that sink in, because it comes as a shock. You cannot wrap dodgy-may-panic
code in some try-block and 'catch the error' - at least not in a way you'd want to use
every day. So how can Rust be safe?

There is a slice method `get` which does not panic. But what does it return?

```rust
// slice2.rs
fn main() {
    let ints = [1, 2, 3, 4, 5];
    let slice = &ints;
    let first = slice.get(0);
    let last = slice.get(5);

    println!("first {:?}", first);
    println!("last {:?}", last);
}
// first Some(1)
// last None
```

`last` failed (forgot zero-based indexing), but returned something called `None`.
`first` is fine, but appears as a value wrapped in `Some`.  Welcome to the `Option`
type!  It may be _either_ `Some` or `None`.

The `Option` type has some useful methods:

```rust
    println!("first {} {}", first.is_some(), first.is_none());
    println!("last {} {}", last.is_some(), last.is_none());
    println!("first value {}", first.unwrap());

// first true false
// last false true
// first value 1
```
If you were to _unwrap_ `last`, you would get a panic. But at least you can call
`is_some` first to make sure - for instance, if you had a distinct no-value default:

```rust
    let maybe_last = slice.get(5);
    let last = if maybe_last.is_some() {
        maybe_last.unwrap()
    } else {
        -1
    };
```
Which is long-winded, so there's a shortcut (the `&` is because `get` always 
returns a reference):

```rust
    let last = slice.get(5).unwrap_or(&-1);
```
You can think of `Option` as a box which may contain a value, or nothing (`None`).
(It is called `Maybe` in Haskell). It may contain _any_ kind of value, which is
its _type parameter_. In this case, the full type is `Option<i32>`, using
C++-style notation for _generics_.  Unwrapping this box may cause an explosion,
but unlike Schroedinger's Cat, we know if it contains a value up-front.

It is very common for Rust functions/methods to return such maybe-boxes, so learn
how to use them safely!

## Vectors

We'll return to slice methods again, but first: vectors. These are _re-sizeable_
arrays and behave much like Python `List` and C++ `std::vector`. The Rust type
`Vec` (pronounced 'vector') behaves very much like an slice in fact; the
difference is that you can append extra values to a vector - note that it must
be declared as mutable.

```rust
// vec1.rs
fn main() {
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);

    let first = v[0];  // will panic if out-of-range
    let maybe_first = v.get(0);

    println!("v is {:?}", v);
    println!("first is {}", first);
    println!("maybe_first is {:?}", maybe_first);
}
// v is [10, 20, 30]
// first is 10
// maybe_first is Some(10)
```
A common beginner mistake is to forget the `mut`; you will get a helpful error
message:

```
3 |     let v = Vec::new();
  |         - use `mut v` here to make mutable
4 |     v.push(10);
  |     ^ cannot borrow mutably
```

There is a very intimate relation between vectors and slices:

```rust
// vec2.rs
fn dump(arr: &[i32]) {
    println!("arr is {:?}", arr);
}

fn main() {
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);

    dump(&v);

    let slice = &v[1..];
    println!("slice is {:?}", slice);
}
```
That little, so-important borrow operator `&` is _coercing_ the vector into a
slice. And it makes complete sense, because the vector is looking after an array of
values, with the difference that the array is allocated _dynamically_.

If you come from a dynamic language, now is time for that little talk. In systems
languages, program memory comes in two kinds: the stack and the heap. It is very fast
to allocate data on the stack, but the stack is limited; typically of the order of
megabytes. The heap can be gigabytes, but allocating is relatively expensive, and
such memory must be _freed_ later. In so-called 'managed' languages (like Java, Go
and the so-called 'scripting' languages) these details are hidden from you by that
convenient municipal utility called the _garbage collector_. Once the system is sure
that data is no longer referenced by other data, it goes back into the pool
of available memory.

Generally, this is a price worth paying. Playing with the stack is terribly unsafe,
because if you make one mistake you can override the return address of the current
function, and you die an ignominious death or (worse) got pwned by some guy living
in his Mom's basement in Minsk.

The first C program I wrote (on an DOS PC)
took out the whole computer. Unix systems always behaved better, and only the process died
with a _segfault_. Why is this worse than a Rust (or Go) program panicking?
Because the panic happens when the original problem happens, not when the program
has become hopelessly confused and eaten all your homework. Panics are _memory safe_
because they happen before any illegal access to memory. This is a common cause of
security problems in C, because all memory accesses are unsafe and a cunning attacker
can exploit this weakness.

Panicking sounds desperate and unplanned, but Rust panics are structured - the stack is _unwound_
just as with exceptions. All allocated objects are dropped, and a backtrace is generated.

The downsides of garbage collection? The first is that it is wasteful of memory, which
matters in those small embedded microchips which increasingly rule our world. The
second is that it will decide, at the worst possible time, that a clean up must happen
_now_. (The Mom analogy is that she wants to clean your room when you are at a
delicate stage with a new lover). Those embedded systems need to respond to things
_when they happen_ ('real-time') and can't tolerate unscheduled outbreaks of
cleaning. Roberto Ierusalimschy, the chief designer of Lua (one of the most elegant
dynamic languages ever) said that he would not like to fly on an airplane that
relied on garbage-collected software.

Back to vectors: when a vector is modified or created, it allocates from the heap and becomes
 the _owner_ of that memory. The slice _borrows_ the array from the vector.
When the vector dies or _drops_, it lets the memory go.

## Iterators

We have got so far without mentioning a key part of the Rust puzzle - iterators.
The for-loop over a range was using an iterator (`0..n` is actually similar to the
Python 3 `range` function).

An iterator is easy to define informally. It is an 'object' with a `next` method
which returns an `Option`. As long as that value is not `None`, we keep calling
`next`:

```rust
// iter1.rs
fn main() {
    let mut iter = 0..3;
    assert_eq!(iter.next(), Some(0));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), None);
}
```
And that is exactly what `for var in iter {}` does.

This may seem an inefficient way to define a for-loop, but `rustc` does crazy-ass
optimizations in release mode and it will be just as fast as a `while` loop. (Contrast
with the situation on the JVM, where iterators always require allocation. The consequence
is that funky functional stuff in Scala is _always_ going to be significantly slower
than in Rust).

Here is the first attempt to iterate over an array:

```rust
// iter2.rs
fn main() {
    let arr = [10, 20, 30];
    for i in arr {
        println!("{}", i);
    }
}
```
which fails, but helpfully:
```
4 |     for i in arr {
  |     ^ the trait `std::iter::Iterator` is not implemented for `[{integer}; 3]`
  |
  = note: `[{integer}; 3]` is not an iterator; maybe try calling
   `.iter()` or a similar method
  = note: required by `std::iter::IntoIterator::into_iter`
```

Following `rustc`'s advice, the following program works as expected.

```rust
// iter3.rs
fn main() {
    let arr = [10, 20, 30];
    for i in arr.iter() {
        println!("{}", i);
    }

    // slices can be converted implicitly to iterators...
    let slice = &arr;
    for i in slice {
        println!("{}", i);
    }
}
```
In fact, it is more efficient to iterate over an array or slice this way
than to use `for i in 0..slice.len() {}` because Rust does not have to obsessively
check every index operation.

We had an example of summing up a range of integers earlier. It involved a `mut`
variable and a loop. Here's the _idiomatic_, pro-level way of doing the sum:

```rust
// sum1.rs
fn main() {
    let sum: i32  = (0..5).sum();
    println!("sum was {}", sum);

    let sum: i64 = [10, 20, 30].iter().sum();
    println!("sum was {}", sum);
}
```

Note that this is one of those cases where you need to be explicit about
the _type_ of the variable, since otherwise Rust doesn't have enough information.
Here we do sums with two different integer sizes, no problem. (It is also no
problem to create a new variable of the same name if you run out of names to
give things.)

With this background, some more of the slice methods will make more sense.
Another documentation tip; on the right-hand side of every page there's a '[-]' which you can
click to collapse the method list. You can then expand the details of anything
that looks interesting. (Anything that looks too weird, just ignore for now.)

The `windows` method gives you an iterator of slices - overlapping windows of
values!

```rust
// slice4.rs
fn main() {
    let ints = [1, 2, 3, 4, 5];
    let slice = &ints;

    for s in slice.windows(2) {
        println!("window {:?}", s);
    }
}
// window [1, 2]
// window [2, 3]
// window [3, 4]
// window [4, 5]
```
Or `chunks`:

```rust
    for s in slice.chunks(2) {
        println!("chunks {:?}", s);
    }
// chunks [1, 2]
// chunks [3, 4]
// chunks [5]
```

## More about vectors...

There is a useful little macro `vec!` for initializing a vector. Note that you
can _remove_ values from the end of a vector using `pop`, and _extend_ a vector
using any compatible iterator.

```rust
// vec3.rs
fn main() {
    let mut v1 = vec![10, 20, 30, 40];
    v1.pop();

    let mut v2 = Vec::new();
    v2.push(10);
    v2.push(20);
    v2.push(30);

    assert_eq!(v1, v2);

    v2.extend(0..2);
    assert_eq!(v2, &[10, 20, 30, 0, 1]);
}
```
Vectors compare with each other and with slices by value.

You can insert values into a vector at arbitrary positions with `insert`,
and remove with `remove`. This is not as efficient as pushing and popping since
the values will have to be moved to make room, so watch out for these operations on big
vectors.

Vectors have a size and a _capacity_. If you `clear` a vector, its size becomes zero,
but it still retains its old capacity. So refilling it with `push`, etc only requires
reallocation when the size gets larger than that capacity.

Vectors can be sorted, and then duplicates can be removed - these operate in-place
on the vector. (If you want to make a copy first, use `clone`.)

```rust
// vec4.rs
fn main() {
    let mut v1 = vec![1, 10, 5, 1, 2, 11, 2, 40];
    v1.sort();
    v1.dedup();
    assert_eq!(v1, &[1, 2, 5, 10, 11, 40]);
}
```

## Strings

Strings in Rust are a little more involved than in dynamic languages; the `String` type,
like `Vec`, allocates dynamically and is resizeable. (So it's like C++'s `std::string`
but not like the immutable strings of Java and Python.) A program may contain a lot
of _string literals_ (like "hello") and a system language should be able to store
these statically in the executable itself. In embedded micros, that could mean putting
them in cheap ROM rather than expensive RAM (for low-power devices, RAM is
also expensive in terms of power consumption.)

So "hello" is not of type `String`. It is of type `&str` (pronounced 'string slice').
It's like the distinction between `const char*` and `std::string` in C++, except
`&str` is much more intelligent.  In fact, `&str` and `String` have a very
similar relationship to each other as do `&[T]` to `Vec<T>`.

```rust
// string1.rs
fn dump(s: &str) {
    println!("str '{}'", s);
}

fn main() {
    let text = "hello dolly";  // the string slice
    let s = text.to_string();  // it's now an allocated string

    dump(text);
    dump(&s);
}
```
Again, the borrow operator can coerce `String` into `&str`, just as `Vec` could
be coerced into `&[]`.

In C++, char pointers can become strings implicitly through the constructor and
through assignment,  but you need the ugly `c_str`
method to get the owned char pointer of the string. And that's an important difference
in emphasis; Rust is making the _allocation_ obvious, but makes it easier to borrow
the characters as a string slice.

Like a vector, you can `push` a character and `pop` one off the end.

```rust
// string5.rs
fn main() {
    let mut s = String::new();
    // initially empty!
    s.push('H');
    s.push_str("ello");
    s.push(' ');
    s += "World!"; // short for `push_str`
    // remove the last char
    s.pop();

    assert_eq!(s, "Hello World");
}
```
You can convert many types to strings using `to_string`
(essentially, if you can display them with '{}' then they can be converted).
The `format!` macro is a very useful way to build
up more complicated strings using the same format strings as `println!`.

```rust
// string5.rs
fn array_to_str(arr: &[i32]) -> String {
    let mut res = '['.to_string();
    for v in arr {
        res += &v.to_string();
        res.push(',');
    }
    res.pop();
    res.push(']');
    res
}

fn main() {
    let arr = array_to_str(&[10, 20, 30]);
    let res = format!("hello {}", arr);

    assert_eq!(res, "hello [10,20,30]");
}
```
Note the `&` in front of `v.to_string()` - the operator is defined on a string
slice, not a `String` itself, so it needs a little persuasion to match.

The notation used for slices works with strings as well:

```rust
// string2.rs
fn main() {
    let text = "static";
    let string = "dynamic".to_string();

    let text_s = &text[1..];
    let string_s = &string[2..4];

    println!("slices {:?} {:?}", text_s, string_s);
}
// slices "tatic" "na"
```

Again, this is superior to the C++ approach, which is to use the `substr`
method - which _makes a copy_. In the Rust case, it's just a borrow.

But, you cannot index strings!  This is because they use the One True Encoding,
UTF-8, where a 'character' may be a number of bytes.

```rust
// string3.rs
fn main() {
    let multilingual = "Hi! ¡Hola! привет!";
    for ch in multilingual.chars() {
        print!("'{}' ", ch);
    }
    println!("");
    println!("len {}", multilingual.len());
    println!("count {}", multilingual.chars().count());

    let maybe = multilingual.find('п');
    if maybe.is_some() {
        let hi = &multilingual[maybe.unwrap()..];
        println!("Russian hi {}", hi);
    }
}
// 'H' 'i' '!' ' ' '¡' 'H' 'o' 'l' 'a' '!' ' ' 'п' 'р' 'и' 'в' 'е' 'т' '!'
// len 25
// count 18
// Russian hi привет!
```

Now, let that sink in - there are 25 bytes, but only 18 characters! However, if
you use a method like `find`, you will get a valid index (if found) and then
any slice will be fine.

(The Rust `char` type is a 4-byte Unicode code point. Strings are _not_ arrays
of chars!)

String slicing may explode like vector indexing, because it uses byte offsets. In this case,
the string consists of two bytes, so trying to pull out the first byte is a Unicode error. So be
careful to only slice strings using valid offsets that come from string methods.

```rust
    let s = "¡";
    println!("{}", &s[0..1]); <-- bad, first byte of a multibyte character
```

Breaking up strings is a popular and useful pastime. The string `split_whitespace`
method returns an _iterator_, and we then choose what to do with it. `collect`
is very general and so needs some clues about _what_ it is collecting - hence
the explicit type.

```rust
    let text = "the red fox and the lazy dog";
    let words: Vec<&str> = text.split_whitespace().collect();
    // ["the", "red", "fox", "and", "the", "lazy", "dog"]
```
You could also say it like this, passing the iterator into the `extend` method:

```rust
    let mut words = Vec::new();
    words.extend(text.split_whitespace());
```
If written in C++, we would have to make these _allocated strings_, whereas
here each slice in the vector is borrowing from the original string. All we
allocate is the space to keep the slices.

Have a look at this cute two-liner; we get an iterator over the chars,
and only take those characters which are not space. Again, `collect` needs
a clue (we may have wanted a vector of chars, say):

```rust
    let stripped: String = text.chars()
        .filter(|ch| !ch.is_whitespace()).collect();
    // theredfoxandthelazydog
```
The `filter` method takes a _closure_, which is Rust-speak for what C++ calls
lambdas or anonymous functions.  Here the argument type is clear from the
context, so the explicit rule is relaxed.

Yes, you can do this as an explicit loop over chars, pushing the returned slices
into a mutable vector, but this is shorter, reads well (_when_ you are used to it,
of course) and just as fast. It is not a _sin_ to use a loop, however, and I encourage
you to write that version as well.

## Interlude: Getting Command Line Arguments

Up to now our programs have lived in blissful ignorance of the outside world; now
it's time to feed them data.

`std::env::args` is how you access command-line arguments; it returns an iterator
over the arguments as strings, including the program name.

```rust
// args0.rs
fn main() {
    for arg in std::env::args() {
        println!("'{}", arg);
    }
}
```
```
src$ rustc args0.rs
src$ ./args0 42 'hello dolly' frodo
'./args0'
'42'
'hello dolly'
'frodo'
```
Would it have been better to return a `Vec`? It's easy enough to use `collect` to
make that vector, using the iterator `skip` method to move past the program
name.

```rust
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() > 0 { // we have args!
        ...
    }
```
Which is fine; it's pretty much how you would do it in C.

A more Rust-y approach to reading a single argument (together with parsing an
integer value):

```rust
// args1.rs
use std::env;

fn main() {
    let first = env::args().nth(1).expect("please supply an argument");
    let n: i32 = first.parse().expect("not an integer!");
    // do your magic
}
```
`nth(1)` gives you the second argument from the iterator, and `expect`
is like an `unwrap` with a readable message.

Converting a string into a number is straightforward, but you do need to specify
the type of the value - how else could `parse` know?

This program can panic, which is fine for dinky test programs. But don't get too
comfortable with this convenient habit.

## Matching

The code in `string3.rs` where we extract the Russian greeting is not how it would
be usually written. Enter _match_:

```rust
    match multilingual.find('п') {
        Some(idx) => {
            let hi = &multilingual[idx..];
            println!("Russian hi {}", hi);
        },
        None => println!("couldn't find the greeting, Товарищ")
    };
```
`match` consists of several _patterns_ with a matching value following the fat arrow,
separated by commas.  It has conveniently unwrapped the value from the `Option` and
bound it to `idx`.  You _must_ specify all the possibilities, so we have to handle
`None`.

Once you are used to it (and by that I mean, typed it out in full a few times) it
feels more natural than the explicit `is_some` check which needed an extra
variable to store the `Option`.

But if you're not interested in failure here, then `if let` is your friend:

```rust
    if let Some(idx) = multilingual.find('п') {
        println!("Russian hi {}", &multilingual[idx..]);
    }
```
This is convenient if you want to do a match and are _only_ interested in one possible
result.

`match` can also operate like a C `switch` statement, and like other Rust constructs
can return a value:

```rust
    let text = match n {
        0 => "zero",
        1 => "one",
        2 => "two",
        _ => "many",
    };
```
The `_` is like C `default` - it's a fall-back case. If you don't provide one then
`rustc` will consider it an error. (In C++ the best you can expect is a warning, which
says a lot about the respective languages).

Rust `match` statements are more powerful than `switch`:

```rust
    let text = match n {
        0...3 => "small",
        4...6 => "medium",
        _ => "large",
     };
```
## Reading from Files

The next step to exposing our programs to the world is to _reading files_.

Recall that `expect` is like `unwrap` but gives a custom error message. We are
going to throw any a few errors here:

```rust
// file1.rs
use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let first = env::args().nth(1).expect("please supply a filename");

    let mut file = File::open(&first).expect("can't open the file");

    let mut text = String::new();
    file.read_to_string(&mut text).expect("can't read the file");

    println!("file had {} bytes", text.len());

}
```
```
src$ file1 file1.rs
file had 366 bytes
src$ ./file1 frodo.txt
thread 'main' panicked at 'can't open the file: Error { repr: Os { code: 2, message: "No such file or directory" } }', ../src/libcore/result.rs:837
note: Run with `RUST_BACKTRACE=1` for a backtrace.
src$ file1 file1
thread 'main' panicked at 'can't read the file: Error { repr: Custom(Custom { kind: InvalidData, error: StringError("stream did not contain valid UTF-8") }) }', ../src/libcore/result.rs:837
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```
So `open` can fail because the file doesn't exist or we aren't allowed to read it,
and `read_to_string` can fail because the file doesn't contain valid UTF-8. (Which is
fair enough, you can use `read_to_end` and put the contents into a vector of bytes
instead.) For files that aren't too big, reading them in one gulp is useful and
straightforward.

If you know anything about file handling in other languages, you may wonder when
the file is _closed_. If we were writing to this file, then not closing it could
result in loss of data.
But the file here is closed when the function ends and the `file` variable is _dropped_.

This 'throwing away errors' thing is getting too much of a habit. You do not
want to put this code into a function, knowing that it could so easily crash
the whole program.  So now we have to talk about exactly what `File::open` returns.
If `Option` is a value that may contain something or nothing, then `Result` is a value
that may contain something or an error. They both understand `unwrap` (and its cousin
`expect`) but they are quite different.

This version defines a function that does not crash. It passes on a `Result` and
it is the _caller_ who must decide how to handle the error.

```rust
// file2.rs
use std::env;
use std::fs::File;
use std::io::Read;
use std::io;

fn read_to_string(filename: &str) -> io::Result<String> {
    let mut file = match File::open(&filename) {
        Ok(f) => f,
        Err(e) => return Err(e),
    };
    let mut text = String::new();
    match file.read_to_string(&mut text) {
        Ok(_) => Ok(text),
        Err(e) => Err(e),
    }
}

fn main() {
    let file = env::args().nth(1).expect("please supply a filename");

    let text = read_to_string(&file).expect("bad file man!");

    println!("file had {} bytes", text.len());
}
```
The `Result` 'box' has two compartments, one labelled `Ok`
and the other `Err`.

The first match safely extracts the value from `Ok`, which
becomes the value of the match. If it's `Err` it just returns the error.
The second match returns the string, wrapped up as an `Ok`, otherwise
(again) the error. The actual value in the `Ok` is unimportant, so we ignore
it with `_`.

This is not so pretty; when most of a function is error handling, then
the 'happy path' gets lost. Go tends to have this problem, with lots of
explicit early returns, or just _ignoring errors_.  (That is, by the way,
the closest thing to evil in the Rust universe.)

Fortunately, there is a shortcut:

```rust
fn read_to_string(filename: &str) -> io::Result<String> {
    let mut file = File::open(&filename)?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;
    Ok(text)
}
```
That `?` operator does almost exactly what the first match did; if the
result was an error, then it will immediately return that error. At the end, we
still need to wrap up the string as a result.

It's been a good year in Rust, and `?` was one of the cool things that
became stable recently. You will still see the macro `try!` used in older code:

```rust
fn read_to_string(filename: &str) -> io::Result<String> {
    let mut file = try!(File::open(&filename));
    let mut text = String::new();
    try!(file.read_to_string(&mut text));
    Ok(text)
}
```

In summary, it's possible to write perfectly safe Rust that isn't ugly, without
needing exceptions.


