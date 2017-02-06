// queue3.rs
use std::collections::VecDeque;

fn main() {
    let mut queue = VecDeque::new();
    queue.extend(1..10);
    queue.extend(12..15);
    while let Some(word) = queue.pop_front() {
        print!("{} ", word);
    }
    println!("");
}
