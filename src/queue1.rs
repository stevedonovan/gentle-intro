// queue1.rs
use std::collections::VecDeque;

fn main() {
    let mut queue = VecDeque::new();
    queue.push_back(10);
    queue.push_back(20);
    assert_eq! (queue.pop_front(),Some(10));
    assert_eq! (queue.pop_front(),Some(20)); 
}
