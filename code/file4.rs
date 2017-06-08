// file4.rs
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn read_all_lines(filename: &str) -> io::Result<()> {
    let file = File::open(&filename)?;

    let mut reader = io::BufReader::new(file);
    let mut buf = String::new();
    let mut stdout = io::stdout();
    while reader.read_line(&mut buf)? > 0 {
        {
            let line = buf.trim_right();
            write!(stdout,"{}\n",line)?;
        }
        buf.clear();
    }

/*
    for line in reader.lines() {
        let line = line?;
        println!("{}",line);
    }
*/
    Ok(())
}

fn main() {
    read_all_lines("file4.rs").expect("bad file man!");
}
