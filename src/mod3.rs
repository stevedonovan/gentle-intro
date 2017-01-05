// mod3.rs
mod foo;
mod boo;

fn main() {
    let f = foo::Foo::new("hello");
    let res = boo::answer();
    let q = boo::bar::question();
    println!("{:?} {} {}",f,res,q);    
}
