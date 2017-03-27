## Parsing Text with Nom

[nom](https://github.com/Geal/nom) is a parser library for Rust which is well worth
the initial time investment. If you have to parse a known data format, like CSV or JSON, then
it's best to use a specialized library like [Rust CSV](https://github.com/BurntSushi/rust-csv) or
the JSON libraries discussed in [Section 3)(?).  Likewise, for configuration files
use dedicated parsers like [ini](https://docs.rs/rust-ini/0.10.0/ini/) or
[toml](http://alexcrichton.com/toml-rs/toml/index.html). (The last one is particularly cool since
it integrates with the Serde framework, just as we saw with __serde-json__).

But if the text is not regular, or some made-up format, then you need to scan that text without
writing a lot of tedious string-processing code. The suggested go-to is often [regex](?), but
regexes can be frustratingly opaque when sufficiently involved. Nom provides a way to parse
text which is just as powerful and can be built up by combining simpler parsers. And regexes have
their limits, for instance, don't [use regexes for parsing HTML](http://stackoverflow.com/questions/1732348/regex-match-open-tags-except-xhtml-self-contained-tags)
but you _could_ use Nom to parse HTML.  If you ever had the itch to write your own programming
language, Nom is a good place for you start on that hard road to obscurity.

There are some [excellent tutorials](?) for learning Nom, but I want to start at the hello-world
level to build some initial familiarity. The basic things you need to know - first, Nom is macros all the
way down, and second, Nom works with byte slices, not strings. The first means that you have to
be especially careful to get Nom expressions right, because the error messages are not going to be
friendly. And the second means that Nom can be used for _any_ data format, not just text. People
have used Nom to decode binary protocols and file headers.  There will be a little friction, because
you feed a Nom parser byte slices and the basic parsers return byte slices, which need to be converted.

```rust
#[macro_use]
extern crate nom;

named!(get_greeting<&[u8]>,
    tag!("hi")
);

fn main() {
    let res = get_greeting("hi ".as_bytes());
    println!("{:?}",res);
}
// Done([32], [104, 105])
```

The `named!` macro creates functions which take byte slices (`&[u8]`) and return the type in
angle brackets.  `tag!` matches a literal string [ASCII?] in that stream of bytes, and its value is
a byte slice representing that literal.

We call the defined parser `get_greeting` with a `&str` converted to '&[u8]`, and
get back an [IResult](http://rust.unhandledexpression.com/nom/enum.IResult.html).
And indeed that `[104, 105]` is "hi" in ASCII!

Look at `[32]` - that's a space in ASCII. This represents the _remainder_ of the bytes left
over from the scan.

Now we want to ignore whitespace. By just wrapping the `tag!` in `ws!` we can match "hi" anywhere
among spaces, tabs or newlines:

```rust
    named!(get_greeting<&[u8]>,
        ws!(tag!("hi"))
    );
    let res = get_greeting(" hi  hi".as_bytes());
    println!("{:?}",res);
// Done([104, 105], [104, 105])
```

The result is "hi" as before, and the remaining bytes are also "hi"! The spaces have been skipped.

It's irritating to work in bytes when we know for a fact that they are valid UTF-8, since this
is what we fed the parser.  [from_utf8](?) will convert bytes to a string, but may of course fail.
The macro `map_res!` applies a function to something,  returning a `Result`, and unwraps the value.
We redeclare `get_greeting` to reflect the new string return type.

```rust
    use std::str::from_utf8;
    named!(get_greeting<&str>,
        ws!(map_res!(tag!("hi"),from_utf8))
    );
    let res = get_greeting(" hi  hi".as_bytes());
    println!("{:?}",res);
// Done([104, 105], "hi")
```

By now you should be suspicious of anything that can throw away errors (`map_res!` does not
panic) but this seems safe for now: flag it mentally with a warning.

"hi" is now rendered nicely as a string, although it isn't telling us anything new.
Let's match _either_ "hi" or "bye". The `alt!` macro ("alternate") takes parser expressions
separated by `|` and matches _any_ of them. Note that you can use whitespace here to make
the parser function easier to read:

```rust
    named!(get_greeting<&str>,
        ws!(map_res!(
            alt!( tag!("hi") | tag!("bye"))
        ,from_utf8))
    );
    println!("{:?}", get_greeting(" hi ".as_bytes()));
    println!("{:?}", get_greeting(" bye ".as_bytes()));
    println!("{:?}", get_greeting("  hola ".as_bytes()));
// Done([], "hi")
// Done([], "bye")
// Error(Alt)
```

The last match failed because there is no alternative that matches "hola".

Clearly we need to understand this `IResult` type to go further, but first let's compare this
with the regex solution:

```rust
    let greetings = Regex::new(r"\s*(hi|bye)\s*").expect("bad regex");
    let caps = greetings.captures(" hi ").expect("match failed");
    println!("{:?}",caps);
// Captures({0: Some(" hi "), 1: Some("hi")})
```

Regular expressions are certainly more _compact_ and there is no bytes vs strings conversion needed.
We needed to put '()' around the two possibilities
separated by '|' so that we will _capture_ the greeting and nothing else. The first result is the
whole string, the second is the matched capture. ('|' is the so-called 'alternation' operator in
regexes, which is the motivation for the `alt!` macro syntax.)

But this is a very simple regex, and they get complicated fast. Being a text mini-language, you
have to escape significant characters like `*` and `(`. If I wanted to match "(hi)" or
"(bye)" the regex becomes "\s*\((hi|bye)\)\s*" but the Nom parser simply becomes
`alt!(tag!("(hi)") | tag!("(bye)"))`.

It's also a heavy-weight dependency. On this fairly feeble i5 laptop, Nom examples take about 0.55
seconds to compile, which is not much more than "Hello world". But the regex examples take about
0.90s. And the stripped executable of the Nom example is about 0.3Mb; the regex example 2.0Mb.

## What a Nom Parser returns

[IResult](http://rust.unhandledexpression.com/nom/enum.IResult.html) has an interesting difference
from the standard `Result` type. There are three possibilities:

  # `Done` - success - you get both the result and the remaining bytes
  # `Error` - failed to parse - you get an error
  # `Imcomplete` - more data needed

We can write a generic `dump` function that handles any return value that can be debug-printed.
This also demonstrates the `to_result` method which returns a regular `Result` - this is probably
the method you will use for most cases since it returns either the returned value or an error.

```rust
#[macro_use]
extern crate nom;
use nom::IResult;
use std::str::from_utf8;
use std::fmt::Debug;

fn dump<T: Debug>(res: IResult<&[u8],T>) {
    match res {
      IResult::Done(bytes, value) => {println!("Done {:?} {:?}",from_utf8(bytes),value)},
      IResult::Error(err) => {println!("Err {:?}",err)},
      IResult::Incomplete(needed) => {println!("Needed {:?}",needed)}
    }
}


fn main() {
    named!(get_greeting<&str>,
        ws!(map_res!(
            alt!( tag!("hi") | tag!("bye"))
        ,from_utf8))
    );

    dump(get_greeting(" hi ".as_bytes()));
    dump(get_greeting(" bye hi".as_bytes()));
    dump(get_greeting("  hola ".as_bytes()));

    println!("result {:?}", get_greeting(" bye  ".as_bytes()).to_result());
}
// Done Ok("") "hi"
// Done Ok("hi") "bye"
// Err Alt
// result Ok("bye")
```

Parsers returning any unparsed bytes, and being able to indicate that they don't have enough
input bytes to decide, is very useful for stream parsing. But usually `to_result` is your friend.

And hey, while we're going macro-crazy here, can always write a helper macro:

```rust
macro_rules! nom_res {
    ($p:expr,$t:expr) => ($p($t.as_bytes()).to_result())
}

println!("result {:?}", nom_res!(get_greeting, " bye  "));
```

## Combining Parsers

Let's continue the greeting example and imagine that a greeting consists of "hi" or "bye", plus
a name. `nom::alpha` matches a series of alphabetical ASCII bytes, which we can convert safely
to a string using `from_utf8`.  The `pair!` macro will collect the result of matching two
parsers as a tuple:

```rust
    named!(full_greeting<(&str,&str)>,
        pair!(
            get_greeting,
            map_res!(nom::alpha,from_utf8)
        )
    );

    println!("result {:?}", nom_res!(full_greeting, " hi Bob  "));
// result Ok(("hi", "Bob"))
```
Now, further imagine that the greeter is perhaps a little shy or doesn't know anybody's name:
let us make the name optional. Naturally, the second value of the tuple becomes an `Option`.

```rust
    named!(full_greeting<(&str,Option<&str>)>,
        pair!(
            get_greeting,
            opt!(map_res!(nom::alpha,from_utf8))
        )
    );

    println!("result {:?}", nom_res!(full_greeting, " hi Bob  "));
    println!("result {:?}", nom_res!(full_greeting, " bye ?"));
// result Ok(("hi", Some("Bob")))
// result Ok(("bye", None))
```

Notice that it was straightforward to combine an existing parser for greetings with a parser
that picks up names, and then it was easy to make that name optional. This is the great power of Nom,
and it's why it's called a "parser combinator library".  You can build up your complicated
parsers from simpler parsers, which you can test individually. (At this point, the equivalent
regex is starting to look like a Perl program: regexes do not combine well.)

However, we are not yet home and dry!  `nom_res!(full_greeting, " bye ")` will fail with an
`Imcomplete` error. Nom knows that "bye" may be followed by a name and wants us to give it more
data. This is how a _streaming_ parser needs to work, so you can feed it a file chunk by chunk,
but here we need to tell Nom that the input is complete.

```rust
    named!(full_greeting<(&str,Option<&str>)>,
        pair!(
            get_greeting,
            opt!(complete!(map_res!(alpha,from_utf8)))
        )
    );

    println!("result {:?}", nom_res!(full_greeting, " bye "));
// result Ok(("bye", None))
```

## Parsing Numbers

Nom provides a function `digit` which matches a series of numerical digits. We do the usual
`map_res!` to get a `&str`, and then can convert this to the particular integer type using the
`FromStr` trait. However, using `map_res!` here is definitely bad - we could overflow our integer.
So use `map!` and return the full `Result` type:

```rust
   use nom::digit;
   use std::str::FromStr;

    type IntResult<N> = Result<N,std::num::ParseIntError>;

    named!(digits<&str>,
        map_res!(digit,from_utf8)
    );

    named!(int8< IntResult<i8> >,
        map!(digits, FromStr::from_str)
    );

    println!("int {:?}", nom_res!(int8, "120"));
    println!("int {:?}", nom_res!(int8, "1200"));
    println!("int {:?}", nom_res!(int8, "x120"));

//int Ok(Ok(120))
//int Ok(Err(ParseIntError { kind: Overflow }))
//int Err(Digit)
```

So what we get is a parser `Result` containing a conversion `Result` - and sure enough, there
is more than one way to fail here. I've spread out the definition since you know have something
reuseable for other integer types, and everything gets inlined in a release build anyway.

Integers may have a sign. This is a case where you don't want the values of the individual matches!

Consider:

```rust
    named!(signed_digits<(Option<&[u8]>,&[u8])>,
        pair!(
            opt!(alt!(tag!("+") | tag!("-"))),  // maybe sign?
            digits
        )
    );

    println!("signed {:?}", nom_res!(signed_digits, "4"));
    println!("signed {:?}", nom_res!(signed_digits, "+12"));
//signed Ok((None, [52]))
//signed Ok((Some([43]), [49, 50]))
```

When we aren't interested in the intermediate results, but just want all the matching
input, then `recognize!` is what you need.

```rust
    named!(maybe_signed_digits<&[u8]>,
        recognize!(signed_digits)
    );

    println!("signed {:?}", nom_res!(maybe_signed_digits, "+12"));

//signed Ok([43, 49, 50])
```

With this technique, we can recognize floating-point numbers. Again we map to string slice
from the byte slice over all these matches. `tuple!` is the generalization of `pair!`,
although we aren't interested in the generated tuple here. `complete!` is needed to resolve
the same problem we had with incomplete greetings - "12" is a valid floating point number.

```rust
    named!(floating_point<&str>,
        map_res!(recognize!(
            tuple!(
                maybe_signed_digits,
                opt!(complete!(pair!(
                    tag!("."),
                    digits
                ))),
                opt!(complete!(pair!(
                    alt!(tag!("e") | tag!("E")),
                    maybe_signed_digits
                )))
            )
        ),from_utf8)
    );
```

By defining a little helper macro, we get some passing tests:

```rust
    macro_rules! nom_eq {
        ($p:expr,$e:expr) => (assert_eq!(nom_res!($p, $e).unwrap(),$e))
    }

    nom_eq!(floating_point, "+2343");
    nom_eq!(floating_point, "-2343");
    nom_eq!(floating_point, "2343");
    nom_eq!(floating_point, "2343.23");
    nom_eq!(floating_point, "2e20");
    nom_eq!(floating_point, "2.0e-6");
```

Although sometimes macros feel a little dirty, making your tests pretty is a fine thing.

And then we can parse and convert floating point numbers. Here I'll throw caution to the
winds and throw away the error:

```rust
    named!(float64<f64>,
        map_res!(floating_point, FromStr::from_str)
    );
```

Please note how it's possible to build up pretty complicated parsers step by step, testing each
little part first.  That's a strong advantage of parser combinators over regexes.


This is a nice example of something that really can't be done with regexes (together with the
obvious hilarious example of HTML).

## Operations over Multiple Matches

We've met `pairs!` and `tuple!` which capture a fixed number of matches as Rust tuples.

There is also `many0` and `many1` - they both capture indefinite numbers of matches as vectors.
The difference is that the first may capture 'zero or many' and the second 'one or many' (like the
difference between the regex `*` versus `+` modifiers.)

`fold_many0` is a _reducing_ operation. The match values are combined into a single value.
 For instance, this is how Rust people did sums over iterators before `sum` was added:

```rust
    let res = [1,2,3].iter().fold(0,|acc,v| acc + v);
    println!("{}",res);
    // 6
```

Here's the Nom equivalent:

```rust
    named!(fold_sum<f64>,
        fold_many1!(
            ws!(float64),
            0.0,
            |acc, v| acc + v
        )
    );

    println!("fold {}", nom_res!(fold_sum,"1 2 3").unwrap());
    //fold 6
```

Up to now, we've had to capture every expression, or just grab all matching bytes with `recognize!`:

```rust
    named!(pointf<(f64,&[u8],f64)>,
        tuple!(
            float64,
            tag!(","),
            float64
        )
    );

    println!("got {:?}", nom_res!(pointf,"20,52.2").unwrap());
 //got (20, [44], 52.2)
```

For more complicated expressions, capturing the results of all the parsers leads to
rather untidy types!

`do_parse!` lets you extract only the values you're interested in. The matches are separated
with `>>` - the matches of interest are of the form `name: parser`. Finally, there's a code
block in parentheses.

```rust
    #[derive(Debug)]
    struct Point {
        x: f64,
        y: f64
    }

    named!(pointf<Point>,
        do_parse!(
            first: float64 >>
            tag!(",") >>
            second: float64
            >>
            (Point{x: first, y: second})
        )
    );

    println!("got {:?}", nom_res!(pointf,"20,52.2").unwrap());
// got Point { x: 20, y: 52.2 }
```

We're not interested in that tag (it can only be a comma) but we assign the two float values
to temporary values which are used to build a struct. The code at the end can be any Rust
expression.


## Parsing Arithmetic Expressions

With the necessary background established, I can explain the example on the front page of
the Nom documentation:

The idea is to build up expressions from the bottom up. Expressions consist of _terms_, which are
added or subtracted. Terms consist of _factors_, which are multiplied or divided. And (for now)
factors are just floating-point numbers:

```rust
    named!(factor<f64>, ws!(float64));

    named!(term<f64>, do_parse!(
        init: factor >>
        res: fold_many0!(
            tuple!(
                alt!(tag!("*") | tag!("/")),
                factor
            ),
            init,
            |acc, v:(_,f64)| {
                if v.0 == b"*" {acc * v.1} else {acc / v.1}
            }
        )
        >> (res)
    ));

    named!(expr<f64>, do_parse!(
        init: term >>
        res: fold_many0!(
            tuple!(
                alt!(tag!("+") | tag!("-")),
                term
            ),
            init,
            |acc, v:(_,f64)| {
                if v.0 == b"+" {acc + v.1} else {acc - v.1}
            }
        )
        >> (res)
    ));
```

This expresses our definitions more precisely - an expression consists of at least one term, and then
zero or many plus-or-minus terms. We don't collect them, but _fold_ them using the appropriate
operator. (It's one of those cases where Rust can't quite work out the type of the expression, so
we need a type hint.)  Doing it like this establishes the correct _operator precedence_ - `*` always
wins over `+` and so forth.

We're going to need floating-point asserts here, and [there's a crate for that](http://brendanzab.github.io/approx/approx/index.html).

Add the line 'approx="0.1.1" to your Cargo.toml, and away we go:

```rust
#[macro_use]
extern crate approx;
...
    assert_relative_eq!(nom_res!(fold_sum,"1 2 3").unwrap(), 6.0);
```

Let's define a convenient little testing macro - `stringify!` turns the expression into a string
literal which we can feed into `nom_res!` and then compare the result with how Rust would
evaluate it.

```rust
    macro_rules! expr_eq {
        ($e:expr) => (assert_relative_eq!(
            nom_res!(expr,stringify!($e)).unwrap(),
            $e)
        )
    }

    expr_eq!(2.3);
    expr_eq!(2.0 + 3.0 - 4.0);
    expr_eq!(2.0*3.0 - 4.0);
```

This is very cool - a few lines to get an expression evaluator! But it gets better.
We add an alternative to numbers in the `factor` parser - expressions contained inside
parentheses:

```rust
    named!(factor<f64>,
        alt!(
            ws!(float64) |
            ws!(delimited!( tag!("("), expr, tag!(")") ))
        )
    );

    expr_eq!(2.2*(1.1 + 4.5)/3.4);
    expr_eq!((1.0 + 2.0)*(3.0 + 4.0*(5.0 + 6.0)));
```

The coolness is that expressions are now defined _recursively_ in terms of expressions!

The particular magic of `delimited!` is that parentheses may be nested - Nom makes sure
the brackets match up.

We are now way past the capabilities of regular expressions, and the stripped executable at 0.5Mb
is still a quarter of the size of a "hello world" regex program.







