use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

fn handle_connection(mut stream: TcpStream) {
    let mut data = [0 as u8; 128];
    loop {
        match stream.read(&mut data) {
            Ok(size) => {
                if size == 0 { 
                    break;
                }
                stream.write(&data[0..size]).unwrap();
            }
            Err(_) => { 
                println!("Error");
            }
        }
    }
}

pub fn listen() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_connection(stream);
                });
            }
            Err(_) => { print!("Failed to connect") }
        }
    }
    Ok(())
}