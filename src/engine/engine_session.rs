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
