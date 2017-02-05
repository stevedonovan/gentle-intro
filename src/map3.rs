// map3.rs
use std::fs::File;
use std::io::prelude::*;

use std::collections::HashMap;

fn main() {
    let mut f = File::  open("sherlock.txt").expect("can't open sherlock.txt");
    let mut text = String::new();
    f.read_to_string(&mut text).expect("can't read the file");
    let mut map = HashMap::new();

    for s in text.split(|c: char| ! c.is_alphabetic()) {
        let word = s.to_lowercase();
        let mut entry = map.entry(word).or_insert(0);
        *entry += 1;
    }

    println!("total words {}",map.len());

    let mut entries: Vec<_> = map.into_iter().collect();
    entries.sort_by(|a,b| b.1.cmp(&a.1));
    for e in entries.iter().take(20) {
        println!("{} {}", e.0, e.1);
    }


}
