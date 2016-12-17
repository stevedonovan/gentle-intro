// slice4.rs
fn main() {
    let ints = [1,2,3,4,5];    
    let slice = &ints;

    for s in slice.windows(2) {
        println!("window {:?}",s);
    }
    
}
