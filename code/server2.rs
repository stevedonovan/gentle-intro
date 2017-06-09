// server2.rs
use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::io;

fn handle_connection(stream: TcpStream) -> io::Result<()>{
    let mut rdr = io::BufReader::new(stream);
    let mut text = String::new();
    rdr.read_line(&mut text)?;
    println!("got '{}'",text.trim_right());
    Ok(())
}

fn main() {

    let listener = TcpListener::bind("127.0.0.1:8000").expect("could not start server");

    // accept connections and get a TcpStream
    for connection in listener.incoming() {
        match connection {
            Ok(stream) => {
                if let Err(e) = handle_connection(stream) {
                    println!("eror {:?}",e);
                }
            }
            Err(e) => { print!("connection failed {}\n",e); }
        }
    }
}
