// thread6.rs
use std::thread;

fn main() {
    let one = thread::spawn(move || {
        println!("I am one");
    });

    let two = thread::spawn(move || {
        // wait for one to finish...
        one.join().unwrap();
        // and then we can go
        println!("I am two");
    });

    two.join().unwrap();
}
