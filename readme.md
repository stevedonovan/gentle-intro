# A Gentle Introduction To Rust

![Rust](PPrustS.png)
[thanks to David Marino](http://leftoversalad.com/c/015_programmingpeople/)

## Why learn a new Programming Language?

The aim of this tutorial is to take you to a place where you can read and write
enough Rust to fully appreciate the excellent learning resources available
online, in particular [The Book](https://doc.rust-lang.org/stable/book/).
It's an opportunity to _try before you buy_, and get enough feeling for the
power of the language to want to go deeper.

As Einstein might have said, "As gentle as possible, but no gentler.". There is a
lot of new stuff to learn here, and it's different enough to require some
rearrangement of your mental furniture. By 'gentle' I mean that the features are
presented practically with examples; as we encounter difficulties, I hope to
show how Rust solves these problems. It is important to understand the problems before
the solutions make sense. To put it in flowery language, we are going for a hike
in hilly country and I will point out some interesting rock formations on the way,
with only a few geology lectures. There will be some uphill but the view will be
inspiring; the community is unusually pleasant and happy to help.
There is the [Rust Users Forum](https://users.rust-lang.org/) and an active
[subreddit](https://www.reddit.com/r/rust/) which is unusually well-moderated.

First, why learn a new programming language? It is an investment of time and energy
and that needs some justification. Even if you do not immediately land
a cool job using that language, it stretches the mental muscles and makes you a
better programmer. That seems a poor kind of return-on-investment but if you're
not learning something _genuinely_ new all the time then you will stagnate and be
like the person who has ten years of experience in doing the same thing over and over.

## Where Rust Shines

Rust is a statically and strongly typed systems programming language. _statically_
means that all types are known at compile-time, _strongly_ means that these types
are designed to make it harder to write incorrect programs. A successful compilation
means you have a much better guarantee of correctness than with a cowboy language
like C. _systems_ means generating the best possible machine code with full control
of memory use.  So the uses are pretty hardcore: operating systems, device drivers
and embedded systems that might not even have an operating system.  However, it's
actually a very pleasant language to write normal application code in as well.

The big difference from C and C++ is that Rust is _safe by default_; all memory accesses
are checked. It is not possible to corrupt memory by accident.

The unifying principles behind Rust are:

  - strictly enforcing _safe borrowing_ of data
  - functions, methods and closures to operate on data
  - tuples, structs and enums to aggregate data
  - pattern matching to select and destructure data
  - traits to define _behaviour_ on data

There is a fast-growing ecosystem of available libraries through Cargo
but here we will concentrate on the core principles of the language
by learning to use the standard library. My advice is to write _lots of small programs_,
so learning to use `rustc` directly is a core skill. When doing the examples in this
tutorial I defined a little script called `rrun` which does a compilation and runs
the result:

```
rustc $1.rs && ./$1
```

## Setting Up

This tutorial assumes that you have Rust installed locally. Fortunately this is
[very straightforward](https://www.rust-lang.org/en-US/downloads.html).

```
$ curl https://sh.rustup.rs -sSf | sh
$ rustup component add rust-docs
```

I would recommend getting the default stable version; it's easy to switch later.

`rustup` is the command you use to manage your Rust installation - this 'component add' command
downloads the Rust documentation so you can view it locally. When a new stable release
appears, you just have to say `rustup update` to upgrade.

This gets the compiler, the Cargo package manager, the API documentation, and the Rust Book.
The journey of a thousand miles starts with one step, and this first step is painless.

You will probably already have an editor you like, and [basic Rust support](https://areweideyet.com/)
is good. I'd suggest you start out with basic syntax highlighting at first, and
work up as your programs get larger.

Personally I'm a fan of [Geany](https://www.geany.org/Download/Releases) which is
one of the few editors with Rust support out-of-the-box; it's particularly easy
on Linux since it's available through the package manager, but it works fine on
other platforms.

The main thing is knowing how to edit, compile and run Rust programs.
You learn to program with your _fingers_; type in
the code yourself, and learn to rearrange things efficiently with your editor.

Zed Shaw's [advice](https://learnpythonthehardway.org/book/intro.html) about learning
to program in Python remains good, whatever the language. He says learning to program
is like learning a musical instrument - the secret is practice and persistence.
There's also good advice from Yoga and the soft martial arts like Tai Chi;
feel the strain, but don't over-strain. You are not building dumb muscle here.

I'd like to thank the many contributors who caught bad English or bad Rust for me,
and thanks to David Marino for his cool characterization
of Rust as a friendly-but-hardcore no-nonsense knight in shining armour.

Steve Donovan © 2017 MIT license version 0.2.0

