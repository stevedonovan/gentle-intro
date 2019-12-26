## Pain Points

It is true to say that Rust is a harder language to learn than most
'mainstream' languages. There are exceptional people who don't find it so
difficult, but note the strict meaning of 'exceptional' - they are _exceptions_.
Many struggle at first, and then succeed. Initial difficulties aren't predictive
of later competency!

We all come from somewhere, and in the case of programming languages this
means previous exposure to mainstream languages like one of the 'dynamic'
languages like Python or one of the 'static' languages like C++. Either
way, Rust is sufficiently different to require mental retooling. Clever
people with experience jump in and are disappointed that their
cleverness is not immediately rewarded; people with less self-worth
think they aren't 'clever' enough.

For those with dynamic language experience (in which I would include
Java) everything is a reference, and all references are mutable by default.
And garbage collection _does_ make it easier to write memory-safe
programs. A lot has gone into making the JVM pretty fast, at the cost
of memory use and predicability. Often that cost is considered worth it -
the old new idea that programmer productivity is more important than
computer performance.

But most computers in the world - the ones that handle really important
things like throttle control on cars - don't have the massive resources
that even a cheap laptop has, and they need to respond to events
in _real time_. Likewise, basic software infrastructure needs to be
correct, robust, and fast (the old engineering trinity). Much of this is
done in C and C++ which are inherently unsafe - the _total cost_ of
this unsafety is the thing to look at here. Maybe you knock the program
together quicker, but _then_ the real development starts.

System languages can't afford garbage collection, because they
are the bedrock on which everything rests. They allow you to be free
to waste resources as you see fit.

If there is no garbage collection, then memory must be managed in
other ways. Manual memory management - I grab memory, use it, and
explicitly give it back - is hard to get right. You can learn enough
C to be productive and dangerous in a few weeks - but it takes years
to become a good safe C programmer, checking every possible error condition.

Rust manages memory like modern C++ - as objects are destroyed, their
memory is reclaimed. You can allocate memory on the heap with `Box`, but
as soon as that box 'goes out of scope' at the end of the function, the
memory is reclaimed. So there is something like `new` but nothing like
`delete`. You create a `File` and at the end, the file handle (a precious
resource) is closed. In Rust this is called _dropping_.

You need to share resources - it's very inefficient to make copies of
everything - and that's where things get interesting. C++ also has
references, although Rust references are rather more like C pointers -
you need to say `*r` to refer to the value, you need to say `&` to
pass a value as a reference.

Rust's _borrow checker_ makes sure that is impossible
for a reference to exist after the original value is destroyed.


## Type Inference

The distinction between 'static' and 'dynamic' isn't everything. Like with
most things, there are more dimensions in play. C is statically-typed
(every variable has a type at compile-time) but weakly-typed (e.g. `void*`
can point to _anything_); Python is dynamically-typed (the type is in
the value, not the variable) but strongly-typed. Java is static/sorta strong
(with reflection as convenient/dangerous escape valve) and Rust is
static/strong, with no runtime reflection.

Java is famous for needing all thoses types _typed out_ in numbing detail,
Rust likes to _infer_ types. This is generally a good idea, but it does
mean that you sometimes need to work out what the actual types are. You
will see `let n = 100` and wonder - what kind of integer is this? By
default, it would be `i32` - a four-byte signed integer. Everyone agrees
by now that C's unspecified integer types (like `int` and `long`) are
a bad idea; better to be explicit. You can always spell out the type,
as in `let n: u32 = 100` or let the literal force the type, as in
`let n = 100u32`.  But type inference goes much further than that!
If you declare `let n = 100` then all `rustc` knows that `n` must be
_some_ integer type. If you then passed `n` to a function expecting
a `u64` then that must be the type of `n`!

After that, you try to pass `n` to a function expecting `u32`.
`rustc` will not let you do this, because `n` has been tied down to
`u64` and it _will not_ take the easy way out and convert that
integer for you.  This is strong typing in action - there are none
of those little conversions and promotions which make your life
smoother until integer overflow bites your ass suddenly. You would have
to explicitly pass `n` as `n as u32` - a Rust typecast. Fortunately,
`rustc` is good at breaking the bad news in an 'actionable' way - that is,
you can follow the compiler's advice about fixing the problem.

