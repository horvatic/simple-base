use std::net::TcpStream;
use std::io::{Read, Write};

const BUFFER_SIZE: usize = 128;

pub trait Session {
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

#[derive(Copy, Clone)]
pub enum SessionStatus {
    Open,
    Closed,
    Error
}

pub fn new_user_session(stream: TcpStream) -> impl Session {
    UserSession { stream, data: [0 as u8; 128] }
}

pub fn new_packet(data: Option<Vec<u8>>) -> Packet {
    return Packet { data }
}

impl Packet {
    pub fn get_data(&self) -> Option<Vec<u8>> {
        return self.data.clone();
    }
}

impl Session for UserSession {
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
                    return Ok((new_packet(None), SessionStatus::Closed))
                }
                return Ok((new_packet(Some(self.data[0..size].to_vec())), SessionStatus::Open));
            }
            Err(_) => { 
                return Err(SessionStatus::Error);
            }
        }
    }
}
