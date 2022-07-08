use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn handle_connection(mut stream: TcpStream) {
    let mut data = [0 as u8; 100];
    match stream.read(&mut data) {
        Ok(size) => {
            stream.write(&data[0..size]).unwrap();
        }
        Err(_) => { 
            println!("Error");
        }
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream);
            }
            Err(_) => { print!("Failed to connect") }
        }
    }
    Ok(())
}