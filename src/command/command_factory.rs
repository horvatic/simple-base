use serde_json::{json, Value, Result};
use crate::command;

pub fn build(data: Option<Vec<u8>>) -> Result<(Value, fn(Value) -> Result<Value>)> {
    match data {
        Some(v) => {
            let json_data: Value = serde_json::from_str(String::from_utf8(v).unwrap().as_str())?;
            let command = json_data["command"].as_str().unwrap();
            if command == "where" {
                return Ok((json_data, command::where_comm::where_command));
            } else if command == "delete" {
                return Ok((json_data, command::delete::delete_command));
            } else if command == "insert" {
                return Ok((json_data, command::insert::insert_command));
            } else if command == "update" {
                return Ok((json_data, command::update::update_command));
            } else {
                return Ok((json_data, command::unknown::unknown_command));
            }
        },
        None => return Ok((json!("{}"), command::unknown::unknown_command))
    }
}