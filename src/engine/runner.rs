use std::net::TcpListener;
use std::thread;

use crate::engine::engine_session;
use crate::network::session;

pub fn run() {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let session: session::UserSession = session::Session::new(stream);
                thread::spawn(move || {
                    engine_session::handle_session(session);
                });
            }
            Err(_) => { print!("Failed to connect") }
        }
    }
}