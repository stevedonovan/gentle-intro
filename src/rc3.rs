// rc2.rs
use std::rc::Rc;

#[derive(Debug)]
struct Parent {
    firstname: String,
    surname: String,
    children: Vec<Rc<Parent>>
}

impl Parent {
    fn new(first: &str, last: &str) -> Parent {
        Parent {
            firstname: first.to_string(),
            surname: last.to_string(),
            children: Vec::new()
        }
    }
   
}

fn main() {
    let mut father = Parent::new("John","Smith");
    let mut mother = Parent::new("Mary","Jones");
    let bob = Parent::new("Bob","Smith");
    let alice = Parent::new("Alice","Smith");
    father.children = vec! [Rc::new(bob),Rc::new(alice)];
    mother.children = father.children.clone();
    println!("father {:?}",father);
    println!("mother {:#?}",mother);
}
