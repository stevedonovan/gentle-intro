// match1.rs
use std::env;

fn main() {
    let first = env::args().nth(1).expect("please supply an argument");
    let n: i32 = first.parse().expect("not an integer!");
    
    let text = match n {
        0 => "zero",
        1 => "one",
        2 => "two",
        _ => "many"
    };

    println!("{}: {}",n, text);
}
