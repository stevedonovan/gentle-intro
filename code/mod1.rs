mod foo {
    #[derive(Debug)]
    pub struct Foo {
        pub s: &'static str
    }    
}

fn main() {
    let f = foo::Foo{s: "hello"};
    println!("{:?}",f);    
}
