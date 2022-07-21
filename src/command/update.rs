use serde_json::{Value, Result};

use std::fs::OpenOptions;
use std::io::prelude::*;

use crate::command::command_path;

pub fn update_command(data: Value) -> Result<Value> {
    match data[command_path::ID_KEY].as_str() {
        Some(id) => {
            match OpenOptions::new().write(true).truncate(true).open(command_path::DIR.to_string() + "/" + id) {
                Ok(mut file) => {
                    match file.write_all(data[command_path::DATA_KEY].to_string().as_bytes()) {
                        Ok(_) => { 
                            let message = format!("{{\"result\": \"id {}\"}}", id.to_string().as_str());
                            return serde_json::from_str(message.as_str())},
                        Err(_) => return serde_json::from_str(r#"
                        {
                            "result": "error running update"
                        }"#),
                    }
                },
                Err(_) => return serde_json::from_str(r#"
                {
                    "result": "error running update, can not find entry"
                }"#),
            }
        }
        None => return serde_json::from_str(r#"
        {
            "result": "error running update"
        }"#),
    }
}