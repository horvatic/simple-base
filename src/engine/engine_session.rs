use crate::network::session;

pub struct EngineSession {
    session: Box<dyn session::Session>
}

impl EngineSession {
    pub fn new(session: Box<dyn session::Session>) -> Self {
        EngineSession { session }
    }

    pub fn handle_session(&mut self) {
        match self.session.read() {
            Ok((data, status)) => {
                if matches!(status, session::SessionStatus::Closed) {
                    return ;
                }
                self.session.write(data);
            }
            Err(status) => {
                match status {
                    session::SessionStatus::Error => print!("Error while reading"),
                    _ => panic!()
                }
            }
        }
    }
}
