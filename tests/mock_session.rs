use simple_base::network::session::{self, new_packet};

pub struct MockSession {
    pub write_data: session::Packet
}

pub fn new_mock_session() -> MockSession {
    return MockSession { write_data: new_packet(None) }
}

impl session::Session for MockSession {
    fn write(&mut self, write_data: session::Packet) -> session::SessionStatus {
        self.write_data = write_data;
        return session::SessionStatus::Open;
    }

    fn read(&mut self) -> Result<(session::Packet, session::SessionStatus), session::SessionStatus> {
        let vec: Vec<u8> = vec!(u8::from_str_radix("A", 16).unwrap());
        return Ok((session::new_packet(Some(vec)), session::SessionStatus::Open))
    }
}
