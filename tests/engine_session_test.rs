use simple_base::engine::engine_session;

mod mock_session;

#[test]
fn read_write_close_successfully() {
    let mut mock_session = mock_session::new_mock_session();

    engine_session::handle_session(&mut mock_session);

    match mock_session.write_data.get_data() {
        Some(v) => {
            let s = String::from_utf8(v).unwrap();
            assert_eq!(s, "A")
        }
        None => assert!(false, "A was not read, or write too"),
    }
}
