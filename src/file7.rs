// file7.rs
use std::env;
use std::path::PathBuf;

fn main() {
    let home = env::home_dir().expect("no home!");
    let mut path = PathBuf::from(home);
    path.push(home);
    path.push(".cargo");

    if path.is_dir() {
        println!("{}",path.display());
    }
}
