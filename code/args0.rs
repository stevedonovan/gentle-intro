// args0.rs
fn main() {
    for arg in std::env::args() {
        println!("'{}'",arg);
    }
}
