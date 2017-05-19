## Object-Orientation in Rust

Everyone comes from somewhere, and the chances are good that your previous programming language
implemented Object-Oriented Programming (OOP) in a particular way:

  * 'classes' act as factories for generating _objects_ (often called _instances_)
  and define unique types.
  * Classes may _inherit_ from other classes (their _parents_), inheriting both data (_fields_)
  and behaviour (_methods_)
  * If B inherits from A, then an instance of B can be passed to something expecting A
  (_subtyping_)
  * An object should hide its data (_encapsulation_), which can only be operated on
  with methods.

Object-oriented _design_ is then about identifying the classes (the 'nouns') and the methods
(the 'verbs') and then establishing relationships between them, _is-a_ and _has-a_.

There was a point in the old Star Trek series where the doctor would say to the captain,
"It's Life, Jim, just not Life as we know it". And this applies very much to Rust-flavoured
object-orientation: it comes as a shock, because Rust data aggregates (structs, enums
and tuples) are dumb. You can define methods on them, and make the data itself private,
all the usual tactics of encapsulation, but they are all _unrelated types_.
There is no subtyping and no inheritance of data (apart from the specialized 
case of `Deref` coercions.)

The relationships between various data types in Rust are
established using _traits_.  A large part of learning Rust is understanding how the
standard library traits operate, because that's the web of meaning that glues all the
data types together.

Traits are interesting because there's no one-to-one correspondence between them and concepts
from mainstream languages. It depends if you're thinking dynamically or statically. In the
dynamic case, they're rather like Java or Go interfaces.

For instance,  it becomes automatic to slap a `#[derive(Debug)]`
on any new types, because the innards of those types can be printed out using `{:?}`. All
stdlib types implement `Debug`:

```rust
let question = "everything";
let answer = 42;

let debugables: Vec<&Debug> = vec![&question,&answer];
for d in &debugables {
    println!("{:?}",d);
}
// "everything"
// 42
```

Now, we could just use `{:?}` to dump the innards of the vector directly, but I want
to emphasize we are iterating over references that are all guaranteed to have the `fmt`
method defined. A vector of references to values implementing `Debug` can be printed out.
That's a clunky thing  to say often, so they are called `Debug` _trait objects_.
Now integers and strings otherwise don't have much in common, but here we've found a common denominator;
they know how to print out their innards.
And this is how you get traditional dynamic dispatch in Rust, because that `fmt` method
defined by the `Debug` trait is called as a virtual method.

'virtual methods' is a concept that only applies to statically-compiled languages; they are the
_default_ in Java, unless you sternly say `final`. In a language like Python, the object has
all of its class information attached, and the interpreter will look up `fmt` in the class
object at runtime.

A little refinement of this example - we _box_ the values. A box contains a reference to data
allocated on the heap, and acts very much like a reference - it's a _smart pointer_. When boxes
go out of scope and `Drop` kicks in, then that memory is released.

```rust
    let question = Box::new("everything");
    let answer = Box::new(42);

    let debugables: Vec<Box<Debug>> = vec![question,answer];
    for d in &debugables {
        println!("{:?}",d);
    }
```

The difference is that you can take this vector, pass it as a reference, give it away, without
having to track any borrowed references. When the vector is dropped, the boxes will be dropped,
and all memory is reclaimed.

## Animals

For some reason, any discussion of OOP and inheritance seems to end up talking about animals. It
makes for a nice story: "See, a Cat is a Carnivore. And a Carnivore is an Animal". But I'll start
with a classic slogan from the Ruby universe: "if it quacks, it's a duck". All your objects have
to do is define `quack` and they can be considered to be ducks, albeit in a very narrow way.

```rust

trait Quack {
    fn quack(&self);
}

struct Duck ();

impl Quack for Duck {
    fn quack(&self) {
        println!("quack!");
    }
}

struct RandomBird {
    is_a_parrot: bool
}

impl Quack for RandomBird {
    fn quack(&self) {
        if ! self.is_a_parrot {
            println!("quack!");
        } else {
            println!("squawk!");
        }
    }
}

let duck1 = Duck();
let duck2 = RandomBird{is_a_parrot: false};
let parrot = RandomBird{is_a_parrot: true};

let ducks: Vec<&Quack> = vec![&duck1,&duck2,&parrot];

for d in &ducks {
    d.quack();
}
// quack!
// quack!
// squawk!
```

