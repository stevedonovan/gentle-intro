mod foo {
    #[derive(Debug)]
    pub struct Foo {
        s: &'static str
    }
    
    impl Foo {
        pub fn new(s: &'static str) -> Foo {
            Foo{s: s}
        }
    }
}

fn main() {
    let f = foo::Foo::new("hello");
    println!("{:?}",f);    
}
