// string4.rs
fn main() {
    let text = "the brown fox and the lazy dog";
    let stripped: String = text.chars()
        .filter(|ch| ! ch.is_whitespace()).collect();

    let words: Vec<&str> = text.split_whitespace().collect();

    println!("{}",stripped);
    println!("{:?}",words);
}
