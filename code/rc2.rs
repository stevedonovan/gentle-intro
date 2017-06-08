// rc2.rs
use std::rc::Rc;

struct HasDrop(&'static str);

impl HasDrop {
    fn new(name: &'static str) -> HasDrop {
        HasDrop(name)
    }

    fn name(&self) -> &'static str {
        self.0
    }
}

impl Drop for HasDrop {
    fn drop(&mut self) {
        println!("Dropping {}",self.name());
    }
}

fn main() {
    let rs1 = Rc::new(HasDrop::new("frodo"));
    let rs2 = rs1.clone();

    println!("name {}, {}", rs1.name(), rs2.name());
}
