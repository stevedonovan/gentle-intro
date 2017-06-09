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
