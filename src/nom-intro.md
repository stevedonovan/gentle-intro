## Parsing Text with Nom

[Nom](https://github.com/Geal/nom), [(documented here)](https://docs.rs/nom) is a parser library
for Rust which is well worth the initial time investment.

If you have to parse a known data format, like CSV or JSON, then
it's best to use a specialized library like [Rust CSV](https://github.com/BurntSushi/rust-csv) or
the JSON libraries discussed in [Section 4](4-modules.html#cargo).

Likewise, for configuration files
use dedicated parsers like [ini](https://docs.rs/rust-ini/0.10.0/ini/) or
[toml](http://alexcrichton.com/toml-rs/toml/index.html). (The last one is particularly cool since
it integrates with the Serde framework, just as we saw with [serde_json](https://docs.rs/serde_json).

But if the text is not regular, or some made-up format, then you need to scan that text without
writing a lot of tedious string-processing code. The suggested go-to is often [regex](https://github.com/rust-lang/regex),
but regexes can be frustratingly opaque when sufficiently involved. Nom provides a way to parse
text which is just as powerful and can be built up by combining simpler parsers. And regexes have
their limits, for instance, don't [use regexes for parsing HTML](http://stackoverflow.com/questions/1732348/regex-match-open-tags-except-xhtml-self-contained-tags)
but you _could_ use Nom to parse HTML.  If you ever had the itch to write your own programming
language, Nom is a good place for you start on that hard road to obscurity.

There are some excellent tutorials for learning Nom, but I want to start at the hello-world
level to build some initial familiarity. The basic things you need to know - first, Nom is macros all the
way down, and second, Nom prefers to work with byte slices, not strings. The first means that you have to
be especially careful to get Nom expressions right, because the error messages are not going to be
friendly. And the second means that Nom can be used for _any_ data format, not just text. People
have used Nom to decode binary protocols and file headers. It can also work with 'text'
in encodings other than UTF-8.

Recent versions of Nom work fine with string slices, although you need to use the macros that
end with `_s`.

```rust
#[macro_use]
extern crate nom;

named!(get_greeting<&str,&str>,
    tag_s!("hi")
);

fn main() {
    let res = get_greeting("hi there");
    println!("{:?}",res);
}
// Done(" there", "hi")

```

The `named!` macro creates functions which take some input type (`&[u8]` by default)
and return the second type in angle brackets.
`tag_s!` matches a literal string in the stream of characters, and its value is
a string slice representing that literal.  (If you wanted to work with `&[u8]` then
use the `tag!` macro.)

We call the defined parser `get_greeting` with a `&str` and
get back an [IResult](http://rust.unhandledexpression.com/nom/enum.IResult.html).
And indeed we get back the matching value.

Look at " there" - This is the string slice left over after matching..

We want to ignore whitespace. By just wrapping the `tag!` in `ws!` we can match "hi" anywhere
among spaces, tabs or newlines:

```rust
named!(get_greeting<&str,&str>,
    ws!(tag_s!("hi"))
);

fn main() {
    let res = get_greeting("hi there");
    println!("{:?}",res);
}
// Done("there", "hi")
```

The result is "hi" as before, and the remaining string is "there"! The spaces have been skipped.

"hi" is matching nicely, although this isn't very useful yet.
Let's match _either_ "hi" or "bye". The `alt!` macro ("alternate") takes parser expressions
separated by `|` and matches _any_ of them. Note that you can use whitespace here to make
the parser function easier to read:

```rust
named!(get_greeting<&str>,
    ws!(alt!(tag_s!("hi") | tag_s!("bye")))
);
println!("{:?}", get_greeting(" hi "));
println!("{:?}", get_greeting(" bye "));
println!("{:?}", get_greeting("  hola "));
// Done("", "hi")
// Done("", "bye")
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

Regular expressions are certainly more _compact_!.
We needed to put '()' around the two possibilities
separated by '|' so that we will _capture_ the greeting and nothing else. The first result is the
whole string, the second is the matched capture. ('|' is the so-called 'alternation' operator in
regexes, which is the motivation for the `alt!` macro syntax.)

But this is a very simple regex, and they get complicated very quickly. Being a text mini-language, you
have to escape significant characters like `*` and `(`. If I wanted to match "(hi)" or
"(bye)" the regex becomes "\s*\((hi|bye)\)\s*" but the Nom parser simply becomes
`alt!(tag_s!("(hi)") | tag_s!("(bye)"))`.

It's also a heavy-weight dependency. On this fairly feeble i5 laptop, Nom examples take about 0.55
seconds to compile, which is not much more than "Hello world". But the regex examples take about
0.90s. And the stripped release build executable of the Nom example is about 0.3Mb (which is about
as small as statically linked Rust programs go) versus 0.8Mb for the regex example.

## What a Nom Parser returns

[IResult](http://rust.unhandledexpression.com/nom/enum.IResult.html) has an interesting difference
from the standard `Result` type - there are three possibilities:

  - `Done` - success - you get both the result and the remaining bytes
  - `Error` - failed to parse - you get an error
  - `Imcomplete` - more data needed

We can write a generic `dump` function that handles any return value that can be debug-printed.
This also demonstrates the `to_result` method which returns a regular `Result` - this is probably
the method you will use for most cases since it returns either the returned value or an error.

```rust
#[macro_use]
extern crate nom;
use nom::IResult;
use std::str::from_utf8;
use std::fmt::Debug;

fn dump<T: Debug>(res: IResult<&str,T>) {
    match res {
      IResult::Done(rest, value) => {println!("Done {:?} {:?}",rest,value)},
      IResult::Error(err) => {println!("Err {:?}",err)},
      IResult::Incomplete(needed) => {println!("Needed {:?}",needed)}
    }
}


fn main() {
    named!(get_greeting<&str,&str>,
        ws!(
            alt!( tag_s!("hi") | tag_s!("bye"))
        )
    );

    dump(get_greeting(" hi "));
    dump(get_greeting(" bye hi"));
    dump(get_greeting("  hola "));

    println!("result {:?}", get_greeting(" bye  ").to_result());
}
// Done "" "hi"
// Done "hi" "bye"
// Err Alt
// result Ok("bye")
```

Parsers returning any unparsed text, and being able to indicate that they don't have enough
input characters to decide, is very useful for stream parsing. But usually `to_result` is your friend.

## Combining Parsers

Let's continue the greeting example and imagine that a greeting consists of "hi" or "bye", plus
a name. `nom::alpha` matches a series of alphabetical characters.
The `pair!` macro will collect the result of matching two parsers as a tuple:

```rust
    named!(full_greeting<&str,(&str,&str)>,
        pair!(
            get_greeting,
            nom::alpha
        )
    );

    println!("result {:?}", full_greeting(" hi Bob  ").to_result());
// result Ok(("hi", "Bob"))
```
Now, further imagine that the greeter is perhaps a little shy or doesn't know anybody's name:
let us make the name optional. Naturally, the second value of the tuple becomes an `Option`.

```rust
    named!(full_greeting<&str, (&str,Option<&str>)>,
        pair!(
            get_greeting,
            opt!(nom::alpha)
        )
    );

    println!("result {:?}", full_greeting(" hi Bob  ").to_result());
    println!("result {:?}", full_greeting(" bye ?").to_result());
// result Ok(("hi", Some("Bob")))
// result Ok(("bye", None))
```

Notice that it was straightforward to combine an existing parser for greetings with a parser
that picks up names, and then it was easy to make that name optional. This is the great power of Nom,
and it's why it's called a "parser combinator library".  You can build up your complicated
parsers from simpler parsers, which you can test individually. (At this point, the equivalent
regex is starting to look like a Perl program: regexes do not combine well.)

However, we are not yet home and dry!  `full_greeting(" bye ")` will fail with an
`Imcomplete` error. Nom knows that "bye" may be followed by a name and wants us to give it more
data. This is how a _streaming_ parser needs to work, so you can feed it a file chunk by chunk,
but here we need to tell Nom that the input is complete.

```rust
    named!(full_greeting<&str,(&str,Option<&str>)>,
        pair!(
            get_greeting,
            opt!(complete!(nom::alpha))
        )
    );

    println!("result {:?}", full_greeting(" bye ").to_result());
// result Ok(("bye", None))
```

## Parsing Numbers

Nom provides a function `digit` which matches a series of numerical digits.
So we use `map!`, to convert the string into an integer,
and return the full `Result` type.

```rust
use nom::digit;
use std::str::FromStr;
use std::num::ParseIntError;

named!(int8 <&str, Result<i8,ParseIntError>>,
    map!(digit, FromStr::from_str)
);

named!(int32 <&str, Result<i32,ParseIntError>>,
    map!(digit, FromStr::from_str)
);

println!("{:?}", int8("120"));
println!("{:?}", int8("1200"));
println!("{:?}", int8("x120"));
println!("{:?}", int32("1202"));

// Done("", Ok(120))
// Done("", Err(ParseIntError { kind: Overflow }))
// Error(Digit)
// Done("", Ok(1202))
```

So what we get is a parser `IResult` containing a conversion `Result` - and sure enough, there
is more than one way to fail here. Note that the body of our converting function has exactly
the same code; the actual conversion depends on the return type of the function.

Integers may have a sign. We can capture integers as a pair, where the first
value may be a sign, and the second value would be any digits following.

Consider:

```rust
named!(signed_digits<&str, (Option<&str>,&str)>,
    pair!(
        opt!(alt!(tag_s!("+") | tag_s!("-"))),  // maybe sign?
        digit
    )
);

println!("signed {:?}", signed_digits("4"));
println!("signed {:?}", signed_digits("+12"));
// signed Done("", (None, "4"))
// signed Done("", (Some("+"), "12"))
```

When we aren't interested in the intermediate results, but just want all the matching
input, then `recognize!` is what you need.

```rust
named!(maybe_signed_digits<&str,&str>,
    recognize!(signed_digits)
);

println!("signed {:?}", maybe_signed_digits("+12"));
// signed Done("", "+12")
```

With this technique, we can recognize floating-point numbers. Again we map to string slice
from the byte slice over all these matches. `tuple!` is the generalization of `pair!`,
although we aren't interested in the generated tuple here. `complete!` is needed to resolve
the same problem we had with incomplete greetings - "12" is a valid number without the
optional floating-point part.

```rust
named!(floating_point<&str,&str>,
    recognize!(
        tuple!(
            maybe_signed_digits,
            opt!(complete!(pair!(
                tag_s!("."),
                digit
            ))),
            opt!(complete!(pair!(
                alt!(tag_s!("e") | tag_s!("E")),
                maybe_signed_digits
            )))
        )
    )
);
```

By defining a little helper macro, we get some passing tests. The test
passes if `floating_point` matches all of the string that it is given.

```rust
macro_rules! nom_eq {
    ($p:expr,$e:expr) => (
        assert_eq!($p($e).to_result().unwrap(), $e)
    )
}

nom_eq!(floating_point, "+2343");
nom_eq!(floating_point, "-2343");
nom_eq!(floating_point, "2343");
nom_eq!(floating_point, "2343.23");
nom_eq!(floating_point, "2e20");
nom_eq!(floating_point, "2.0e-6");
```

(Although sometimes macros feel a _little_ dirty, making your tests pretty is a fine thing.)

And then we can parse and convert floating point numbers. Here I'll throw caution to the
winds and throw away the error:

```rust
    named!(float64<f64>,
        map_res!(floating_point, FromStr::from_str)
    );
```

Please note how it's possible to build up complicated parsers step by step, testing each
part in isolation first. That's a strong advantage of parser combinators over regexes.
It is very much the classic programming tactic of divide-and-rule.

## Operations over Multiple Matches

We've met `pairs!` and `tuple!` which capture a fixed number of matches as Rust tuples.

There is also `many0` and `many1` - they both capture indefinite numbers of matches as vectors.
The difference is that the first may capture 'zero or many' and the second 'one or many' (like the
difference between the regex `*` versus `+` modifiers.)  So `many1!(ws!(float64))` would
parse "1 2 3" into  `vec![1.0,2.0,3.0]`, but will fail on the empty string.

`fold_many0` is a _reducing_ operation. The match values are combined into a single value,
using a binary operator.
For instance, this is how Rust people did sums over iterators before `sum` was added; this fold
starts with an initial value (here zero) for the _accumulator_ and keeps adding values to
that accumulator using `+`.

```rust
    let res = [1,2,3].iter().fold(0,|acc,v| acc + v);
    println!("{}",res);
    // 6
```

Here's the Nom equivalent:

```rust
    named!(fold_sum<&str,f64>,
        fold_many1!(
            ws!(float64),
            0.0,
            |acc, v| acc + v
        )
    );

    println!("fold {}", fold_sum("1 2 3").to_result().unwrap());
    //fold 6
```

Up to now, we've had to capture every expression, or just grab all matching bytes with `recognize!`:

```rust
    named!(pointf<(f64,&[u8],f64)>,
        tuple!(
            float64,
            tag_s!(","),
            float64
        )
    );

    println!("got {:?}", nom_res!(pointf,"20,52.2").unwrap());
 //got (20, ",", 52.2)
```

For more complicated expressions, capturing the results of all the parsers leads to
rather untidy types!  We can do better.

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
            tag_s!(",") >>
            second: float64
            >>
            (Point{x: first, y: second})
        )
    );

    println!("got {:?}", nom_res!(pointf,"20,52.2").unwrap());
// got Point { x: 20, y: 52.2 }
```

We're not interested in that tag's value (it can only be a comma) but we assign the two float values
to temporary values which are used to build a struct. The code at the end can be any Rust
expression.

## Parsing Arithmetic Expressions

With the necessary background established, we can do simple arithmetic expressions.
This is a good example of something that really can't be done with regexes.

The idea is to build up expressions from the bottom up. Expressions consist of _terms_, which are
added or subtracted. Terms consist of _factors_, which are multiplied or divided. And (for now)
factors are just floating-point numbers:

```rust
    named!(factor<f64>,
        ws!(float64)
    );

    named!(term<&str,f64>, do_parse!(
        init: factor >>
        res: fold_many0!(
            tuple!(
                alt!(tag_s!("*") | tag_s!("/")),
                factor
            ),
            init,
            |acc, v:(_,f64)| {
                if v.0 == "*" {acc * v.1} else {acc / v.1}
            }
        )
        >> (res)
    ));

    named!(expr<&str,f64>, do_parse!(
        init: term >>
        res: fold_many0!(
            tuple!(
                alt!(tag_s!("+") | tag_s!("-")),
                term
            ),
            init,
            |acc, v:(_,f64)| {
                if v.0 == "+" {acc + v.1} else {acc - v.1}
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
    assert_relative_eq!(fold_sum("1 2 3").to_result().unwrap(), 6.0);
```

Let's define a convenient little testing macro. `stringify!` turns the expression into a string
literal which we can feed into `expr` and then compare the result with how Rust would
evaluate it.

```rust
    macro_rules! expr_eq {
        ($e:expr) => (assert_relative_eq!(
            expr(stringify!($e)).to_result().unwrap(),
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
    named!(factor<&str,f64>,
        alt!(
            ws!(float64) |
            ws!(delimited!( tag_s!("("), expr, tag_s!(")") ))
        )
    );

    expr_eq!(2.2*(1.1 + 4.5)/3.4);
    expr_eq!((1.0 + 2.0)*(3.0 + 4.0*(5.0 + 6.0)));
```

The coolness is that expressions are now defined _recursively_ in terms of expressions!

The particular magic of `delimited!` is that parentheses may be nested - Nom makes sure
the brackets match up.

We are now way past the capabilities of regular expressions, and the stripped executable at 0.5Mb
is still half the size of a "hello world" regex program.







