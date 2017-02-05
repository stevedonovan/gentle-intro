// thread3.rs
use std::thread;

fn main() {
    let name = "dolly".to_string();
    let t = thread::spawn(|| {
        println!("hello {}",name);
    });
    println!("wait {:?}", t.join());
}
