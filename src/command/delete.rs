use serde_json::{Value, Result};

use std::fs;

use crate::command::command_path;

const ID_KEY: &str = "id";

pub fn delete_command(data: Value) -> Result<Value> {
    match data[ID_KEY].as_str() {
        Some(id) => {
            match fs::remove_file(command_path::DIR.to_string() + "/" + id) {
                Ok(_) => return serde_json::from_str(r#"
                    {
                        "result": "run delete"
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