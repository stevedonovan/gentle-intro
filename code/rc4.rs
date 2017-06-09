// rc2.rs
use std::rc::Rc;
use std::rc::Weak;

#[derive(Debug)]
struct Parent {
    firstname: String,
    surname: String,
    children: Vec<Rc<Parent>>,
    mother: Weak<Parent>,
    father: Weak<Parent>
    
}

impl Parent {
    fn new(first: &str, last: &str) -> Parent {
        Parent {
            firstname: first.to_string(),
            surname: last.to_string(),
            children: Vec::new(),
            mother: Weak::new(),
            father: Weak::new(),
        }
    }
   
}

fn main() {
    // the parents
    let mut father = Parent::new("John","Smith");
    let mut mother = Parent::new("Mary","Jones");
    // the kids
    let mut bob = Parent::new("Bob","Smith");
    let mut alice = Parent::new("Alice","Smith");
    // father gets a vector of strong references to his kids
    father.children = vec! [Rc::new(bob),Rc::new(alice)];

    // mother gets a copy!
    mother.children = father.children.clone();
    
    let father_ref = Rc::new(father);
    let mother_ref = Rc::new(mother);
    bob.mother = Rc::downgrade(&mother_ref);
    bob.father = Rc::downgrade(&father_ref);
    alice.mother = Rc::downgrade(&mother_ref);
    alice.father = Rc::downgrade(&father_ref);

    assert_eq! (alice.father.upgrade().unwrap().surname, "Smith");
    
    println!("mother {:#?}",mother_ref);
}
