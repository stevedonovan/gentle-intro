// file6.rs
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn quit(msg: &str) {
    write!(io::stderr(),"error: {}\n", msg).expect("write?");
    std::process::exit(1);
}

fn write_out(f: &str) -> io::Result<()> {
    let mut out = File::create(f)?;
    write!(out,"answer is {}\n",42)?;
    Ok(())
}

fn main() {
  write_out("test.txt").expect("write failed");

  quit("we quit!");
}
