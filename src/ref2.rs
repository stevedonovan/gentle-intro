// ref2.rs

/*
fn array_ref() -> &[i32] {
    let arr = [1,2,3];
    &arr
}
*/

fn tail(s: &str) -> &str {
    &s[1..]
}

fn main() {
    let rs1 = &("hello dolly".to_string());
    println!("ref {}",rs1);
}
