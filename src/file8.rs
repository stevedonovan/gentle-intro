// file8.rs
use std::env;
//use std::path::PathBuf;

fn main() {
    let mut path = env::current_dir().expect("can't access current dir");
    loop {
        println!("{}",path.display());
        if ! path.pop() {
            break;
        }
    }
}
