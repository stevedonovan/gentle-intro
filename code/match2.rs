// match2.rs
use std::env;

fn main() {
    let first = env::args().nth(1).expect("please supply an argument");
    let n: i32 = first.parse().expect("not an integer!");
    
    let text = match n {
        0..=3 => "small",
        4..=6 => "medium",
        _ => "large"
     };

    println!("{}: {}",n, text);
}