So, Rust code can be very free of explicit types:

```rust
let mut v = Vec::new();
// v is deduced to have type Vec<i32>
v.push(10);
v.push(20);
v.push("hello") <--- just can't do this, man!
```
Not being able to put strings into a vector of integers is a feature,
not a bug. The flexibility of dynamic typing is also a curse.

(If you _do_ need to put integers and strings into the same vector, then
Rust `enum` types are the way to do it safely.)

Sometimes you need to at least give a type _hint_. `collect` is a
fantastic iterator method, but it needs a hint. Say I have a
iterator returning `char`. Then `collect`
can swing two ways:

```rust
// a vector of char ['h','e','l','l','o']
let v: Vec<_> = "hello".chars().collect();
// a string "doy"
let m: String = "dolly".chars().filter(|&c| c != 'l').collect();
```

When feeling uncertain about the type of a variable, there's always this
trick, which forces `rustc` to reveal the actual type name in an
error message:

```rust
let x: () = var;
```

`rustc` may pick an over-specific type. Here we want to put different
references into a vector as `&Debug` but need to declare the type
explicitly.

```rust
use std::fmt::Debug;

let answer = 42;
let message = "hello";
let float = 2.7212;

let display: Vec<&Debug> = vec![&message, &answer, &float];

for d in display {
    println!("got {:?}", d);
}
```

## Mutable References

The rule is: only one mutable reference at a time. The reason is
that tracking mutability is hard when it can happen _all over the place_.
Not obvious in dinky little programs, but things can get bad in big
codebases.

The further constraint is that you can't have immutable references while
there's a mutable reference out. Otherwise, anybody who has those
references doesn't have a guarantee that they won't change. C++ also
has immutable references (e.g. `const string&`) but does _not_ give
you this guarantee that someone can't keep a `string&` reference and modify it
behind your back.

This is a challenge if you are used to languages where every reference
is mutable! Unsafe, 'relaxed' languages depend on people understanding
their own programs and nobly deciding not to do Bad Things. But
big programs are written by more than one person and are beyond the
power of a single individual to understand in detail.

In Rust versions before 1.31, the borrow checker was less smart about lifetimes
and used to reject this program:

```rust
let mut m = HashMap::new();
m.insert("one", 1);
m.insert("two", 2);

if let Some(r) = m.get_mut("one") { // <-- mutable borrow of m
    *r = 10;
} else {
    m.insert("one", 1); // can't borrow mutably again!
}
```

Clearly this does not _really_ violate the Rules since if we got `None` we
haven't actually borrowed anything from the map.
Accordingly, recent versions of Rust have enabled _non-lexical lifetimes_, with
which the program above compiles without complaint.

In older Rust code, you might see various ugly workarounds:

```rust
let mut found = false;
if let Some(r) = m.get_mut("one") {
    *r = 10;
    found = true;
}
if ! found {
    m.insert("one", 1);
}
```

Which is yucky, but it works because the bothersome borrow is kept to
the first if-statement.

The better way here is to use `HashMap`'s [entry API](https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html).

```rust
use std::collections::hash_map::Entry;

match m.entry("one") {
    Entry::Occupied(e) => {
        *e.into_mut() = 10;
    },
    Entry::Vacant(e) => {
        e.insert(1);
    }
};
```

The borrow checker understands a few more important cases.
If you have a struct, fields can be independently borrowed. So
composition is your friend; a big struct should contain smaller
structs, which have their own methods. Defining all the mutable methods
on the big struct will lead to a situation where you can't modify
things, even though the methods might only refer to one field.

With mutable data, there are special methods for treating parts of the
data independently. For instance, if you have a mutable slice, then `split_at_mut`
will split this into two mutable slices. This is perfectly safe, since Rust
knows that the slices do not overlap.

## References and Lifetimes

Rust cannot allow a situation where a reference outlives the value. Otherwise
we would have a 'dangling reference' where it refers to a dead value -
a segfault is inevitable.

`rustc` can often make sensible assumptions about lifetimes in functions:

