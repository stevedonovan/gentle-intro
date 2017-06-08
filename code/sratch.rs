use std::rc::Rc;

fn main() {
    let s = "hello dolly".to_string();
    let strong1= Rc::new(s);
    let weak1 = Rc::downgrade(&strong1);
    let strong2 = weak1.upgrade();

    println!("got {:?} {:?}",strong1,strong2);
}
