use serde_json::{Value, Result};

use std::fs::File;
use std::io::prelude::*;

use crate::command::command_path;

pub fn where_command(data: Value) -> Result<Value> {
    match data[command_path::ID_KEY].as_str() {
        Some(id) => {
            match File::open(command_path::DIR.to_string() + "/" + id) {
                Ok(mut file) => {
                    let mut contents = String::new();
                    match file.read_to_string(&mut contents) {
                    Ok(_) => return serde_json::from_str(contents.as_str()),
                    Err(_) => return serde_json::from_str(r#"
                    {
                        "result": "error running where, can not find entry"
                    }"#)
                    }
                },
                Err(_) => return serde_json::from_str(r#"
                {
                    "result": "error running where, can not find entry"
                }"#),
            }
        }
        None => return serde_json::from_str(r#"
        {
            "result": "error running where"
        }"#),
    }
}