```rust
fn pair(s: &str, ch: char) -> (&str, &str) {
    if let Some(idx) = s.find(ch) {
        (&s[0..idx], &s[idx+1..])
    } else {
        (s, "")
    }
}
fn main() {
    let p = pair("hello:dolly", ':');
    println!("{:?}", p);
}
// ("hello", "dolly")
```

This is quite safe because we cope with the case where the delimiter isn't found.
`rustc` is here assuming that both strings in the tuple are borrowed from the
string passed as an argument to the function.

Explicitly, the function definition looks like this:

```rust
fn pair<'a>(s: &'a str, ch: char) -> (&'a str, &'a str) {...}
```
What the notation says is that the output strings live _at most as long_ as the
input string. It's not saying that the lifetimes are the same, we could drop them
at any time, just that they cannot outlive `s`.

So, `rustc` makes common cases prettier with _lifetime ellision_.

Now, if that function received _two_ strings, then you would need to
explicitly do lifetime annotation to tell Rust what output string is
borrowed from what input string.

You always need an explicit lifetime when a struct borrows a reference:

```rust
struct Container<'a> {
    s: &'a str
}
```

Which is again insisting that the struct cannot outlive the reference.
For both structs and functions, the lifetime needs to be declared in `<>`
like a type parameter.

Closures are very convenient and a powerful feature - a lot of the power
of Rust iterators comes from them. But if you store them, you have
to specify a lifetime. This is because basically a closure is a generated
struct that can be called, and that by default borrows its environment.
Here the `linear` closure has immutable references to `m` and `c`.

```rust
let m = 2.0;
let c = 0.5;

let linear = |x| m*x + c;
let sc = |x| m*x.cos()
...
```

Both `linear` and `sc` implement `Fn(x: f64)->f64` but they are _not_
the same animal - they have different types and sizes!  So to store
them you have to make a `Box<Fn(x: f64)->f64 + 'a>`.

Very irritating if you're used to how fluent closures are in Javascript
or Lua, but C++ does a similar thing to Rust and needs `std::function`
to store different closures, taking a little penalty for the virtual
call.


## Strings

It is common to feel irritated with Rust strings in the beginning. There are different
ways to create them, and they all feel verbose:

```rust
let s1 = "hello".to_string();
let s2 = String::from("dolly");
```
Isn't "hello" _already_ a string? Well, in a way. `String` is an _owned_ string,
allocated on the heap; a string literal "hello" is of type `&str` ("string slice")
and might be either baked into the executable ("static") or borrowed from a `String`.
System languages need this distinction - consider a tiny microcontroller, which has
a little bit of RAM and rather more ROM. Literal strings will get stored in ROM
("read-only") which is both cheaper and consumes much less power.

But (you may say) it's so simple in C++:

```C
std::string s = "hello";
```
Which is shorter yes, but hides the implicit creation of a string object.
Rust likes to be explicit about memory allocations, hence `to_string`.
On the other hand, to borrow from a C++ string requires `c_str`, and
C strings are stupid.

