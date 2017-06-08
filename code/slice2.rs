// slice2.rs
fn main() {
    let ints = [1,2,3,4,5];    
    let slice = &ints;
    let first = slice.get(0);
    let last = slice.get(5);
    
    println!("first {} {}",first.is_some(),first.is_none());
    println!("last {} {}",last.is_some(), last.is_none());

    println!("first value {}",first.unwrap());
}
