# Standard Library Containers

## Reading the Documentation

In this section I'll briefly introduce some common parts of the Rust standard
library. The documentation is excellent but a little context and a few examples is
always useful.

Initially, reading the Rust documentation can be challenging, so I'll go through `Vec` as an
example.  A useful tip is to tick the '[-]' box to collapse the docs. (If you download the
standard library source using `rustup component add rust-src` a '[src]' link will appear next to this.)
This gives you a bird's eye view of all the available methods.

The first point to notice is that _not all possible methods_ are defined on `Vec` itself. They are (mostly)
mutable methods that change the vector, e.g. `push`. Some methods are only implemented for vectors where
the type matches some constraint. For example, you can only call `dedup` (remove duplicates) if the
type is indeed something that can be compared for equality.  There are multiple `impl` blocks that
define `Vec` for different type constraints.

Then there's the very special relationship between `Vec<T>` and `&[T]`.  Any method that works on
slices will also directly work on vectors, without explicitly having to use the `as_slice` method.
This relationship is expressed by `Deref<Target=[T]>`. This also kicks in when you pass a vector by
reference to something that expects a slice - this is one of the few places where
a conversion between types happens automatically. So slice methods like `first`, which maybe-returns
a reference to the first element, or `last`, work for vectors as well. Many of the methods are similar
to the corresponding string methods, so there's `split_at` for getting a pair of slices split at an index,
`starts_with` to check whether a vector starts with sequence of values, and `contains` to check whether
a vector contains a particular value.

There's no `search` method for finding the index of a particular value, but here's a rule of thumb:
if you can't find a method on the container, look for a method on the iterator:

```rust
    let v = vec![10,20,30,40,50];
    assert_eq!(v.iter().position(|&i| i == 30).unwrap(), 2);
```

(The `&` is because this is an iterator over _references_ - alternatively you could say `*i == 30` for
the comparison.)

Likewise, there's no `map` method on vectors, because `iter().map(...).collect()` will do the job
just as well. Rust does not like to allocate unnecessarily - often you don't need the result of that `map`
as an actual allocated vector.

So I'd suggest you become familiar with all the iterator methods, because they are crucial to writing
good Rust code without having to write loops out all the time. As always, write little programs to explore
iterator methods, rather than wrestling with them in the context of a more complicated program.

The `Vec<T>` and `&[T]` methods are followed by the common traits: vectors know how to do a debug display of themselves
(but only if the elements implement `Debug`). Likewise, they are clonable if their elements are clonable.
They implement `Drop`, which happens when vectors get to finally die; memory is released,
and all the elements are dropped as well.

The `Extend` trait means values from iterators can be added to a vector without a loop:

```rust
v.extend([60,70,80].iter());
let mut strings = vec!["hello".to_string(), "dolly".to_string()];
strings.extend(["you","are","fine"].iter().map(|s| s.to_string()));
```

There's also `FromIterator`, which lets vectors be _constructed_ from iterators. (The iterator `collect`
method leans on this.)