Here we have two completely different types (one is so dumb it doesn't even have data), and yes,
they all `quack()`. One is behaving a little odd (for a duck) but they share the same method name
and Rust can keep a collection of such objects in a type-safe way.

Rust is a little over-obsessed with memory safety (since that's the _big thing_ it brings to
system languages) but type safety is a fantastic thing.  Without static typing, you might insert
a _cat_ into that collection of Quackers, resulting in run-time chaos.

Here's a funny one:

```rust
// and why the hell not!
impl Quack for i32 {
    fn quack(&self) {
        for i in 0..*self {
            print!("quack {} ",i);
        }
        println!("");
    }
}

let int = 4;

let ducks: Vec<&Quack> = vec![&duck1,&duck2,&parrot,&int];
...
// ...
// quack 0 quack 1 quack 2 quack 3
```

What can I say? It quacks, it must be a duck. What's interesting is that you can apply your traits
to any Rust value, not just 'objects'. (Since `quack` is passed a reference, there's an explicit
dereference `*` to get the integer.)

However, you can't do this with a trait and a type from the same crate, so the standard library
cannot be 'monkey patched', which is another piece of Ruby folk practice (and not the most wildly
admired either.)

Up to this point, the trait `Quack` was behaving very much like a Java interface, and like
modern Java interfaces you can have _provided_ methods which supply a default implementation
if you have implemented the _required_ methods. (The `Iterator` trait is a good example.)

But, note that traits are not part of the _definition_ of a type and you can define and implement
new traits on any type, subject to the same-crate restriction.

It's possible to pass a reference to any `Quack` implementor:

```rust
fn quack_ref (q: &Quack) {
    q.quack();
}

quack_ref(&d);
```

And that's subtyping, Rust-style.

Since we're doing Programming Language Comparisons 101 here, I'll mention that Go has an interesting
take on the quacking business - if there's a Go interface `Quack`, and a type has a `quack` method,
then that type satisfies `Quack` without any need for explicit definition. This also breaks the
baked-into-definition Java model, and allows compile-time duck-typing, at the cost of some
clarity and type-safety.

But there is a problem with duck-typing.
One of the signs of bad OOP is too many methods which have some
generic name like `run`. "If it has run(), it must be Runnable" doesn't sound so catchy as
the original!  So it is possible for a Go interface to be _accidentally_ valid. In Rust,
both the `Debug` and `Display` traits define `fmt` methods, but they really mean different
things.

So Rust traits allow traditional _polymorphic_ OOP.  But what about inheritance? People usually
mean _implementation inheritance_ whereas Rust does _interface inheritance_.  It's as if a Java
programmer never used `extend` and instead used `implements`. And this is actually
[recommended practice](http://www.javaworld.com/article/2073649/core-java/why-extends-is-evil.html)
by Alan Holub. He says:

> I once attended a Java user group meeting where James Gosling (Java's inventor) was the featured
> speaker. During the memorable Q&A session, someone asked him: "If you could do Java over again,
> what would you change?" "I'd leave out classes," he replied. After the laughter died down,
> he explained that the real problem wasn't classes per se, but rather implementation inheritance
> (the extends relationship). Interface inheritance (the implements relationship) is preferable.
> You should avoid implementation inheritance whenever possible

So even in Java, you've probably been doing it wrong!

Implementation inheritance has some serious problems. But it does feel so very
_convenient_. There's this fat base class called `Animal` and it has loads of useful
functionality (it may even expose its innards!) which our derived class `Cat` can use. That is,
it is a form of code reuse. But code reuse is a separate concern.

Getting the distinction between implementation and interface inheritance is important when
understanding Rust.

Note that traits may have _provided_ methods. Consider `Iterator` - you only _have_ to override
`next`, but get a whole host of methods free.  This is similar to 'default' methods of modern
Java interfaces. Here we only define `name` and `upper_case` is defined for us. We _could_
override `upper_case` as well, but it isn't _required_.

```rust
trait Named {
    fn name(&self) -> String;
    
    fn upper_case(&self) -> String {
        self.name().to_uppercase()
    }
}

struct Boo();

impl Named for Boo {
    fn name(&self) -> String {
        "boo".to_string()
    }
}

let f = Boo();

assert_eq!(f.name(),"boo".to_string());
assert_eq!(f.upper_case(),"BOO".to_string());
```
This is a _kind_ of code reuse, true, but note that it does not apply to data, only the interface!

## Ducks and Generics

An example of generic-friendly duck function in Rust would be this trivial one:

```rust
fn quack<Q> (q: &Q)
where Q: Quack {
    q.quack();
}

let d = Duck();
quack(&d);
```

The type parameter is _any_ type which implements `Quack`. There's an important difference
between `quack` and `quack_ref`.  The body of this function is compiled for _each_ of the calling
types and no virtual method is needed; such functions can be completely inlined. It
uses the trait `Quack` in a different way, as a _constraint_ on generic types.

This is the C++ equivalent to the generic `quack` (note the `const`):

```cpp
template <class Q>
void quack(const Q& q) {
    q.quack();
}
```

Note that the type parameter is not constrained in any way.

This is very much compile-time duck-typing - if we pass a reference to a
non-quackable type, then the compiler will complain bitterly about no `quack` method.
At least the error is found at compile-time, but it's worse when a type is accidentally
Quackable, as happens with Go. More involved template functions and classes lead to
terrible error messages, because there are _no_ constraints on the generic types.

You could define a function which could handle an iteration over Quacker pointers:

```cpp
template <class It>
void quack_everyone (It start, It finish) {
    for (It i = start; i != finish; i++) {
        (*i)->quack();
    }
}
```

This would then be implemented for _each_ iterator type `It`.
The Rust equivalent is a little more challenging:

```rust
fn quack_everyone <I> (iter: I)
where I: Iterator<Item=Box<Quack>> {
    for d in iter {
        d.quack();
    }
}

let ducks: Vec<Box<Quack>> = vec![Box::new(duck1),Box::new(duck2),Box::new(parrot),Box::new(int)];

quack_everyone(ducks.into_iter());
```

Iterators in Rust aren't duck-typed but are types that must implement `Iterator`, and in
this case the iterator provides boxes of `Quack`.  There's no ambiguity about the types
involved, and the values must satisfy `Quack`. Often the function signature is the most challenging
thing about a generic Rust function, which is why I recommend reading the source - the
implementation is often much simpler! Here the only type parameter is the actual iterator type,
which means that this will work with anything that can deliver a sequence of boxes, not just
a vector iterator.

## Inheritance

A common problem with object-oriented design is trying to force things into a _is-a_ relationship,
and neglecting _has-a_ relationships. The [GoF](https://en.wikipedia.org/wiki/Design_Patterns)
said "Prefer Composition to Inheritance" in their Design Patterns book, twenty-two years ago.

Here's an example: you want to model the employees of some company, and `Employee` seems a good
name for a class.  Then, Manager is-a Employee (this is true) so we start building our
hierarchy with a `Manager` subclass of `Employee`. This isn't as smart as it seems. Maybe we got
carried away with identifying important Nouns, maybe we (unconsciously) think that managers and
employees are different kinds of animals?  It's better for `Employee` to has-a `Roles` collection,
and then a manager is just an `Employee` with more responsibilities and capabilities.

Or consider Vehicles - ranging from bicycles to 300t ore trucks. There are multiple ways to think
about vehicles, road-worthiness (all-terrain, city, rail-bound, etc), power-source (electric,
diesel, diesel-electric, etc), cargo-or-people, and so forth.  Any fixed hierarchy of classes
you create based on one aspect ignores all other aspects. That is, there are multiple possible
classifications of vehicles!

Composition is more important in Rust for the obvious reason that you can't inherit functionality
in a lazy way from a base class.

Composition is also important because the borrow checker is smart enough
to know that borrowing different struct fields are separate borrows. You can have
a mutable borrow of one field while having an immutable borrow of another field,
and so forth. Rust cannot tell that a method only accesses one field, so the
fields should be structs with their own methods for implementation convenience.
(The _external_ interface of the struct can be anything you like using suitable traits.)

There is, however, a restricted but very important kind of
'inheritance' with [Deref](https://rust-lang.github.io/book/second-edition/ch15-02-deref.html),
which is the trait for the 'dereference' operator `*`.
`String` implements `Deref<Target=str>` and so all the methods defined on `&str` are automatically
available for `String` as well!  In a similar way, the methods of `Foo` can be directly 
called on `Box<Foo>`.  Some find this a little ... magical, but it is tremendously convenient.
There is a simpler language inside modern Rust, but it would not be half as pleasant to use.

Generally, there is _trait inheritance_:

```rust
trait Show {
    fn show(&self) -> String;
}

trait Location {
    fn location(&self) -> String;
}

trait ShowTell: Show + Location {}
```

The last trait simply combines our two distinct traits into one, although it could specify
other methods.

Things now proceed as before:

```
#[derive(Debug)]
struct Foo {
    name: String,
    location: String
}

impl Foo {
    fn new(name: &str, location: &str) -> Foo {
        Foo{
            name: name.to_string(),
            location: location.to_string()
        }
    }
}

impl Show for Foo {
    fn show(&self) -> String {
        self.name.clone()
    }
}

impl Location for Foo {
    fn location(&self) -> String {
        self.location.clone()
    }
}

impl ShowTell for Foo {}

```

Now, if I have a value `foo` of type `Foo`, then a reference to that value will
satisfy `&Show`, `&Location` or `&ShowTell` (which implies both.)

Here's a useful little macro:

```rust
macro_rules! dbg {
    ($x:expr) => {
        println!("{} = {:?}",stringify!($x),$x);
    }
}
```
It takes one argument (represented by `$x`) which must be an 'expression'. We print out its
value, and a _stringified_ version of the value. C programmers can be a _little_ smug at this point,
but this means that if I passed `1+2` (an expression) then `stringify!(1+2)` is the literal
string "1+2". This will save us some typing when playing with code:

```rust
let foo = Foo::new("Pete","bathroom");
dbg!(foo.show());
dbg!(foo.location());

let st: &ShowTell = &foo;

dbg!(st.show());
dbg!(st.location());

fn show_it_all(r: &ShowTell) {
    dbg!(r.show());
    dbg!(r.location());
}

let boo = Foo::new("Alice","cupboard");
show_it_all(&boo);

fn show(s: &Show) {
    dbg!(s.show());
}

show(&boo);

// foo.show() = "Pete"
// foo.location() = "bathroom"
// st.show() = "Pete"
// st.location() = "bathroom"
// r.show() = "Alice"
// r.location() = "cupboard"
// s.show() = "Alice"
```

This _is_ object-orientation, just not the kind you may be used to.

Please note that the `Show` reference passed to `show` can not be _dynamically_
upgraded to a `ShowTell`!  Languages with more dynamic class systems allow you to
check whether a given object is an instance of a class and then to do a
dynamic cast to that type. It isn't really a good idea in general, and specifically
cannot work in Rust because that `Show` reference has 'forgotten' that it was originally
a `ShowTell` reference.

You always have a choice: polymorphic, via trait objects, or monomorphic, via generics
constrainted by traits. Modern C++ and the Rust standard library tends to take the generic
route, but the polymorphic route is not obselete. You do have to understand the different
trade-offs - generics generate the fastest code, which can be inlined. This may lead
to code bloat. But not everything needs to be as _fast as possible_ - it may only happen
a 'few' times in the lifetime of a typical program fun.

So, here's a summary:

  - structs and enums are dumb, although you can define methods and do data hiding.
  - a _limited_ form of subtyping is possible on data using `Deref`
  - traits don't have any data, but can be implemented for any type (not just structs.)
  - traits can inherit from other traits
  - traits can have provided methods, allowing interface code re-use
  - traits give you both virtual methods (polymorphism) and generic constraints (monomorphism)

## Example: Windows API

One of the areas where traditional OOP is used extensively is GUI toolkits. An `EditControl` or a `ListWindow`
is-a `Window`, and so forth. This makes writing Rust bindings to GUI toolkits more difficult
than it needs to be.

Win32 programming can be done [directly](https://www.codeproject.com/Tips/1053658/Win-GUI-Programming-In-Rust-Language)
in Rust, and it's a little less awkward than the original C. As soon as I graduated from C
to C++ I wanted something cleaner and did my own OOP wrapper.

A typical Win32 API function is [ShowWindow](https://docs.rs/user32-sys/0.0.9/i686-pc-windows-gnu/user32_sys/fn.ShowWindow.html)
which is used to control the visibility of a window. Now, an `EditControl` has some specialized
functionality, but it's all done with a Win32 `HWND` ('handle to window') opaque value.
You would like `EditControl` to also have a `show` method, which traditionally would be done
by implementation inheritance. You _not_ want to have to type out all these inherited methods
for each type! But Rust traits provide a solution. There would be a `Window` trait:

```rust
trait Window {
    // you need to define this!
    fn get_hwnd(&self) -> HWND;

    // and all these will be provided
    fn show(&self, visible: bool) {
        unsafe {
         user32_sys::ShowWindow(self.get_hwnd(), if visible {1} else {0})
        }
    }

    // ..... oodles of methods operating on Windows

}
```

So, the implementation struct for `EditControl` can just contain a `HWND` and implement `Window`
by defining one method; `EditControl` is a trait that inherits from `Window` and defines the extended
interface.  Something like `ComboxBox` - which behaves like an `EditControl` _and_ a
`ListWindow` can be easily implemented with trait inheritance.

The Win32 API ('32' no longer means '32-bit' anymore) is in fact object-oriented, but an
older style, influenced by Alan Kay's definition: objects contain hidden data, and are operated
on by _messages_. So at the heart of any Windows application there's a message loop, and
the various kinds of windows (called 'window classes') implement these methods with their
own switch statements.  There is a message called `WM_SETTEXT` but the implementation can be
different: a label's text changes, but a top-level window's caption changes.

[Here](https://gabdube.github.io/native-windows-gui/book_20.html) is a rather promising
minimal Windows GUI framework. But to my taste, there are too many `unwrap` instances
going on - and some of them aren't even errors. This is because NWG is exploiting the
loose dynamic nature of messaging.  With a proper type-safe interface, more errors are
caught at compile-time.

The [next edition](https://rust-lang.github.io/book/second-edition/ch17-00-oop.html)
of The Rust Programming Language book has a very good discussion on what 'object-oriented'
means in Rust.


