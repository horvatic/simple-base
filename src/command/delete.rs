use serde_json::{Value, Result};

use std::fs;

use crate::command::command_path;

pub fn delete_command(data: Value) -> Result<Value> {
    match data[command_path::ID_KEY].as_str() {
        Some(id) => {
            match fs::remove_file(command_path::DIR.to_string() + "/" + id) {
                Ok(_) => return serde_json::from_str(r#"
                    {
                        "result": "record deleted"
                    }"#),
                Err(_) => return serde_json::from_str(r#"
                {
                    "result": "error running delete"
                }"#),
            }
        }
        None => return serde_json::from_str(r#"
        {
            "result": "error running delete"
        }"#),
    }
}