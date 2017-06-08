// slice1.rs
fn main() {
    let ints = [1,2,3,4,5];    
    let slice1 = &ints[0..2]; 
    let slice2 = &ints[1..];

    println!("ints {:?}",ints);
    println!("slice1 {:?}",slice1);
    println!("slice2 {:?}",slice2);    
}
