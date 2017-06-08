// set2.rs
use std::collections::HashSet;
use std::hash::Hash;

trait ToSet<T> {
    fn to_set(self) -> HashSet<T>;
}

impl <T,I> ToSet<T> for I
where T: Eq + Hash, I: Iterator<Item=T> {
    
    fn to_set(self) -> HashSet<T> {
       self.collect()
    }
}

/*
fn make_set(words: &str) -> HashSet<&str> {
    words.split_whitespace().collect()
}
*/

fn make_set(words: &str) -> HashSet<String> {
    words.split_whitespace().map(|s| s.to_string()).collect()
}


fn main() {
    let fruit = make_set("apple orange pear");
    let colours = make_set("brown purple orange yellow");

    for c in fruit.intersection(&colours) {
        println!("{:?}",c);
    }

    let intersect: () = fruit.intersection(&colours).cloned().to_set();
    
    println!("{:?}",intersect);
    println!("{:?}",fruit);
    
}