Any container needs to be iterable as well. Recall that there are [three kinds of iterators](2-structs-enums-lifetimes.html#the-three-kinds-of-iterators)

```rust
for x in v {...} // returns T, consumes v
for x in &v {...} // returns &T
for x in &mut v {...} // returns &mut T
```
The `for` statement relies on the `IntoIterator` trait, and there's indeed three implementations.

Then there is indexing, controlled by `Index` (reading from a vector) and `IndexMut` (modifying a
vector.)  There are many possibilities, because there's slice indexing as well, like `v[0..2]`,
returning these return slices, as well as plain `v[0]` which returns a reference to the first element.

There's a few implementations of the `From` trait. For instance `Vec::from("hello".to_string())`
will give you a vector of the underlying bytes of the string `Vec<u8>`.
Now, there's already a method `into_bytes` on `String`, so why the redundancy?
It seems confusing to have multiple ways of doing the same thing.
But it's needed because explicit traits make generic methods possible.

Sometimes limitations of the Rust type system make things clumsy. An example here is how `PartialEq`
is _separately_ defined for arrays up to size 32!  (This will get better.) This allows the convenience
of directly comparing vectors with arrays, but beware the size limit.

And there are [Hidden Gems](http://xion.io/post/code/rust-iter-patterns.html) buried
deep in the documentation. As Karol Kuczmarski says "Because let’s be honest: no one scrolls that far".
How does one handle errors in an iterator? Say you map over some operation that might
fail and so returns `Result`, and then want to collect the results:

```rust
fn main() {
    let nums = ["5","52","65"];
    let iter = nums.iter().map(|s| s.parse::<i32>());
    let converted: Vec<_> = iter.collect();
    println!("{:?}",converted);
}
//[Ok(5), Ok(52), Ok(65)]
```

Fair enough, but now you have to unwrap these errors - carefully!.
But Rust already knows how to do the Right Thing,
if you ask for the vector to be _contained_ in a `Result` - that is,
either is a vector or an error:

```rust
    let converted: Result<Vec<_>,_> = iter.collect();
//Ok([5, 52, 65])
```

And if there was a bad conversion? Then you would just get `Err` with the first
error encountered. It's a good example of how extremely flexible `collect` is.
(The notation here can be intimidating - `Vec<_>` means "this is a vector, work
out the actual type for me and `Result<Vec<_>,_>` is furthermore asking
Rust to work out the error type as well.)

So there's a _lot_ of detail in the documentation.
But it's certainly clearer than what the C++ docs say about `std::vector`

> The requirements that are imposed on the elements depend on the actual operations performed
> on the container. Generally, it is required that element type is a complete type and meets
> the requirements of Erasable, but many member functions impose stricter requirements.

With C++, you're on your own. The explicitness of Rust is daunting at first, but as you learn to
read the constraints you will know exactly what any particular method of `Vec` requires.

I would suggest that you get the source using `rustup component add rust-src`, since the
standard library source is very readable and the method implementations are usually less scary than the
method declarations.

## Maps

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

`&"zwei"`? This is because `get` returns a _reference_ to the value, not the value
itself. Here the value type is `&str`, so we get a `&&str`. In general it _has_ to be
a reference, because we can't just _move_ a value out of its owning type.

`get_mut` is like `get` but returns a possible mutable reference. Here we have
a map from strings to integers, and wish to update the value for the key 'two':

```rust
let mut map = HashMap::new();
map.insert("one",1);
map.insert("two",2);
map.insert("three",3);

println!("before {}", map.get("two").unwrap());

{
    let mut mref = map.get_mut("two").unwrap();
    *mref = 20;
}

println!("after {}", map.get("two").unwrap());
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
of `a_map["two"]` always returning an integer is that we can't tell the difference
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

An entertaining thing to do with text is count word frequency. It is straightforward
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
more thoroughly.  The total number of unique words (`map.len()`) is 8071.

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

A little surprise - what's that empty word? It is because `split` works on single-character
delimiters, so any punctuation or extra spaces causes a new split.

## Sets

Sets are maps where you care only about the keys, not any associated values.
So `insert` only takes one value, and you use `contains` for testing whether a value
is in a set.

Like all containers, you can create a `HashSet` from an iterator. And this
is exactly what `collect` does, once you have given it the necessary type hint.

```rust
// set1.rs
use std::collections::HashSet;

fn make_set(words: &str) -> HashSet<&str> {
    words.split_whitespace().collect()
}

fn main() {
    let fruit = make_set("apple orange pear orange");

    println!("{:?}", fruit);
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
exists (`Hash`). Remember that there is no _type_ called `Iterator`, so `I` represents
any type that _implements_ `Iterator`.

This technique for implementing our own methods on standard library types may appear
to be a little too powerful, but again, there are Rules. We can only do this for our
own traits. If both the struct and the trait came from the same crate (particularly,
the stdlib) then such implemention would not be allowed. In this way, you are
saved from creating confusion.

Before congratulating ourselves on such a clever, convenient shortcut, you should be
aware of the consequences. If `make_set` was written so, so that these are sets
of owned strings, then the actual type of `intersect` could come as a surprise:

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

## Example: Interactive command processing

It's often useful to have an interactive session with a program. Each line is read in and
split into words; the command is looked up on the first word, and the rest of the words
are passed as an argument to that command.

A natural implementation is a map from command names to closures. But how do we store
closures, given that they will all have different sizes? Boxing them will copy them
onto the heap:

Here's a first try:

```rust
    let mut v = Vec::new();
    v.push(Box::new(|x| x * x));
    v.push(Box::new(|x| x / 2.0));

    for f in v.iter() {
        let res = f(1.0);
        println!("res {}", res);
    }
```

We get a very definite error on the second push:

```
  = note: expected type `[closure@closure4.rs:4:21: 4:28]`
  = note:    found type `[closure@closure4.rs:5:21: 5:28]`
note: no two closures, even if identical, have the same type
```

`rustc` has deduced a type which is too specific, so it's necessary to force that
vector to have the _boxed trait type_ before things just work:

```rust
    let mut v: Vec<Box<Fn(f64)->f64>> = Vec::new();
```
We can now use the same trick and keep these boxed closures in a `HashMap`. We still
have to watch out for lifetimes, since closures can borrow from their environment.

 It's tempting as first to make them `FnMut` - that is, they can modify any captured variables. But we will
have more than one command, each with its own closure, and you cannot then mutably borrow
the same variables.

So the closures are passed a _mutable reference_ as an argument, plus
a slice of string slices (`&[&str]`) representing the command arguments.
They will return some `Result` - We'll use `String` errors at first.

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
        if parts.len() == 0 {
            return Ok("".to_string());
        }
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
                println!("{:?}", res);

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
So finally, the Main Program. Look at how `ok(answer)` works - because
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
the trait `std::error::Error`, which we must bring into scope to use the `description`
method - Rust doesn't let traits operate unless they're visible.

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
history is _very_ convenient, so use the [rustyline](https://crates.io/crates/rustyline) crate
from Cargo.

