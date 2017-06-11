// thread5.rs
// thread5.rs
use std::thread;
use std::sync::Arc;

struct MyString(String);

impl MyString {
    fn new(s: &str) -> MyString {
        MyString(s.to_string())
    }
}

fn main() {
    let mut threads = Vec::new();
    let name = Arc::new(MyString::new("dolly"));

    for i in 0..5 {
        let tname = name.clone();
        let t = thread::spawn(move || {
            println!("hello {} count {}", tname.0, i);
        });
        threads.push(t);
    }

    for t in threads {
        t.join().expect("thread failed");
    }
}
