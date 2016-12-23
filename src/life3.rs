// life3.rs

#[derive(Debug)]
struct A <'a> {
    s: &'a str
}

fn main() {
    let string = "I'm a little string".to_string();
    let a = A { s: &string };

    println!("{:?}",a);
}
