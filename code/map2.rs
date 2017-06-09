// map2.rs

use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("one",1);
    map.insert("two",2);
    map.insert("three",3);

    if let Some(v) = map.get("two") {
        let res = *v + 1;
        assert_eq!(res, 3);
    }

    println!("before {}",map.get("two").unwrap());

    {
        let mut mref = map.get_mut("two").unwrap();
        *mref = 20;
    }

    match map.get_mut("two") {
        Some(mref) => *mref = 20,
        None => panic!("_now_ we can panic!")
    }

    println!("after {}",map.get("two").unwrap());


}
