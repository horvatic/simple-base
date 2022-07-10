use std::net::TcpStream;
use std::io::{Read, Write};

const BUFFER_SIZE: usize = 128;

pub trait Session {
    fn new(stream: TcpStream) -> Self;
    fn write(&mut self, write_data: Packet) -> SessionStatus;
    fn read(&mut self) -> Result<(Packet, SessionStatus), SessionStatus>;
}

pub struct UserSession {
    stream: TcpStream,
    data: [u8; BUFFER_SIZE]
}

pub struct Packet {
    data: Option<Vec<u8>>
}

pub enum SessionStatus {
    Open,
    Closed,
    Error
}

impl Session for UserSession {
    fn new(stream: TcpStream) -> Self {
        Self { stream, data: [0 as u8; 128] }
    }

    fn write(&mut self, write_data: Packet) -> SessionStatus {
        match self.stream.write(write_data.data.unwrap().as_ref()) {
            Ok(size) => {
                if size == 0 {
                    return SessionStatus::Closed;
                }
                return SessionStatus::Open;
            }
            Err(_) => {
                return SessionStatus::Error
            }
        }
    }

    fn read(&mut self) -> Result<(Packet, SessionStatus), SessionStatus> {
        match self.stream.read(&mut self.data) {
            Ok(size) => {
                if size == 0 { 
                    return Ok((Packet{ data: None }, SessionStatus::Closed))
                }
                return Ok((Packet{data: Some(self.data[0..size].to_vec())}, SessionStatus::Open));
            }
            Err(_) => { 
                return Err(SessionStatus::Error);
            }
        }
    }
}