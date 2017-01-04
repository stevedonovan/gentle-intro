// closure4.rs
fn main() {
    let mut v: Vec<Box<Fn(f64)->f64>> = Vec::new();
    v.push(Box::new(|x| x*x));
    v.push(Box::new(|x| x/2.0));

    for f in v.iter() {
        let res = f(1.0);
        println!("res {}",res);
    }
}
