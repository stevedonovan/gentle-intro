// life4.rs

#[derive(Debug)]
struct A <'a> {
    s: &'a str
}

fn main() {
    let a = A { s: &"I'm a little string".to_string() };

    println!("{:?}",a);
}
