use crate::network::session;

pub fn handle_session(session: &mut impl session::Session) {
    match session.read() {
        Ok((data, status)) => {
            if matches!(status, session::SessionStatus::Closed) {
                return ;
            }
            session.write(data);
        }
        Err(status) => {
            match status {
                session::SessionStatus::Error => print!("Error while reading"),
                _ => panic!()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub struct MockSession {
        pub write_data: session::Packet,
        pub read_return_data: String,
        pub read_return_status: session::SessionStatus
    }

    impl MockSession {
        pub fn new() -> Self {
            return MockSession { write_data: session::new_packet(None), read_return_status: session::SessionStatus::Error, read_return_data: "".to_string() }
        }

        pub fn set_read_return(&mut self, sample_data: String, session_status: session::SessionStatus) {
            self.read_return_data = sample_data;
            self.read_return_status = session_status;
        } 
    } 

    impl session::Session for MockSession {
        fn write(&mut self, write_data: session::Packet) -> session::SessionStatus {
            self.write_data = write_data;
            return session::SessionStatus::Open;
        }

        fn read(&mut self) -> Result<(session::Packet, session::SessionStatus), session::SessionStatus> {
            if matches!(self.read_return_status, session::SessionStatus::Error) {
                return Err(session::SessionStatus::Error);
            }
            
            let sample_data = self.read_return_data.as_bytes();
            return Ok((session::new_packet(Some(sample_data.to_vec())), self.read_return_status))
        }
    }

    #[test]
    fn read_write_close_successfully() {
        let mut mock_session = MockSession::new();
        mock_session.set_read_return("A".to_string(), session::SessionStatus::Open);
    
        super::handle_session(&mut mock_session);
    
        match mock_session.write_data.get_data() {
            Some(v) => {
                let s = String::from_utf8(v).unwrap();
                assert_eq!(s, "A")
            }
            None => assert!(false, "A was not read, or write too"),
        }
    }
    
    #[test]
    fn read_close_successfully() {
        let mut mock_session = MockSession::new();
        mock_session.set_read_return("".to_string(), session::SessionStatus::Closed);
    
        super::handle_session(&mut mock_session);
    
        match mock_session.write_data.get_data() {
            Some(_) => {
                assert!(false, "Did not expect write")
            }
            None => assert!(true),
        }
    }
}