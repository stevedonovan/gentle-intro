// file11.rs
use std::env;
use std::fs;
use std::io;

fn dump_dir(dir: &str) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let data = entry.metadata()?;
        let path = entry.path();
        if data.is_file() {
            if let Some(ex) = path.extension() {
                if ex == "rs" && data.len() > 1024 {
                    println!("{} length {}",path.display(),data.len());
                }
            }
        }
    }
    Ok(())
}

fn main() {
    let dir = env::args().skip(1).next().unwrap_or(".".to_string());

    dump_dir(&dir).expect("could not dump dir");
}
