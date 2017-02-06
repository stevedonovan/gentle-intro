// badcell.rs
use std::cell::RefCell;

fn main() {
    let greeting = RefCell::new("hello".to_string());

    assert_eq!(*greeting.borrow(), "hello");
    assert_eq!(greeting.borrow().len(), 5);

    let mut gr = greeting.borrow_mut();
    *gr = "hola".to_string();
    
    assert_eq!(*greeting.borrow(), "hola");

}
