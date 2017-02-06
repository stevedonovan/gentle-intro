// queue2.rs
use std::collections::VecDeque;

fn main() {
    let mut queue = VecDeque::new();
    queue.push_back("hello");
    queue.push_back("dolly");
    while let Some(word) = queue.pop_front() {
        print!("{} ", word);
    }
    println!("");
}
