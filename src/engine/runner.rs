use std::net::{TcpListener, TcpStream};
use std::thread;
use std::cell::Cell;

use crate::engine::engine_session;
use crate::network::session;

#[derive(Copy, Clone)]
pub struct Runner {
    running: bool
}

impl Runner {
    pub fn new() -> Self {
        Runner { running: true }
    }

    pub fn status(&self) -> bool {
        self.running
    }

    pub fn stop(&mut self) {
        self.running = false
    }
}

pub fn spawn_session(runner: Cell<Runner>, stream: TcpStream) {
    let session = Box::new(session::new_user_session(stream));
    thread::spawn(move || {
        let mut engine_user_session = engine_session::EngineSession::new(session);
        while runner.get().status() {
            engine_user_session.handle_session();
        }
    });
}

pub fn run() {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    let mut main_runner = Runner::new();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let runner = Cell::new(main_runner);
                spawn_session(runner, stream)
            }
            Err(_) => { print!("Failed to connect") }
        }
    }
    main_runner.stop();
}
