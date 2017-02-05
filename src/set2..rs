// set2.rs

use std::collections::HashSet;

fn make_set(words: &str) -> HashSet<&str> {
    words.split_whitespace().collect()
}

fn main() {
    let fruit = make_set("apple orange pear");
    let colours = make_set("brown purple orange yellow");

    for c in fruit.intersection(&colours) {
        println!("{:?}",c);
    }

    println!("{:?}",fruit);
    
}
