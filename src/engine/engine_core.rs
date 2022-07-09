use std::net::TcpListener;
use std::thread;

#[path="../network/mod.rs"]
mod network;

fn handle_session(mut session: impl network::session::Session) {
    loop {
        match session.read() {
            Ok((data, status)) => {
                if matches!(status, network::session::SessionStatus::Closed) {
                    return ;
                }
                session.write(data);
            }
            Err(status) => {
                match status {
                    network::session::SessionStatus::Error => print!("Error while reading"),
                    _ => panic!()
                }
            }
        }
    }
}

pub fn run() {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let session: network::session::UserSession = network::session::Session::new(stream);
                thread::spawn(move || {
                    handle_session(session);
                });
            }
            Err(_) => { print!("Failed to connect") }
        }
    }
}