use crate::network::session;
use crate::network::packet;
use serde_json::{Value, Result};

pub fn handle_session(session: &mut impl session::Session, command_factory: fn(Option<Vec<u8>>) -> Result<(Value, fn(Value) -> Result<Value>)>) -> session::SessionStatus {
    match session.read() {
        Ok((data, status)) => {
            if matches!(status, session::SessionStatus::Closed) {
                return session::SessionStatus::Closed;
            }
            match command_factory(data.get_data()) {
                Ok((result, cmd)) => {
                    return run_command(session, result, cmd);
                },
                Err(_) => {
                    let message = "error\n\n";
                    session.write(packet::new_packet(Some(message.as_bytes().to_vec())));
                    return session::SessionStatus::Open;
                },
            }

        }
        Err(status) => {
            match status {
                session::SessionStatus::Error => panic!("Error while reading"),
                _ => panic!()
            }
        }
    }
}

fn run_command(session: &mut impl session::Session, request: Value, cmd: fn(Value) -> Result<Value>) -> session::SessionStatus {
    match cmd(request) {
        Ok(result) => {
            let mut message = result.to_string();
            message.push_str("\n\n");
            session.write(packet::new_packet(Some(message.as_bytes().to_vec())));
            return session::SessionStatus::Open;
        },
        Err(_) => {
            let message = "error\n\n";
            session.write(packet::new_packet(Some(message.as_bytes().to_vec())));
            return session::SessionStatus::Open;
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    pub struct MockSession {
        pub write_data: packet::Packet,
        pub read_return_data: String,
        pub read_return_status: session::SessionStatus
    }

    impl MockSession {
        pub fn new() -> Self {
            return MockSession { write_data: packet::new_packet(None), read_return_status: session::SessionStatus::Error, read_return_data: "".to_string() }
        }

        pub fn set_read_return(&mut self, sample_data: String, session_status: session::SessionStatus) {
            self.read_return_data = sample_data;
            self.read_return_status = session_status;
        } 
    } 

    impl session::Session for MockSession {
        fn write(&mut self, write_data: packet::Packet) -> session::SessionStatus {
            self.write_data = write_data;
            return session::SessionStatus::Open;
        }

        fn read(&mut self) -> core::result::Result<(packet::Packet, session::SessionStatus), session::SessionStatus> {
            if matches!(self.read_return_status, session::SessionStatus::Error) {
                return Err(session::SessionStatus::Error);
            }
            
            let sample_data = self.read_return_data.as_bytes();
            return Ok((packet::new_packet(Some(sample_data.to_vec())), self.read_return_status))
        }
    }

    #[test]
    fn read_write_close_successfully() {
        let mut mock_session = MockSession::new();
        let command = json!({
            "command": "where"
        });
        let expected = "{\"result\":\"run where\"}\n\n";
        mock_session.set_read_return(command.to_string(), session::SessionStatus::Open);
    
        super::handle_session(&mut mock_session, |_a| {
            return Ok((json!("{}"), |_a| { 
                serde_json::from_str(r#"
                {
                    "result": "run where"
                }"#)
            }))
        });
    
        match mock_session.write_data.get_data() {
            Some(v) => {
                assert_eq!(String::from_utf8(v).unwrap(), expected.to_string());
            }
            None => assert!(false, "A was not read, or write too"),
        }
    }
    
    
    #[test]
    fn read_close_successfully() {
        let mut mock_session = MockSession::new();
        mock_session.set_read_return("".to_string(), session::SessionStatus::Closed);
    
        super::handle_session(&mut mock_session, |_a| {
            return Ok((json!("{}"), |_a| { 
                serde_json::from_str(r#"{}"#)
            }))
        });
    
        match mock_session.write_data.get_data() {
            Some(_) => {
                assert!(false, "Did not expect write")
            }
            None => assert!(true),
        }
    }
}