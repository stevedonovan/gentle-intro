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
