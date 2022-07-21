use std::net::TcpStream;
use std::io::{Read, Write};
use crate::network::packet;

const BUFFER_SIZE: usize = 128;

pub trait Session {
    fn write(&mut self, write_data: packet::Packet) -> SessionStatus;
    fn read(&mut self) -> Result<(packet::Packet, SessionStatus), SessionStatus>;
}

pub struct UserSession {
    stream: TcpStream,
    data: [u8; BUFFER_SIZE]
}


#[derive(Copy, Clone)]
pub enum SessionStatus {
    Open,
    Closed,
    Error
}

impl UserSession {
    pub fn new(stream: TcpStream) -> Self {
        UserSession { stream, data: [0 as u8; 128] }
    }
}

impl Session for UserSession {
    fn write(&mut self, write_data: packet::Packet) -> SessionStatus {
        match self.stream.write(write_data.get_data().unwrap().as_ref()) {
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

    fn read(&mut self) -> Result<(packet::Packet, SessionStatus), SessionStatus> {
        match self.stream.read(&mut self.data) {
            Ok(size) => {
                if size == 0 { 
                    return Ok((packet::new_packet(None), SessionStatus::Closed))
                }
                let mut message =  String::from_utf8(self.data[0..size].to_vec()).unwrap();
                message.retain(|c| !c.is_whitespace() && c != '\n');
                if message == "{\"command\":\"exit\"}" {
                    return Ok((packet::new_packet(None), SessionStatus::Closed))
                }

                return Ok((packet::new_packet(Some(self.data[0..size].to_vec())), SessionStatus::Open));
            }
            Err(_) => { 
                return Err(SessionStatus::Error);
            }
        }
    }
}
