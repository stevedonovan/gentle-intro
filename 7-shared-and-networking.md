# Threads, Networking and Sharing

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
`&String` from `greeting.borrow()` or the `&mut String` from `greeting.borrow_mut()`.

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
cheap, because each time you clone it just updates a _reference count_ to the _same data_.
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
type PlayerRef = Rc<RefCell<Player>>;
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

## Threads Don't Borrow

It's possible for the thread closure to capture values, but by _moving_,  not by _borrowing_!

```rust
// thread3.rs
use std::thread;

fn main() {
    let name = "dolly".to_string();
    let t = thread::spawn(|| {
        println!("hello {}", name);
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
7 |         println!("hello {}", name);
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
            println!("hello {}", i);
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

Threads can't share the same environment - by _design_ in Rust. In particular,
they cannot share regular references because the closures move their captured variables.

_shared references_ are fine however - but you cannot use `Rc` for this. This is because
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
            println!("hello {} count {}", tname,i);
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
unpredictable. A program may run fine on your machine, but occasionally crash on the
server, usually on the weekend. Worse still, the symptoms of such problems are
not easy to diagnose.

## Channels

There are ways to send data between threads. This
is done in Rust using _channels_. `std::sync::mpsc::channel()` returns a tuple consisting
of the _receiver_ channel and the _sender_ channel. Each thread is passed a copy
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

`send` never blocks, which is useful because threads can push out data without waiting
for the receiver to process. In addition, the channel is buffered so multiple
send` operations can take place, which will be received in order.

However, not blocking means that `Ok` does not automatically mean 'successfully delivered message'!

A `sync_channel` _does_ block on send. With an argument of zero, the send blocks until the
recv happens. The threads must meet up or _rendezvous_ (on the sound principle that most things
sound better in French.)

```rust
    let (tx, rx) = mpsc::sync_channel(0);

    let t1 = thread::spawn(move || {
        for i in 0..5 {
            tx.send(i).unwrap();
        }
    });

    for _ in 0..5 {
        let res = rx.recv().unwrap();
        println!("{}",res);
    }
    t1.join().unwrap();
```

We can easily cause an error here by calling `recv` when there has been no corresponding `send`, e.g
by looping `for i in 0..4`. The thread ends, and `tx` drops, and then `recv` will fail. This will also
happen if the thread panics, which causes its stack to be unwound, dropping any values.

If the `sync_channel` was created with a non-zero argument `n`, then it acts like a queue with a
maximum size of `n` - `send` will only block when it tries to add more than `n` values to the queue.

Channels are strongly typed - here the channel had type `i32` - but type inference makes this implicit.
If you need to pass different kinds of data, then enums are a good way to express this.

## Synchronization

Let's look at _synchronization_. `join` is very basic, and merely waits until a
particular thread has finished.  A `sync_channel` synchronizes two threads - in the last example, the
spawned thread and the main thread are completely locked together.

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


## Shared State

But how can threads _modify_ shared state?

Recall the `Rc<RefCell>` strategy for _dynamically_ doing a
mutable borrow on shared references.  The threading equivalent to `RefCell` is
`Mutex` - you may get your mutable reference by calling `lock`. While this reference
exists, no other thread can access it. `mutex` stands for 'Mutual Exclusion' - we lock
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
## Higher-Level Operations

