use serde_json::{json, Value, Result};
use crate::command;

const COMMAND_KEY: &str = "command";
const WHERE_KEY: &str = "where";
const DELETE_KEY: &str = "delete";
const INSERT_KEY: &str = "insert";
const UPDATE_KEY: &str = "update";

pub fn build(data: Option<Vec<u8>>) -> Result<(Value, fn(Value) -> Result<Value>)> {
    match data {
        Some(v) => {
                let json_data: Value = serde_json::from_str(String::from_utf8(v).unwrap().as_str())?;
                match json_data[COMMAND_KEY].as_str() {
                Some(command) => {
                    if command == WHERE_KEY {
                        return Ok((json_data, command::where_comm::where_command));
                    } else if command == DELETE_KEY {
                        return Ok((json_data, command::delete::delete_command));
                    } else if command == INSERT_KEY {
                        return Ok((json_data, command::insert::insert_command));
                    } else if command == UPDATE_KEY {
                        return Ok((json_data, command::update::update_command));
                    } else {
                        return Ok((json_data, command::unknown::unknown_command));
                    }
                },
                None => return Ok((json!("{}"), command::unknown::unknown_command))
            }
        },
        None => return Ok((json!("{}"), command::unknown::unknown_command))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_where_successfully() {
        let command = json!({
            "command": "where"
        });
        let expected = "{\"command\":\"where\"}";
    
        match super::build(Some(command.to_string().as_bytes().to_vec())) {
            Ok((result, cmd)) => {
                assert_eq!(result.to_string(), expected.to_string());
                if cmd == command::where_comm::where_command {
                    assert!(true)
                } else {
                    assert!(false, "Did not return where command")
                }
            },
            Err(_) => {
                assert!(false, "Could not build where")
            }
        }
    }

    #[test]
    fn build_delete_successfully() {
        let command = json!({
            "command": "delete"
        });
        let expected = "{\"command\":\"delete\"}";
    
        match super::build(Some(command.to_string().as_bytes().to_vec())) {
            Ok((result, cmd)) => {
                assert_eq!(result.to_string(), expected.to_string());
                if cmd == command::delete::delete_command {
                    assert!(true)
                } else {
                    assert!(false, "Did not return delete command")
                }
            },
            Err(_) => {
                assert!(false, "Could not build delete")
            }
        }
    }

    #[test]
    fn build_insert_successfully() {
        let command = json!({
            "command": "insert"
        });
        let expected = "{\"command\":\"insert\"}";
    
        match super::build(Some(command.to_string().as_bytes().to_vec())) {
            Ok((result, cmd)) => {
                assert_eq!(result.to_string(), expected.to_string());
                if cmd == command::insert::insert_command {
                    assert!(true)
                } else {
                    assert!(false, "Did not return insert command")
                }
            },
            Err(_) => {
                assert!(false, "Could not build insert")
            }
        }
    }

    #[test]
    fn build_unknown_successfully() {
        let command = json!({
            "command": "wefwefefwefwefew"
        });
        let expected = "{\"command\":\"wefwefefwefwefew\"}";
    
        match super::build(Some(command.to_string().as_bytes().to_vec())) {
            Ok((result, cmd)) => {
                assert_eq!(result.to_string(), expected.to_string());
                if cmd == command::unknown::unknown_command {
                    assert!(true)
                } else {
                    assert!(false, "Did not return unknown command")
                }
            },
            Err(_) => {
                assert!(false, "Could not build unknown")
            }
        }
    }

    #[test]
    fn build_update_successfully() {
        let command = json!({
            "command": "update"
        });
        let expected = "{\"command\":\"update\"}";
    
        match super::build(Some(command.to_string().as_bytes().to_vec())) {
            Ok((result, cmd)) => {
                assert_eq!(result.to_string(), expected.to_string());
                if cmd == command::update::update_command {
                    assert!(true)
                } else {
                    assert!(false, "Did not return update command")
                }
            },
            Err(_) => {
                assert!(false, "Could not build update")
            }
        }
    }
}