// vec4.rs
fn main() {
    let mut v1 = vec![1,10,5,1,2,11,2,40];
    v1.sort();
    v1.dedup();
    assert_eq!(v1,&[1,2,5,10,11,40]);
}