It's better to find higher-level ways of doing threading, rather than managing the synchronization
yourself. An example is when you need to do things in parallel and collect the results. One very
cool crate is [pipeliner](https://docs.rs/pipeliner/0.1.1/pipeliner/) which has a very straightforward
API. Here's the 'Hello, World!' - an iterator feeds us inputs and we execute up to `n` of the operations
on the values in parallel.

```rust
extern crate pipeliner;
use pipeliner::Pipeline;

fn main() {
    for result in (0..50).with_threads(10).map(|x| x + 1) {
        println!("result: {}", result);
    }
}
```

It's silly of course, because the operation is so cheap to calculate, but shows how simple it is
to collect parallel results.

Here's something more useful. Doing network operations in parallel is very useful, because they can
take time, and you don't want to wait for them _all_ to finish before starting to do work.

This example is pretty crude (believe me, there are better ways of doing it) but here we want to focus
on the principle. We reuse the `shell` function defined in section 4 to call `ping` on a range
of IP4 addresses.

```rust
extern crate pipeliner;
use pipeliner::Pipeline;

use std::process::Command;

fn shell(cmd: &str) -> (String,bool) {
    let cmd = format!("{} 2>&1",cmd);
    let output = Command::new("/bin/sh")
        .arg("-c")
        .arg(&cmd)
        .output()
        .expect("no shell?");
    (
        String::from_utf8_lossy(&output.stdout).trim_right().to_string(),
        output.status.success()
    )
}

fn main() {
    let addresses: Vec<_> = (1..40).map(|n| format!("ping -c1 192.168.0.{}",n)).collect();
    let n = addresses.len();

    for result in addresses.with_threads(n).map(|s| shell(&s)) {
        if result.1 {
            println!("got: {}", result.0);
        }
    }

}
```

And the result on my home network looks like this:

```
got: PING 192.168.0.1 (192.168.0.1) 56(84) bytes of data.
64 bytes from 192.168.0.1: icmp_seq=1 ttl=64 time=43.2 ms

--- 192.168.0.1 ping statistics ---
1 packets transmitted, 1 received, 0% packet loss, time 0ms
rtt min/avg/max/mdev = 43.284/43.284/43.284/0.000 ms
got: PING 192.168.0.18 (192.168.0.18) 56(84) bytes of data.
64 bytes from 192.168.0.18: icmp_seq=1 ttl=64 time=0.029 ms

--- 192.168.0.18 ping statistics ---
1 packets transmitted, 1 received, 0% packet loss, time 0ms
rtt min/avg/max/mdev = 0.029/0.029/0.029/0.000 ms
got: PING 192.168.0.3 (192.168.0.3) 56(84) bytes of data.
64 bytes from 192.168.0.3: icmp_seq=1 ttl=64 time=110 ms

--- 192.168.0.3 ping statistics ---
1 packets transmitted, 1 received, 0% packet loss, time 0ms
rtt min/avg/max/mdev = 110.008/110.008/110.008/0.000 ms
got: PING 192.168.0.5 (192.168.0.5) 56(84) bytes of data.
64 bytes from 192.168.0.5: icmp_seq=1 ttl=64 time=207 ms
...
```

The active addresses come through pretty fast within the first half-second, and we then wait for the negative
results to come in. Otherwise, we would wait for the better part of a minute! You can now proceed
to scrape things like ping times from the output, although this would only work on Linux. `ping`
is universal, but the exact output format is different for each platform.  To do better we need to use
the cross-platform Rust networking API, and so let's move onto Networking.

## A Better Way to Resolve Addresses

If you _just_ want availability and not detailed ping statistics, the `std::net::ToSocketAddrs` trait
will do any DNS resolution for you:

```rust
use std::net::*;

fn main() {
    for res in "google.com:80".to_socket_addrs().expect("bad") {
        println!("got {:?}", res);
    }
}
// got V4(216.58.223.14:80)
// got V6([2c0f:fb50:4002:803::200e]:80)
```

It's an iterator because there is often more than one interface associated with a domain - there are
both IPV4 and IPV6 interfaces to Google.

So, let's naively use this method to rewrite the pipeliner example. Most networking protocols use both an
address and a port:

```rust
extern crate pipeliner;
use pipeliner::Pipeline;

use std::net::*;

fn main() {
    let addresses: Vec<_> = (1..40).map(|n| format!("192.168.0.{}:0",n)).collect();
    let n = addresses.len();

    for result in addresses.with_threads(n).map(|s| s.to_socket_addrs()) {
        println!("got: {:?}", result);
    }
}
// got: Ok(IntoIter([V4(192.168.0.1:0)]))
// got: Ok(IntoIter([V4(192.168.0.39:0)]))
// got: Ok(IntoIter([V4(192.168.0.2:0)]))
// got: Ok(IntoIter([V4(192.168.0.3:0)]))
// got: Ok(IntoIter([V4(192.168.0.5:0)]))
// ....
```

This is much faster than the ping example because it's just checking that the IP address is valid - if we fed
it a list of actual domain names the DNS lookup could take some time, hence the importance of parallelism.

Suprisingly, it sort-of Just Works. The fact that everything in the standard library implements `Debug`
is great for exploration as well as debugging.  The iterator is returning `Result` (hence `Ok`) and
in that `Result` is an `IntoIter` into a `SocketAddr` which is an enum with either a IPV4 or a IPV6 address.

```rust
    for result in addresses.with_threads(n)
        .map(|s| s.to_socket_addrs().unwrap().next().unwrap())
    {
        println!("got: {:?}", result);
    }
// got: V4(192.168.0.1:0)
// got: V4(192.168.0.39:0)
// got: V4(192.168.0.3:0)
```
This also works, surprisingly enough, at least for our simple example. The first `unwrap` gets rid of
the `Result`, and then we explicitly pull the first value out of the iterator. The `Result` will get
bad typically when we give a nonsense address (like an address name without a port.)

## TCP Client Server

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

`TcpStream::connect` takes anything that can convert into a `SocketAddr`, in particular the plain strings
we have been using.

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
                println!("got '{}'", text);
            }
            Err(e) => { println!("connection failed {}", e); }
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
    println!("got '{}'", text.trim_right());
    Ok(())
}

fn main() {

    let listener = TcpListener::bind("127.0.0.1:8000").expect("could not start server");

    // accept connections and get a TcpStream
    for connection in listener.incoming() {
        match connection {
            Ok(stream) => {
                if let Err(e) = handle_connection(stream) {
                    println!("error {:?}", e);
                }
            }
            Err(e) => { print!("connection failed {}\n", e); }
        }
    }
}
```

`read_line` might fail in `handle_connection`, but the resulting error is safely handled.

One-way communications like this are certainly useful - for instance. a set of services across a
network which want to collect their status reports together in one central place. But it's
reasonable to expect a polite reply, even if just 'ok'!

A simple example is an 'echo' server. The client writes some text ending in a newline to the
server, and receives the same text back with a newline - the stream is readable and writeable.

```rust
// client_echo.rs
use std::io::prelude::*;
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8000").expect("connection failed");
    let msg = "hello from the client!";

    write!(stream,"{}\n", msg).expect("write failed");

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


