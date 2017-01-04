fn main() {
    let s = "hello".to_string();

    let sl1 = &s[1..];

    let sl2 = sl1;

    println!("{} {}",sl1,sl2);

}
