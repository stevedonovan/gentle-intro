// map1.rs

use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("one","eins");
    map.insert("two","zwei");
    map.insert("three","drei");

    assert_eq! (map.contains_key("two"),true);
    assert_eq! (map.get("two"),Some(&"zwei"));

    for (k,v) in map.iter() {
        println!("key {} value {}",k,v);
    }

}
