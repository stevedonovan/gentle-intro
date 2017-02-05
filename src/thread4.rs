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
