## Maps

In this section I'll briefly introduce some very useful parts of the Rust standard
library. The documentation (as always) is excellent but a little motivation is
useful.

_Maps_  (sometimes called _associative arrays_ or _dicts_) let you look up values
associated with a _key_.  It's not really a fancy concept, and can be done with
an array of tuples:

```rust
    let entries = [("one","eins"),("two","zwei"),("three","drei")];

    if let Some(val) = entries.iter().find(|t| t.0 == "two") {
        assert_eq!(val.1,"zwei");
    }
```

This is fine for small maps and just requires equality to be defined for the keys,
but the search takes linear time - proportional to the size of the map.

A `HashMap` does much better when there are a _lot_ of key/value pairs to be
searched:

```rust
use std::collections::HashMap;

let mut map = HashMap::new();
map.insert("one","eins");
map.insert("two","zwei");
map.insert("three","drei");

assert_eq! (map.contains_key("two"), true);
assert_eq! (map.get("two"), Some(&"zwei"));
```

'&"zwei"'? This is because `get` returns a _reference_ to the value, not the value
itself. Here the value type is `&str`, so we get a `&&str`. In general it _has_ to be
a reference, because we can't just _move_ a value out of its owning type.

`get_mut` is like `get` but returns a possible mutable reference. Here we have
a map from strings to integers, and wish to update the value for the key 'two':

```rust
let mut map = HashMap::new();
map.insert("one",1);
map.insert("two",2);
map.insert("three",3);

println!("before {}",map.get("two").unwrap());

{
    let mut mref = map.get_mut("two").unwrap();
    *mref = 20;
}

println!("after {}",map.get("two").unwrap());
// before 2
// after 20
```

Note that getting that writable reference takes place in its own block - otherwise,
we would have a mutable borrow lasting until the end, and then Rust won't allow
you to borrow from `map` again with `map.get("two")`; it cannot allow any readable
references while there's already a writable reference in scope. (If it did, it could
not guarantee that those readable references would remain valid.)
So the solution is to make
sure that mutable borrow doesn't last very long.

It is not the most elegant API possible, but we can't throw away any possible
errors. Python would bail out with an exception, and C++ would just create
a default value. (This is convenient but sneaky; easy to forget that the price
of `map["two"]` always returning an integer is that we can't tell the difference
between zero and 'not found', _plus_ an extra entry is created!)

And no-one just calls `unwrap`, except in examples. However, most Rust code you see consists
of little standalone examples!  Much more likely for a match to take place:

```rust
if let Some(v) = map.get("two") {
    let res = v + 1;
    assert_eq!(res, 3);
}
...
match map.get_mut("two") {
    Some(mref) => *mref = 20,
    None => panic!("_now_ we can panic!")
}
```

We can iterate over the key/value pairs, but not in any particular order.

```rust
for (k,v) in map.iter() {
    println!("key {} value {}", k,v);
}
// key one value eins
// key three value drei
// key two value zwei
```

There are also `keys` and `values` methods returning iterators over the keys and
values respectively, which makes creating vectors of values easy.

## Example: Counting Words

An entertaining thing to do with text is count word length frequency. It is straightforward
to break text into words with `split_whitespace`, but really we must respect punctuation.
So the words should be defined as consisting only of alphabetic characters.
And the words need to be compared as lower-case as well.

Doing a mutable lookup on a map is straightforward, but also handling the case where the
lookup fails is a little awkward.  Fortunately there's an elegant
way to update the values of a map:

```rust
let mut map = HashMap::new();

for s in text.split(|c: char| ! c.is_alphabetic()) {
    let word = s.to_lowercase();
    let mut count = map.entry(word).or_insert(0);
    *count += 1;
}
```

If there's no existing count corresponding to a word, then let's create a new entry
containing zero for that word and _insert_ it into the map. Its exactly what a C++
map does, except it's done explicitly and not sneakily.

There is exactly one explicit type in this snippet, and that's the `char` needed
because of a quirk of the string `Pattern` trait used by `split`.
But we can deduce that the key type is `String` and the value type is `i32`.

