// trait3.rs

struct FRange {
    val: f64,
    end: f64,
    incr: f64
}

fn range(x1: f64, x2: f64, skip: f64) -> FRange {
    FRange {val: x1, end: x2, incr: skip}
}
    

impl Iterator for FRange {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.val;
        if res >= self.end {
            None
        } else {
            self.val += self.incr;
            Some(res)
        }
    }
}

pub trait ToVec {
    type Item;
    fn to_vec(self) -> Vec<Self::Item>;
}

use std::iter::FromIterator;

impl <T,I> ToVec for I
where T: Sized, I: Iterator<Item=T> {
    type Item = T;
    
    fn to_vec(self) -> Vec<Self::Item> {
        FromIterator::from_iter(self)
    }
}


fn main() {
    for x in range(0.0, 1.0, 0.1) {
        println!("{:.1} ",x);
    }

    let v: Vec<f64> = range(0.0, 1.0, 0.1).collect();
    println!("{:?}",v);

    let v = range(0.0, 1.0, 0.1).to_vec();
    println!("{:?}",v);
}
