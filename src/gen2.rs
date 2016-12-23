// gen1.rs

use std::ops::Mul;

fn sqr<T: Mul + Copy> (x: T) -> T::Output {
    x*x
}

fn main() {
    let res = sqr(10.0);
    println!("res {}",res);
}