Using [The Adventures of Sherlock Holmes](http://www.gutenberg.org/cache/epub/1661/pg1661.txt)
from Project Gutenberg, we can test this out
more thoroughly.  The total number of unique words (`map.len`) is 8071.

How to find the twenty most common words? First, convert the map into a vector
of (key,value) tuples. (This consumes the map, since we used `into_iter`.)

```rust
let mut entries: Vec<_> = map.into_iter().collect();
```
Next we can sort in descending order. `sort_by` expects the result of the `cmp`
method that comes from the `Ord` trait, which is implemented by the
integer value type:

```rust
    entries.sort_by(|a,b| b.1.cmp(&a.1));
```

 And finally print out the first twenty entries:

```rust
    for e in entries.iter().take(20) {
        println!("{} {}", e.0, e.1);
    }
```

(Well, you _could_ just loop over `0..20` and index the vector here - it isn't wrong,
just a little un-idiomatic - and potentially more expensive for big iterations.)


```
 38765
the 5810
and 3088
i 3038
to 2823
of 2778
a 2701
in 1823
that 1767
it 1749
you 1572
he 1486
was 1411
his 1159
is 1150
my 1007
have 929
with 877
as 863
had 830
```

A little sunrise - what's that empty word? It is because `split` works on single-character
delimiters, so any punctuation or extra spaces causes a new split.

## Sets

Sets are maps where you care only about the keys, not any associated values.
So `insert` only takes one value, and you use `contains` for testing whether a value
is in a set.

Like all containers, you can create a `HashSet` from an iterator. And this
is exactly what `collect` does, once you have given it the necessary type hint.

```
// set1.rs
use std::collections::HashSet;

fn make_set(words: &str) -> HashSet<&str> {
    words.split_whitespace().collect()
}

fn main() {
    let fruit = make_set("apple orange pear orange");

    println!("{:?}",fruit);
}
// {"orange", "pear", "apple"}
```
Note (as expected) that repeated insertions of the same key have no effect, and the order
of values in a set are not important.

They would not be sets without the usual operations:

```rust
let fruit = make_set("apple orange pear");
let colours = make_set("brown purple orange yellow");

for c in fruit.intersection(&colours) {
    println!("{:?}",c);
}
// "orange"
```
They all create iterators, and you can use `collect` to make these into sets.

Here's a shortcut, just as we defined for vectors:

```rust
use std::hash::Hash;

trait ToSet<T> {
    fn to_set(self) -> HashSet<T>;
}

impl <T,I> ToSet<T> for I
where T: Eq + Hash, I: Iterator<Item=T> {

    fn to_set(self) -> HashSet<T> {
       self.collect()
    }
}

...

let intersect = fruit.intersection(&colours).to_set();
```
As with all Rust generics, you do need to constrain types - this can only be
implemented for types that understand equality (`Eq`) and for which a 'hash function'
exists (`Hash`). Remember that there is no _type_ called `Iterator`, so `I` defines
any type that _implements_ `Iterator`.

This technique for implementing our own methods on standard library types may appear
to be a little too powerful, but again, there are Rules. We can only do this for our
own traits. If both the struct and the trait came from the same crate (particularly,
the stdlib) then such implemention would not be allowed. In this way, you are
saved from creating confusion.

Before congratulating ourselves on such a clever, convenient shortcut, you should be
aware of the consequences. If `make_set` was written so, so that these are sets
of owned strings, then the actual type of `intersect` could come as a sunrise:

```rust
fn make_set(words: &str) -> HashSet<String> {
    words.split_whitespace().map(|s| s.to_string()).collect()
}
...
// intersect is HashSet<&String>!
let intersect = fruit.intersection(&colours).to_set();
```
And it cannot be otherwise, since Rust will not suddenly start making copies of owned
strings. `intersect` contains a single `&String` borrowed from `fruit`. I can promise
you that this will cause you trouble later, when you start patching up lifetimes!
A better solution is to use the iterator's `cloned` method to make owned string copies
of the intersection.

```rust
// intersect is HashSet<String> - much better
let intersect = fruit.intersection(&colours).cloned().to_set();
```

A more robust definition of `to_set` might be `self.cloned().collect()`,
which I invite you to try out.

## Queues

`Vec` has a classic stack interface - it's efficient to push (at to the end) or pop
(remove from the end). This is often called LIFO (last in first out).

Vectors aren't that efficient at inserting or removing anywhere else. Sometimes what
you want is a _queue_ - FIFO (first in first out).  This is what `VecDeque` provides;
items are added at the end and removed from the front.

```rust
// queue1.rs
use std::collections::VecDeque;

fn main() {
    let mut queue = VecDeque::new();
    queue.push_back(10);
    queue.push_back(20);
    assert_eq! (queue.pop_front(),Some(10));
    assert_eq! (queue.pop_front(),Some(20));
}
```
A common way of using queues is to fill them up and empty them in order of insertion
(doing this with a `Vec` would get the words in reverse order.)

```rust
queue.push_back("hello");
queue.push_back("dolly");
while let Some(word) = queue.pop_front() {
    print!("{} ", word);
}
println!("");
// hello dolly
```

Queues behave like vectors in many ways. `get(i)` returns a reference to an element
at an index (maybe), and `queue[i]` will return a reference (or panic)

How do I know this?
Because it implements the trait `Index`, which is how Rust does the overloaded
index operator. There's also `IndexMut`, which kicks in when the indexing is on the
left side `queue[0] = "foo"`.

A powerful method (also implemented for `Vec`) is `extend`

```rust
    queue.extend(1..10);
    queue.extend(12..15);
    // queue is now 1 2 3 4 5 6 7 8 9 12 13 14
```

Again, this comes from implementing the `Extend` trait. You will see that many
methods of containers aren't defined directly on the type, but on traits. This makes
it possible to write generic code using them - you can't rely just on compile-time
duck-typing like C++ templates ("If it says extend(), then it's Extendable"). But it
does making reading the documentation a little more challenging.

Let's have a look at some other trait implementations in the docs for `VecDeque`:

  - `PartialOrd` queues can be compared with '<', '>', '<=' and '=<'. (But only if
  the value type also satisfies `PartialOrd`)
  - `Eq` queues can be compared with '==' (same proviso)
  - `Clone` queues can be cloned  (same proviso)
  - `From` queues can be constructed from vectors: `let q = VecDeque::from(v)`
  - `Hash` queues can be used as keys in a hash map (seriously)
  - `IntoIterator` queues can be iterated directly using `for`. Note the three
  implementations, corresponding to `for v in q`, `for v in &q` and
  `for v in &mut q`.
  ....

But `VecDeque` doesn't have that intimate relationship with array slices `&[T]`
that `Vec` has, expressed by the `Deref` trait.

## Example: Interactive command processing

It's often useful to have an interactive session with a program. Each line is read in and
split into words; the command is looked up on the first word, and the rest of the words
are passed as an argument to that command.

A natural implementation is a map from command names to closures. It's tempting as
first to make them `FnMut` - that is, they can modify any captured variables. But we will
have more than one command, each with its own closure, and you cannot then mutably borrow
the same variables.

```rust
// this is a no-no
let mut a = 10;
let set = |n| a = n;
let get = || a;  // can't borrow `a` again!
```

So the closures are passed a _mutable reference_ and
an array slice of string slices (`&[&str]`) and return some `Result`.
We'll use `String` errors at first.

Recall that all closures
implementing a particular function signature are all distinct types, and are (in fact)
compiler-generated structs that _borrow_ variables from the environment of the closure.
They are different types and may well have different sizes, so we put them in a `Box`.

Any struct that borrows references needs an explicit lifetime, since Rust needs
assurance that these references outlive our `Cli` struct - so closures have lifetimes.

`D` is the data type, which can be anything with a size.

```rust
type CliResult = Result<String,String>;

struct Cli<'a,D> {
    data: D,
    callbacks: HashMap<String, Box<Fn(&mut D,&[&str])->CliResult + 'a>>
}

impl <'a,D: Sized> Cli<'a,D> {
    fn new(data: D) -> Cli<'a,D> {
        Cli{data: data, callbacks: HashMap::new()}
    }

    fn cmd<F>(&mut self, name: &str, callback: F)
    where F: Fn(&mut D, &[&str])->CliResult + 'a {
        self.callbacks.insert(name.to_string(),Box::new(callback));
    }
```

`cmd` is passed a name and any closure that matches our signature, which is boxed
and entered into the map.  `Fn` means that our closures borrow their environment
but can't modify it. It's one of those generic methods where the declaration is scarier than
the actual implementation!  Forgetting the explicit lifetime is a common error
here - Rust won't let us forget that these closures have a lifetime limited to
their environment!

Now for reading and running commands:

```rust
    fn process(&mut self,line: &str) -> CliResult {
        let parts: Vec<_> = line.split_whitespace().collect();
        if parts.len() == 0 { return Ok("".to_string()); }
        match self.callbacks.get(parts[0]) {
            Some(callback) => callback(&mut self.data,&parts[1..]),
            None => Err("no such command".to_string())
        }
    }

    fn go(&mut self) {
        let mut buff = String::new();
        while io::stdin().read_line(&mut buff).expect("error") > 0 {
            {
                let line = buff.trim_left();
                let res = self.process(line);
                println!("{:?}",res);

            }
            buff.clear();
        }
    }

```

This is all reasonably straightforward - split the line into words as a vector,
look up the first word in the map and call the closure with our stored mutable
data and the rest of the words. An empty line is ignored and not considered an error.

Next, let's define some helper functions to make it easier for our closures to
return correct and incorrect results. There's a _little_ bit of cleverness going on;
they are generic functions that work for any type that can be converted to a `String`.

```rust
fn ok<T: ToString>(s: T) -> CliResult {
    Ok(s.to_string())
}

fn err<T: ToString>(s: T) -> CliResult {
    Err(s.to_string())
}
```

So finally, the Main Program - look at how `ok(answer)` works - because
integers know how to convert themselves to strings!

```rust
use std::error::Error;

fn main() {
    println!("Welcome to the Interactive Prompt! ");

    struct Data {
        answer: i32
    }

    let mut cli = Cli::new(Data{answer: 42});

    cli.cmd("go",|data,args| {
        if args.len() == 0 { return err("need 1 argument"); }
        data.answer = match args[0].parse::<i32>() {
            Ok(n) => n,
            Err(e) => return err(e.description())
        };
        println!("got {:?}", args);
        ok(data.answer)
    });

    cli.cmd("show",|data,_| {
        ok(data.answer)
    });

    cli.go();
}
```

The error handling is a bit clunky here, and we'll later see how to use the question
mark operator in cases like this.
Basically, the particular error `std::num::ParseIntError` implements
the trait `std::error::Error`, which we must bring into scope - Rust doesn't let traits
operate unless they're visible.

And in action:

```
Welcome to the Interactive Prompt!
go 32
got ["32"]
Ok("32")
show
Ok("32")
goop one two three
Err("no such command")
go 42 one two three
got ["42", "one", "two", "three"]
Ok("42")
go boo!
Err("invalid digit found in string")
```

Here are some obvious improvements for you to try. First, if we give `cmd` three
arguments with the second being a help line, then we can store these help lines
and automatically implement a 'help' command. Second, having some command editing and
history is _so_ convenient, so use the [rustyline](https://crates.io/crates/rustyline) crate
from Cargo.

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

## Changing the Unchangeable

If you're feeling pig-headed (as I get) you wonder if it's _ever_ possible to get
around the restrictions of the borrow checker.

Consider the following little program, which compiles and runs without problems.

```rust
// cell.rs
use std::cell::Cell;

fn main() {
    let answer = Cell::new(42);

    assert_eq!(answer.get(), 42);

    answer.set(77);

    assert_eq!(answer.get(), 77);
}
```

The answer was changed - and yet the _variable_ `answer` was not mutable!

This is obviously perfectly safe, since the value inside the cell is only accessed
through `set` and `get`.  This goes by the grand name of _interior mutability_. The
usual is called _inherited mutability_: if I have a struct value `v`, then I can only
write to a field `v.a` if `v` itself is writeable. `Cell` values relax this rule, since
we can change the value contained within them with `set` even if the cell itself is
not mutable.

`Cell` would be one way of preserving our original design in the last example, at the
cost of introducing a few `set` and `get` calls.

However, `Cell` only works with values that can be copied, that is, they implement `Copy`,
like primitive types and structs containing them marked as 'derive(Copy)'.

For other values, we have to get a reference we can work on, either mutable or immutable.
This is what `RefCell` provides - you ask it explicitly for a reference to the contained
value:

```rust
// refcell.rs
use std::cell::RefCell;

fn main() {
    let greeting = RefCell::new("hello".to_string());

    assert_eq!(*greeting.borrow(), "hello");
    assert_eq!(greeting.borrow().len(), 5);

    *greeting.borrow_mut() = "hola".to_string();

    assert_eq!(*greeting.borrow(), "hola");
}
```
The explicit dereference operator `*` can be a little bit confusing in Rust, because
often you don't need it - for instance `greeting.borrow().len()` is fine since method
calls will deference implicitly.  But you _do_ need `*` to pull out the underlying
`&String` from `greeting.borrow()` or the `&mut String` from `greeting.mut_borrow()`.

Using a `RefCell` isn't always safe, because any references returned from these
methods must follow the usual rules.


```rust
    let mut gr = greeting.borrow_mut(); // gr is a mutable borrow
    *gr = "hola".to_string();

    assert_eq!(*greeting.borrow(), "hola"); // <== we blow up here!
....
thread 'main' panicked at 'already mutably borrowed: BorrowError'
```

You cannot borrow immutably if you have already borrowed mutably! Except - and this
is important - the violation of the rules happens at _runtime_.  The solution (as always)
is to keep the scope of mutable borrows as limited as possible - in this case, you could
put a block around the first two lines here so that the mutable reference `gr` gets
dropped before we borrow again.

So, this is not a feature you use without good reason, since you will _not_ get a
compile-time error.  These types provide _dynamic borrowing_ in cases where the usual
rules make some things impossible.

## Shared References

Up to now, the relationship between a value and its borrowed references has been clear
and known at compile time.  The value is the owner, and the references cannot outlive it.
But many cases simply don't fit into this neat pattern. For example, say we have
a `Player` struct and a `Role` struct. A `Player` keeps a vector of references to `Role`
objects. There isn't a neat one-to-one relationship between these values, and persuading
`rustc` to cooperate becomes nasty.

`Rc` works like `Box` - heap memory is allocated and the value is moved to it. If you
clone a `Box`, it allocates a full cloned copy of the value.  But cloning an `Rc` is
cheap, because each time you clone it just updates a _reference count_to the _same data_.
This is an old and very popular strategy for memory management,
for example it's used in the Objective C runtime on iOS/MacOS.
(In modern C++, it is implemented with `std::shared_ptr`.)

When a `Rc` is dropped, the reference count is decremented. When that count goes to zero
the owned value is dropped and the memory freed.

```rust
// rc1.rs
use std::rc::Rc;

fn main() {
    let s = "hello dolly".to_string();
    let rs1 = Rc::new(s); // s moves to heap; ref count 1
    let rs2 = rs1.clone(); // ref count 2

    println!("len {}, {}", rs1.len(), rs2.len());
} // both rs1 and rs2 drop, string dies.
```
You may make as many references as you like to the original value - it's _dynamic borrowing_
again. You do not have to carefully track the relationship between the value `T` and
its references `&T`. There is some runtime cost involved, so it isn't the _first_
solution you choose, but it makes patterns of sharing possible which would fall foul
of the borrow checker.  Note that `Rc` gives you immutable shared references, since
otherwise that would break one of the very basic rules of borrowing.
A leopard can't change its spots without ceasing to be a leopard.

In the case of a `Player`, it can now keep its roles as a `Vec<Rc<Role>>` and things
work out fine - we can add or remove roles but not _change_ them after their creation.

However, what if each `Player` needs to keep references to a _team_ as a vector of
`Player` references? Then everything becomes immutable, because all the `Player` values
need to be stored as `Rc`!  This is the place where `RefCell` becomes necessary. The team
may be then defined as `Vec<Rc<RefCell<Player>>>`.  It is now possible to change
a `Player` value using `borrow_mut`, _provided_ no-one has 'checked out' a reference
to a `Player` at the same time. For example, say we have a rule that if something special
happens to a player, then all of their team gets stronger:

```rust
    for p in &self.team {
        p.borrow_mut().make_stronger();
    }
```
So the application code isn't too bad, but the type signatures get a bit scary. You can
always simplify them with a `type` alias:

```rust
type PlayRef = Rc<RefCell<Player>>;
```

## Multithreading

Over the last twenty years, there has been a shift away from raw processing speed
to CPUs having multiple cores. So the only way to get the most out of a modern computer
is to keep all of those cores busy. It's certainly possible to spawn child processes
in the background as we saw with `Command` but there's still a synchronization problem:
we don't know exactly when those children are finished without waiting on them.

There are other reasons for needing separate _threads of execution_, of course. You cannot
afford to lock up your whole process just to wait on blocking i/o, for instance.

Spawning threads is straightforward in Rust - you feed `spawn` a closure which is
executed in the background.

```rust
// thread1.rs
use std::thread;
use std::time;

fn main() {
    thread::spawn(|| println!("hello"));
    thread::spawn(|| println!("dolly"));

    println!("so fine");
    // wait a little bit
    thread::sleep(time::Duration::from_millis(100));
}
// so fine
// hello
// dolly
```

Well obviously just 'wait a little bit' is not a very rigorous solution! It's better
to call `join` on the returned object - then the main thread waits for the
spawned thread to finish.

```rust
// thread2.rs
use std::thread;

fn main() {
    let t = thread::spawn(|| {
        println!("hello");
    });
    println!("wait {:?}", t.join());
}
// hello
// wait Ok(())
```
Here's an interesting variation: force the new thread to panic.

```rust
    let t = thread::spawn(|| {
        println!("hello");
        panic!("I give up!");
    });
    println!("wait {:?}", t.join());
```
We get a panic as expected, but only the panicking thread dies! We still manage
to print out the error message from the `join`. So yes, panics are not always fatal,
but threads are relatively expensive, so this should not be seen as a routine way
of handling panics.

```
hello
thread '<unnamed>' panicked at 'I give up!', thread2.rs:7
note: Run with `RUST_BACKTRACE=1` for a backtrace.
wait Err(Any)
```
It's possible for the thread closure to capture values, but _not_ by borrowing!

```rust
// thread3.rs
use std::thread;

fn main() {
    let name = "dolly".to_string();
    let t = thread::spawn(|| {
        println!("hello {}",name);
    });
    println!("wait {:?}", t.join());
}
```

And here's the helpful error message:

```
error[E0373]: closure may outlive the current function, but it borrows `name`, which is owned by the current function
 --> thread3.rs:6:27
  |
6 |     let t = thread::spawn(|| {
  |                           ^^ may outlive borrowed value `name`
7 |         println!("hello {}",name);
  |                             ---- `name` is borrowed here
  |
help: to force the closure to take ownership of `name` (and any other referenced variables), use the `move` keyword, as shown:
  |     let t = thread::spawn(move || {
```
That's fair enough! Imagine spawning this thread from a function - it will exist
after the function call has finished and `name` gets dropped.  So adding `move` solves our
problem.

The returned objects can be used to keep track of multiple threads:

```rust
// thread4.rs
use std::thread;

fn main() {
    let mut threads = Vec::new();

    for i in 0..5 {
        let t = thread::spawn(move || {
            println!("hello {}",i);
        });
        threads.push(t);
    }

    for t in threads {
        t.join().expect("thread failed");
    }
}
// hello 0
// hello 2
// hello 4
// hello 3
// hello 1

```
Rust insists that we handle the case where the join failed - i.e. that thread panicked.
(You would typically not bail out of the main program when this happens, just note the
error, retry etc)

There is no particular order to thread execution (this program gives different orders
for different runs), and this is key - they really are _independent threads of execution_.
Multithreading is easy; what's hard is _concurrency_ - managing and synchronizing multiple
threads of execution.

There are ways to send data between threads. This
is done in Rust using _channels_. `std::sync::mpsc::channel()` returns a pair:
the _receiver_ channel and the _sender_ channel. Each thread is passed a copy
of the sender with `clone`, and calls `send`. Meanwhile the main thread calls
`recv` on the receiver.

```rust
// thread9.rs
use std::thread;
use std::sync::mpsc;

fn main() {
    let nthreads = 5;
    let (tx, rx) = mpsc::channel();

    for i in 0..nthreads {
        let tx = tx.clone();
        thread::spawn(move || {
            let response = format!("hello {}", i);
            tx.send(response).unwrap();
        });
    }

    for _ in 0..nthreads {
        println!("got {:?}", rx.recv());
    }
}
// got Ok("hello 0")
// got Ok("hello 1")
// got Ok("hello 3")
// got Ok("hello 4")
// got Ok("hello 2")
```

There's no need to join here since the threads send their response just before they
end execution, but obviously this can happen at any time. `recv` will block, and will
return an error if the sender channel is disconnected. `recv_timeout` will only block
for a given time period, and may return a timeout error as well.

`send` never blocks, which is useful because threads can push out data without waiting for the receiver to process.
In addition, the channel is buffered so multiple `send` operations can take place,
which will be received in order.

However, not blocking means that `Ok` does not automatically mean 'successfully delivered message'!

Threads can't share the same environment - by _design_ in Rust. In particular,
they cannot share regular references because the closures move their captured variables.

__shared references_ are fine however - but you cannot use `Rc` for this. This is because
`Rc` is not _thread safe_ - it's optimized to be fast for the non-threaded case. For
threads, you need `std::sync::Arc` - 'Arc' stands for 'Atomic Reference Counting'. That
is, it guarantees that the reference count will be modified in one logical operation. To
make this guarantee, it must ensure that the operation is locked so that only the current
thread has access.

```rust
// thread5.rs
use std::thread;
use std::sync::Arc;

fn main() {
    let mut threads = Vec::new();
    let name = Arc::new("dolly".to_string());

    for i in 0..5 {
        let tname = name.clone();
        let t = thread::spawn(move || {
            println!("hello {} count {}",tname,i);
        });
        threads.push(t);
    }

    for t in threads {
        t.join().expect("thread failed");
    }
}
```

So the shared reference `name` is passed to each new thread by making a new reference
with `clone` and moving it into the closure. It's a little verbose, but this is a safe
pattern. Safety is important in concurrency precisely because the problems are so
unpredictable. A program may run fine on your machine, and occasionally crash on the
server, usually on the weekend. Worse still, the symptoms of such problems are
not easy to diagnose.

Let's look at _synchronization_. `join` is very basic, and merely tells us that a
particular thread has finished. In this case, the second thread only gets going
when the first thread has finished.

```rust
    let one = thread::spawn(move || {
        println!("I am one");
    });

    let two = thread::spawn(move || {
        // wait for one to finish...
        one.join().unwrap();
        // and then we can go
        println!("I am two");
    });
```
(Which _is_ synchronization, just not very interesting.)

Barrier synchronization is a checkpoint where the threads must wait until _all_ of
them have reached that point. Then they can keep going as before. The barrier is
created with the number of threads that we want to wait for. As before we use use `Arc`
to share the barrier with all the threads.

```rust
// thread7.rs
use std::thread;
use std::sync::Arc;
use std::sync::Barrier;

fn main() {
    let nthreads = 5;
    let mut threads = Vec::new();
    let barrier = Arc::new(Barrier::new(nthreads));

    for i in 0..nthreads {
        let barrier = barrier.clone();
        let t = thread::spawn(move || {
            println!("before wait {}", i);
            barrier.wait();
            println!("after wait {}", i);
        });
        threads.push(t);
    }

    for t in threads {
        t.join().unwrap();
    }
}
// before wait 2
// before wait 0
// before wait 1
// before wait 3
// before wait 4
// after wait 4
// after wait 2
// after wait 3
// after wait 0
// after wait 1
```
The threads do their semi-random thing, all meet up, and then continue. It's like a kind
of resumable `join` and useful when you need to farm off pieces of a job to
different threads and want to take some action when all the pieces are finished.

How can threads _modify_ shared state?

Recall the `Rc<RefCell>` strategy for _dynamically_ doing a
mutable borrow on shared references.  The threading equivalent to `RefCell` is
`Mutex` - you may get your mutable reference by calling `lock`. While this reference
exists, no other thread can access it. `mutex` stands for `Mutual Exclusion' - we lock
a section of code so that only one thread can access it, and then unlock it. You get the
lock with the `lock` method, and it is unlocked when the reference is dropped.

```rust
// thread9.rs
use std::thread;
use std::sync::Arc;
use std::sync::Mutex;

fn main() {
    let answer = Arc::new(Mutex::new(42));

    let answer_ref = answer.clone();
    let t = thread::spawn(move || {
        let mut answer = answer_ref.lock().unwrap();
        *answer = 55;
    });

    t.join().unwrap();

    let ar = answer.lock().unwrap();
    assert_eq!(*ar, 55);

}
```
This isn't so straightforward as using `RefCell` because asking for the lock on
the mutex might fail, if another thread has panicked while holding the lock.
(In this case, the documentation actually recommends just exiting the thread with `unwrap`
because things have gone seriously wrong!)

It's even more important to keep this mutable borrow as short as possible, because
as long as the mutex is locked, other threads are _blocked_. This is not the place for
expensive calculations! So typically such code would be used like this:

```rust
// ... do something in the thread
// get a locked reference and use it briefly!
{
    let mut data = data_ref.lock().unwrap();
    // modify data
}
//... continue with the thread
```

## TCP Networking

Rust provides a straightforward interface to the most commonly used network protocol, TCP.
It is very fault-resistant and is the base on which our networked world is built - _packets_ of
data are sent and received, with acknowledgement. By contrast, UDP sends packets out into the wild
without much error correction - there's a joke that goes "I could tell you a joke about UDP but you
might not get it."
(Jokes about networking are only funny for a specialized meaning of the word 'funny')

However, error handling is _very_ important with networking, because anything can happen, and will,
eventually.

TCP works as a client/server model; the server listens on a address and a particular _network port_,
and the client connects to that server. A connection is established and thereafter the client and server
can communicate with a socket.

A simple TCP client in Rust is easy - a `TcpStream` struct is both readable and writeable. As usual, we
have to bring the `Read`, `Write` and other `std::io` traits into scope:

```rust
// client.rs
use std::net::TcpStream;
use std::io::prelude::*;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8000").expect("connection failed");

    write!(stream,"hello from the client!\n").expect("write failed");
 }
```

The server is not much more complicated; we set up a listener and wait for connections. When a
client connects, we get a `TcpStream` on the server side. In this
case, we read everything that the client has written into a string.

```rust
// server.rs
use std::net::TcpListener;
use std::io::prelude::*;

fn main() {

    let listener = TcpListener::bind("127.0.0.1:8000").expect("could not start server");

    // accept connections and get a TcpStream
    for connection in listener.incoming() {
        match connection {
            Ok(mut stream) => {
                let mut text = String::new();
                stream.read_to_string(&mut text).expect("read failed");
                println!("got '{}'",text);
            }
            Err(e) => { println!("connection failed {}",e); }
        }
    }
}
```

Here I've chosen a port number moreorless at random, but [most ports](https://en.wikipedia.org/wiki/List_of_TCP_and_UDP_port_numbers)
are assigned some meaning.

Note that both parties have to agree on a protocol - the client expects it can write
text to the stream, and the server expects to read text from the stream.  If they don't play the same
game, then situations can occur where one party is blocked, waiting for bytes that never come.

Error checking is important - network I/O can fail for many reasons, and errors that might appear
once in a blue moon on a local filesystem can happen on a regular basis.
Someone can trip over the network cable, the other party could crash,  and so forth.
This little server isn't very robust, because it will fall over on the first read error.

Here is a more solid server that handles the error without failing. It also specifically reads a _line_
from the stream, which is done using `io::BufReader` to create an `io::BufRead` on which we can call
`read_line`.

```rust
// server2.rs
use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::io;

fn handle_connection(stream: TcpStream) -> io::Result<()>{
    let mut rdr = io::BufReader::new(stream);
    let mut text = String::new();
    rdr.read_line(&mut text)?;
    println!("got '{}'",text.trim_right());
    Ok(())
}

fn main() {

    let listener = TcpListener::bind("127.0.0.1:8000").expect("could not start server");

    // accept connections and get a TcpStream
    for connection in listener.incoming() {
        match connection {
            Ok(stream) => {
                if let Err(e) = handle_connection(stream) {
                    println!("error {:?}",e);
                }
            }
            Err(e) => { print!("connection failed {}\n",e); }
        }
    }
}
```

`read_line` might fail in `handle_connection`, but the resulting error is safely handled.


One-way communications like this are certainly useful - for instance. a set of services across a
network which want to collect their status reports together in one central place. But it's
reasonable to expect a polite reply, even if just 'ok'!

A simple example is an 'echo' server. The client writes some text ending in a newline to the
server, and receives the same text back with a newline.

```rust
// client_echo.rs
use std::io::prelude::*;
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8000").expect("connection failed");
    let msg = "hello from the client!";

    write!(stream,"{}\n",msg).expect("write failed");

    let mut resp = String::new();
    stream.read_to_string(&mut resp).expect("read failed");
    let text = resp.trim_right();
    assert_eq!(msg,text);
}
```

The server has an interesting twist. Only `handle_connection` changes:

```rust
fn handle_connection(stream: TcpStream) -> io::Result<()>{
    let mut ostream = stream.try_clone()?;
    let mut rdr = io::BufReader::new(stream);
    let mut text = String::new();
    rdr.read_line(&mut text)?;
    ostream.write_all(text.as_bytes())?;
    Ok(())
}
```

This is a common gotcha with simple two-way socket communication; we want to read a line, so
need to feed the readable stream to `BufReader` - but it _consumes_ the stream! So we have to
clone the stream, creating a new struct which refers to the same underlying socket. Then we
have happiness.


