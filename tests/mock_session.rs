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
        let sample_data = "A".as_bytes();
        return Ok((session::new_packet(Some(sample_data.to_vec())), session::SessionStatus::Open))
    }
}
