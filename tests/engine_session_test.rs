use simple_base::engine::engine_session;

mod mock_session;

#[test]
fn read_write_close_successfully() {
    let mock_session = mock_session::new_mock_session();
    let session = Box::new(mock_session);
    let mut engine_user_session = engine_session::EngineSession::new(session);

    engine_user_session.handle_session();

    
}
