// thread1.rs
use std::thread;
use std::time;

fn main() {
    thread::spawn(|| println!("hello"));
    thread::spawn(|| println!("dolly"));
    
    println!("so fine");
    thread::sleep(time::Duration::from_millis(100));
}
