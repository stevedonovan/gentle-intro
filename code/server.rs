// server.rs
use std::net::TcpListener;
use std::io::prelude::*;

fn main() {

    let listener = TcpListener::bind("127.0.0.1:8000").expect("could not start server");

    // accept connections and get a TcpStream
    for connection in listener.incoming() {
        match connection {
            Ok(mut stream) => {
                let mut text = String::new();
                stream.read_to_string(&mut text).expect("read failed");
                println!("got '{}'",text);
            }
            Err(e) => { print!("connection failed {}\n",e); }
        }
    }
}
