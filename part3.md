## Example: S-Exprs

The idea: we want a `Value` which can _itself_ contain a vector of `Value`
types. This is not a problem, because `Vec` is like a generalized form of `Box` - it has
a known size.

```Rust
// enum4.rs

#[derive(Debug)]
enum Value {
    Number(f64),
    Str(String),
    Bool(bool),
    Arr(Vec<Value>)
}

fn main() {
    use Value::*;

    let s = "hello".to_string();
    let v = vec![Number(1.0),Bool(false),Str(s)];
    let arr = Arr(v);

    let res = Arr(vec![Number(2.0),arr]);

    println!("{:?}",res);
}
// Arr([Number(2), Arr([Number(1), Bool(false), Str("hello")])])
```

This is cool - we have a _strongly-typed_ dynamic data structure which can be
extended indefinitely.

However, that output format isn't ideal. Let's define how these values print themselves
out with `Display` - it's very much like `Debug`, except it works for '{}', not '{:?}'.

```rust
use std::fmt;

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Value::*;
        match *self {
            Number(n) => write!(f,"{} ",n),
            Str(ref s) => write!(f,"{} ",s),
            Bool(b) => write!(f,"{} ",b),
            Arr(ref arr) => {
                write!(f,"(")?;
                for v in arr.iter() {
                    v.fmt(f)?;
                }
                write!(f,")")
            }
        }
    }
}
```

Note the return value - `fmt` returns a `Result` because `write!` may fail. So we
carefully make sure we check the result of every `write!` call.

This is a classic recursive function -
when the `Value` is actually a vector of `Value`, we call
`fmt` on each of those values. The question mark operator is used to return
results immediately if they represent failure (`Err`).

The result of `println!("{}",res)` is now more compact and readable:

```
(2 (1 false hello ))
```

This is a famous old-fashioned format called a 's-expr', used by Lisp.

Constructing these structures manually is painful. This is the point where people
often write macros to make life easier - and this is a _respected_ choice in Rust,
unlike C++.

But it will be more interesting (and useful) to implement a _builder_ instead.

