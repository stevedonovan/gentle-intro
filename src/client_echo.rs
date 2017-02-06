// client_echo.rs
use std::io::prelude::*;
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8000").expect("connection failed");
    let msg = "hello from the client!";

    write!(stream,"{}\n",msg).expect("write failed");

    let mut resp = String::new();
    stream.read_to_string(&mut resp).expect("read failed");
    let text = resp.trim_right();
    assert_eq!(msg,text);
}
