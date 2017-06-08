// iter3.rs
fn main() {
    let arr = [10,20,30];
    for i in arr.iter() {
        println!("{}",i);
    }

    let slice = &arr;
    for i in slice {
        println!("{}",i);
    }

    
}
