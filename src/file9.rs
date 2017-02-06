// file9.rs
use std::env;

fn main() {
    let mut path = env::current_dir().expect("can't access current dir");
    loop {
        path.push("config.txt");
        if path.is_file() {
            println!("gotcha {}",path.display());
            break;
        } else {
            path.pop();
        }
        if ! path.pop() {
            break;
        }
    }
}
