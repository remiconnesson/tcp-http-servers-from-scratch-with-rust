use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    let connection_listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("TcpServer running on port 3000.");
    for stream in connection_listener.incoming() {
        let mut stream = stream.unwrap(); // shadow the iteration variable.
        println!("Connection established.");
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        stream.write(&mut buffer).unwrap();
    }
}
