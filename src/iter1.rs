// iter1.rs
fn main() {
    let mut iter = 0..3;
    assert_eq!(iter.next(),Some(0));
    assert_eq!(iter.next(),Some(1));
    assert_eq!(iter.next(),Some(2));
    assert_eq!(iter.next(),None);
}