Fortunately, things are better in Rust - _once_ you accept that both `String` and `&str`
are necessary. The methods of `String` are mostly for changing the string,
like `push` adding a char (under the hood it's very much like a `Vec<u8>`).
But all the methods of `&str` are also available. By the same `Deref`
mechanism, a `String` can be passed as `&str` to a function - which is
why you rarely see `&String` in function definitions.

There are a number of ways to convert `&str` to `String`, corresponding
to various traits. Rust needs these traits to work with types generically.
As a rule of thumb, anything that implements `Display` also knows `to_string`,
like `42.to_string()`.

Some operators may not behave according to intuition:

```rust
    let s1 = "hello".to_string();
    let s2 = s1.clone();
    assert!(s1 == s2);  // cool
    assert!(s1 == "hello"); // fine
    assert!(s1 == &s2); // WTF?
```

Remember, `String` and `&String` are different types, and `==` isn't
defined for that combination. This might puzzle a C++ person who is
used to references being almost interchangeable with values.
Furthermore, `&s2` doesn't _magically_ become a `&str`, that's
a _deref coercion_ which only happens when assigning to a `&str`
variable or argument. (The explicit `s2.as_str()` would work.)

However, this more genuinely deserves a WTF:

```rust
let s3 = s1 + s2;  // <--- no can do
```
You cannot concatenate two `String` values, but you can concatenate
a `String` with a `&str`.  You furthermore cannot concatenate a
`&str` with a `String`. So mostly people don't use `+` and use
the `format!` macro, which is convenient but not so efficient.

Some string operations are available but work differently. For instance,
languages often have a `split` method for breaking up a string into an array
of strings. This method for Rust strings returns an _iterator_, which
you can _then_ collect into a vector.

```rust
let parts: Vec<_> = s.split(',').collect();
```

This is a bit clumsy if you are in a hurry to get a vector. But
you can do operations on the parts _without_ allocating a vector!
For instance, length of largest string in the split?

```rust
let max = s.split(',').map(|s| s.len()).max().unwrap();
```

(The `unwrap` is because an empty iterator has no maximum and we must
cover this case.)

The `collect` method returns a `Vec<&str>`, where the parts are
borrowed from the original string - we only need allocate space
for the references.  There is no method like this in C++, but until
recently it would have to individually allocate each substring. (C++ 17
has `std::string_view` which behaves like a Rust string slice.)

## A Note on Semicolons

Semicolons are _not_ optional, but usually left out in the same places as
in C, e.g. after `{}` blocks. They also aren't needed after `enum` or
`struct` (that's a C peculiarity.)  However, if the block must have a
_value_, then the semi-colons are dropped:

```rust
    let msg = if ok {"ok"} else {"error"};
```

Note that there must be a semi-colon after this `let` statement!

If there were semicolons after these string literals then the returned
value would be `()` (like `Nothing` or `void`). It's common error when
defining functions:

```rust
fn sqr(x: f64) -> f64 {
    x * x;
}
```

`rustc` will give you a clear error in this case.

## C++-specific Issues

### Rust value semantics are Different

In C++, it's possible to define types which behave exactly like primitives
and copy themselves. In addition, a move constructor can be defined to
specify how a value can be moved out of a temporary context.

In Rust, primitives behave as expected, but the `Copy` trait can only
be defined if the aggregate type (struct, tuple or enum) itself contains
only copyable types. Arbitrary types may have `Clone`, but you have
to call the `clone` method on values. Rust requires any allocation
to be explicit and not hide in copy constructors or assignment operators.

So, copying and moving is always defined as just moving bits around and is
not overrideable.

If `s1` is a non `Copy` value type, then `s2 = s1;` causes a move to happen,
and this _consumes_ `s1`!  So, when you really want a copy, use `clone`.

Borrowing is often better than copying, but then you must follow the
rules of borrowing. Fortunately, borrowing _is_ an overridable behaviour.
For instance, `String` can be borrowed as `&str`, and shares all the
immutable methods of `&str`. _String slices_ are very powerful compared
to the analogous C++ 'borrowing' operation, which is to extract a `const char*`
using `c_str`. `&str` consists of a pointer to some owned bytes (or a string
literal) and a _size_. This leads to some very memory-efficient patterns.
You can have a `Vec<&str>` where all the strings have been borrowed from
some underlying string - only space for the vector needs to be allocated:

For example, splitting by whitespace:

```rust
fn split_whitespace(s: &str) -> Vec<&str> {
    s.split_whitespace().collect()
}
```

Likewise, a C++ `s.substr(0,2)` call will always copy the string, but a slice
will just borrow: `&s[0..2]`.

There is an equivalent relationship between `Vec<T>` and `&[T]`.

### Shared References

Rust has _smart pointers_ like C++ - for instance, the equivalent of
`std::unique_ptr` is `Box`. There's no need for `delete`, since any
memory or other resources will be reclaimed when the box goes out of
scope (Rust very much embraces RAII).

```rust
let mut answer = Box::new("hello".to_string());
*answer = "world".to_string();
answer.push('!');
println!("{} {}", answer, answer.len());
```

People find `to_string` irritating at first, but it is _explicit_.

Note the explicit dererefence `*`, but methods on smart pointers
don't need any special notation (we do not say `(*answer).push('!')`)

Obviously, borrowing only works if there is a clearly defined owner of
the original content. In many designs this isn't possible.

In C++, this is where `std::shared_ptr` is used; copying just involves
modifying a reference count on the common data. This is not without
cost, however:

- even if the data is read-only, constantly modifying the reference
  count can cause cache invalidation
- `std::shared_ptr` is designed to be thread-safe and carries locking
  overhead as well

In Rust, `std::rc::Rc` also acts like a shared smart pointer using
reference-counting. However, it is for immutable references only! If you
want a thread-safe variant, use `std::sync::Arc` (for 'Atomic Rc').
So Rust is being a little awkward here in providing two variants, but you
get to avoid the locking overhead for non-threaded operations.

These must be immutable references because that is fundamental to Rust's
memory model. However, there's a get-out card: `std::cell::RefCell`.
If you have a shared reference defined as `Rc<RefCell<T>>` then you
can mutably borrow using its `borrow_mut` method. This applies the
Rust borrowing rules _dynamically_ - so e.g. any attempt to call
`borrow_mut` when a borrow was already happening will cause a panic.

This is still _safe_. Panics will happen
_before_ any memory has been touched inappropriately! Like exceptions,
they unroll the call stack. So it's an unfortunate word for such
a structured process - it's an ordered withdrawal rather than a
panicked retreat.

The full `Rc<RefCell<T>>` type is clumsy, but the application code isn't
unpleasant. Here Rust (again) is prefering to be explicit.

If you wanted thread-safe access to shared state, then `Arc<T>` is the
only _safe_ way to go. If you need mutable access, then `Arc<Mutex<T>>`
is the equivalent of `Rc<RefCell<T>>`. `Mutex` works a little differently
than how it's usually defined: it is a container for a value. You get
a _lock_ on the value and can then modify it.

```rust
let answer = Arc::new(Mutex::new(10));

// in another thread
..
{
  let mut answer_ref = answer.lock().unwrap();
  *answer_ref = 42;
}
```

Why the `unwrap`? If the previous holding thread panicked, then
this `lock` fails. (It's one place in the documentation where `unwrap`
is considered a reasonable thing to do, since clearly things have
gone seriously wrong. Panics can always be caught on threads.)

It's important (as always with mutexes) that this exclusive lock is
held for as little time as possible. So it's common for them to
happen in a limited scope - then the lock ends when the mutable
reference goes out of scope.

Compared with the apparently simpler situation in C++ ("use shared_ptr dude")
this seems awkward. But now any _modifications_ of shared state become obvious,
and the `Mutex` lock pattern forces thread safety.

Like everything, use shared references with [caution](https://news.ycombinator.com/item?id=11698784).

### Iterators

Iterators in C++ are defined fairly informally; they involve smart pointers,
usually starting with `c.begin()` and ending with `c.end()`. Operations on
iterators are then implemented as stand-alone template functions, like `std::find_if`.

Rust iterators are defined by the `Iterator` trait; `next` returns an `Option` and when
the `Option` is `None` we are finished.

The most common operations are now methods.
Here is the equivalent of `find_if`. It returns an `Option` (case
of not finding is `None`) and here the `if let` statement is convenient for
extracting the non-`None` case:

```rust
let arr = [10, 2, 30, 5];
if let Some(res) = arr.find(|x| x == 2) {
    // res is 2
}
```

### Unsafety and Linked Lists

It's no secret that parts of the Rust stdlib are implemented using `unsafe`. This
does not invalidate the conservative approach of the borrow checker. Remember that
"unsafe" has a particular meaning - operations which Rust cannot fully verify at
compile time. From Rust's perspective, C++ operates in unsafe mode all the time!
So if a large application needs a few dozen lines of unsafe code, then that's fine,
since these few lines can be carefully checked by a human. Humans are not good at
checking 100Kloc+ of code.

I mention this, because there appears to be a pattern:
an experienced C++ person tries to implement a linked list or a tree structure,
and gets frustrated. Well, a double-linked list _is_ possible in safe Rust,
with `Rc` references going forward, and `Weak` references going back. But the
standard library gets more performance out of using... pointers.
