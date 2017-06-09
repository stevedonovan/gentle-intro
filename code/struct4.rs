// struct4.rs
use std::fmt;

#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    
    fn new(first: &str, name: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: name.to_string()
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}",self.first_name, self.last_name)
    }
    
    fn set_first_name(&mut self, name: &str) {
        self.first_name = name.to_string();
    }
    
    fn to_tuple(self) -> (String,String) {
        (self.first_name, self.last_name)
    }
}

fn main() {
    let mut p = Person::new("John","Smith");
    
    println!("{:?}", p);
    
    p.set_first_name("Jane");
    
    println!("{:?}", p);
    
    println!("{:?}", p.to_tuple());
    // p has now moved.
    
}
