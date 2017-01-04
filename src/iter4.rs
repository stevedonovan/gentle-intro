// iter4.rs

#[derive(Debug)]
struct Foo {
    s: Option<String>
}

fn main() {
    let mut f = Foo{s: Some("hello".to_string())};

    if f.s.is_some() {
        f.is.unwrap
    }
    println!("{:?}",f);

    
}


