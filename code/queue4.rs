// queue4.rs
use std::collections::BinaryHeap;

fn main() {
    let mut queue = BinaryHeap::new();
    queue.push(5);
    queue.push(20);
    queue.push(10);
    queue.push(33);
    
    while let Some(n) = queue.pop() {
        print!("{} ", n);
    }
    println!("");
}
