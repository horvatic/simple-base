use std::net::{TcpListener, TcpStream};
use std::thread;
use std::cell::Cell;

use crate::engine::engine_session;
use crate::network::session;
use crate::command::command_factory;

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
    thread::spawn(move || {
        let mut session = session::UserSession::new(stream);
        while runner.get().status() {
            let result = engine_session::handle_session(&mut session, command_factory::build);
            if matches!(result, session::SessionStatus::Closed) {
                return;
            }
        }
    });
}

pub fn run(addr: &str) {
    let listener = TcpListener::bind(addr).unwrap();
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