It will be a struct containg a vector of `Value`; little methods will add the various
values to that vector. (There's also a _stack_ which we will come to just now.)

```rust
struct Builder {
    stack: Vec<Vec<Value>>,
    current: Vec<Value>
}

impl Builder {
    fn new() -> Builder {
        Builder {
            stack: Vec::new(),
            current: Vec::new()
        }
    }


    fn push(&mut self, v: Value) -> &mut Builder {
        self.current.push(v);
        self
    }

    fn s(&mut self, s: &str) -> &mut Builder {
        self.push(Value::Str(s.to_string()))
    }

    fn b(&mut self, v: bool) -> &mut Builder {
        self.push(Value::Bool(v))
    }

    fn n(&mut self, v: f64) -> &mut Builder {
        self.push(Value::Number(v))
    }
    ...
}
```

I've factored out a `push` method which actually appends each new value to the vector -
then the little methods are easy to write.  Note that they always return `self` -
so we can _chain_ the methods together.

```rust
let mut b = Builder::new();
b.s("hello").s("dolly").n(42.0);
```

Now, consider the problem of how to construct a `Value`. We already have a `Vec<Value>`
and just need to wrap it up as a `Value::Arr`.  But if we try to do
`Value::Arr(self.current)` then there's the usual complaint: "cannot move out of
borrowed context".  Well, it turns out you can _swap_ it out. A Rust owner type is like
a small child clutching a treasure. You can placate the child by giving it the
_exact same kind of treasure_ while taking away the original. This may not be good
child psychology, but Rust considers it an honourable deal. So `extract_current()` will _take_
the vector of values out of `self.current` and leave in its place a vector we provide:

```rust
    ...
    fn extract_current(&mut self, arr: Vec<Value>) -> Vec<Value> {
        let mut current = arr;
        std::mem::swap(&mut current, &mut self.current);
        current
    }

    fn value(&mut self) -> Value {
        Value::Arr(self.extract_current(Vec::new()))
    }

```

The idea is to mosey along, appending values to `current` with `s` etc, until we
call `open`, where we save the current vector on the stack and start collecting values
in a new vector. On `close`, a `Value` is created from the vector and we restore the
old vector from the stack.

```rust

    fn open(&mut self) -> &mut Builder {
        let current = self.extract_current(Vec::new());
        self.stack.push(current);
        self
    }

    fn close(&mut self) -> &mut Builder {
        let last_current = self.stack.pop().expect("stack empty");
        let current = self.extract_current(last_current);
        self.current.push(Value::Arr(current));
        self
    }
    ...

    let res = Builder::new().open()
    .s("one")
    .open()
        .s("two")
        .b(true)
        .open()
          .s("four")
          .n(1.0)
        .close()
    .close().close().value();

    println!("{:?}",res);
    println!("{}",res);
    // Arr([Arr([Str("one"), Arr([Str("two"), Bool(true), Arr([Str("four"), Number(1)])])])])
    // ((one (two true (four 1 ))))
```

This is the 'Builder pattern' - create a helper type which is built up with a chain
of methods, which finally constructs the desired value. (There's a lot of mutation
taking place in that `Builder` but it's all nicely contained and temporary.)

There's a serious weakness in this implementation - in `close` we expect the stack to
be non-empty, and otherwise basically just crash. It's too easy to do this, so we have
to think about proper error handling.  One way to peel the advocado is to keep an
`Option<String>` representing the error, which is initially `None`. It will be set to
`Some` if something bad happens - after that, do nothing but pass along `self`!
And `value` will then return `Result<Value,String>`. This pass-the-handgrenade-along
tactic works well with builders.

The other issue is that the extra open/close operations cause the list to be put inside
a list; there's a reason for not wanting this behaviour.

See `sexpr.rs` for the security-enhanced, bullet-proof version.

It would be cool to _parse_ such s-exprs. That is, given a string containing an s-expr,
build up a `Value` structure using a `Builder`. They are particularly straightforward
to parse - we look at each character in the string.
If the char is 'whitespace' (spaces, tabs, newlines) then see if we have collected
any chars, and process that string. Otherwise, '(' means `open` and ')' means `close`,
and anything else goes into the string. We also check after ')' to see if a word
has been collected.

```rust
fn parse(text: &str) -> Result<Value,String> {
    let mut builder = Builder::new();
    let mut word = String::new();
    for ch in text.chars() {
        if ch.is_whitespace() {
            if word.len() > 0 {
                parse_word(&mut builder, &word)?;
                word.clear();
            }
        } else
        if ch == '(' {
            builder.open();
        } else
        if ch == ')' {
            if word.len() > 0 {
                parse_word(&mut builder, &word)?;
                word.clear();
            }
            builder.close();
        } else {
            word.push(ch);
        }
    }
    builder.value()
}

use std::error::Error;

fn parse_word(builder: &mut Builder, word: &str) -> Result<(),String> {
    // guaranteed to be at least one character!
    let first = word.chars().next().unwrap();
    if word == "T" || word == "F" {
        builder.b(word == "T");
    } else
    if first.is_digit(10) || first == '-' {
        match word.parse::<f64>() {
        Ok(num) => builder.n(num),
        Err(err) => return Err(err.description().to_string())
        };
    } else {
        builder.s(&word);
    }
    Ok(())
}
```
That `word.parse::<f64>()` looks a little ugly. We previously had:

```rust
let x: f64 = word.parse();
// alternatively
let x = word.parse::<f64>();
```

The `::<>` is affectionately known as the 'turbofish operator'. This code could be
more elegantly expressed with `?` but Rust error types don't convert to strings
directly. _Proper_ Rust programs define error types, but that's for another day.

`parse_word` might fail if any numbers are bad, so it must return an error. We don't
care about the `Ok` value, so just use the empty type. Since it can return an error,
there's a question-mark operator wherever it's called. `parse` otherwise just returns
the value from the `Builder`.

I won't pretend this is _easy_, but reading non-trivial code is the best way to learn.
The strategy is to separate out different things, so that the builder builds, the
parser parses. Mixing these all up is not a good idea, it will lead to confusing code
for anybody that follows you (and in two months, you are that person.) Clever code
is rarely a good idea in itself.

Note that doing this kind of thing properly is a _complete pain_ in C. It's
easier in Python, although the error handling would be different. But it would be
slower, and the dynamic typing will create difficulties for working with larger
programs.  The really cool thing about Rust is that if you follow the rules (and the
compiler doesn't let you bend them arbitrarily) your programs will not leak memory.
They will use a lot less memory than the equivalent managed equivalent in Java or Go,
and they will approach C in speed.

## Errors (Done Right)

Now `Result<T,String>` is a fine way to do simple error returns but `String` is an
awkward error type. First, it is too general (you can also just throw strings in C++ but then
people look at you funny), and second, it does not play nice with other error types.
In particular, the useful `?` operator cannot be used with it. It relies on `std::convert::From`
to convert error types into your error type, and we cannot implement this for `String` -
because both the trait and the type are defined elsewhere.

So, let's derive a custom error type. It must implement

  - `std::error::Error`
  - `Display` (to print itself out nicely)
  - `Debug` (to print out its details)

```rust
use std::error::Error
use std::fmt;

#[derive(Debug)]
pub struct SexprError {
    details: String
}

impl fmt::Display for SexprError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}


impl Error for SexprError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl SexprError {
    pub fn new(msg: &str) -> SexprError {
        SexprError{details: msg.to_string()}
    }

    pub fn err<T>(msg: String) -> Result<T,SexprError> {
        Err(SexprError { details: msg })
    }
}
```

These are straightforward traits, and it's a straightforward struct.
I'll get to that interesting `err` method just now.

Now let's look at the interesting task of evaluating s-exprs like `(+ 10 (* 1.5 3) 20)`.
Operators like `+` and `*` work on arbitrary numbers of values, `-` and `/` work on
pairs.

So the job is to look at `Value`'s that look like `(<str> x1 x2 ...)`.

```rust
fn eval(v: &Value) -> Result<f64,SexprError> {
    match *v {
        Value::Arr(ref arr) if arr.len() > 2 => eval_op(&arr),
        Value::Number(x) => Ok(x),
        ref v => SexprError::err(format!("cannot convert {:?} to number", v))
    }
}
```

Inside the match, we're interested in numbers and lists, anything else must
be an error. `ref v` matches _anything_ and binds `v` to a reference to that `Value`.
The `if` clause ensures that the lists are long enough.

`eval_op` is just looking for lists that start with a string:


```rust
fn eval_op(arr: &[Value]) -> Result<f64,SexprError> {
    match arr[0] {
        Value::Str(ref s) => {
            if s == "+" || s == "*" {
                let adding = s == "+";
                let mut res = if adding {0.0} else {1.0};
                for v in &arr[1..] {
                    let num = eval(v)?;
                    res = if adding {
                        res + num
                    } else {
                        res * num
                    }
                }
                Ok(res)
            } else
            if s == "-" || s == "/" {
                let x = eval(&arr[1])?;
                let y = eval(&arr[2])?;
                let res = if s == "-" {
                    x - y
                } else {
                    x / y
                };
                Ok(res)
            } else {
                SexprError::err(format!("unknown operator {:?}", s))
            }
        },
        ref v => SexprError::err(format!("operator must be string {:?}", v))
    }
}
```

Now we have an operator string, and do the two kinds of operators. We go through the
_rest_ of that vector (expressed by the slice `&arr[1..]`) and call `eval` on those
values. If they fail, `?` will return the error; otherwise do the product or the sum.
(It's possible to do this with the iterator `product` and `sum` methods but _handling
the error_ is seriously awkward, so let's loop like barbarians.)

`SexprError::err` returns a `Result<T,SexprError>` and Rust can work out what `T` must
be in this case. It saves us from this ugliness:

```rust
Err(SexprError::new(&format!("operator must be string {:?}", v)))
```

The rest of `sexpr.rs` starts looking better. Consider `parse_word`:

```rust
fn parse_word(builder: &mut Builder, word: &str) -> Result<(),SexprError> {
    // guaranteed to be at least one character!
    let first = word.chars().next().unwrap();
    if word == "T" || word == "F" {
        builder.b(word == "T");
    } else
    if first.is_digit(10) || first == '-' {
        let num: f64 = word.parse()?;
        builder.n(num);
    } else {
        builder.s(&word);
    }
    Ok(())
}
```

Yay, question-mark has replaced an ugly `match`!  But for this to work, `SexprError`
must implement a trait that will convert the error of `parse` into itself:

```rust
impl From<std::num::ParseFloatError> for SexprError {
    fn from(err: std::num::ParseFloatError) -> Self {
        SexprError::new(err.description())
    }
}
```







