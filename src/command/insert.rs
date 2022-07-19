use std::fs::File;
use std::io::prelude::*;

use serde_json::{Value, Result};
use uuid::Uuid;

use crate::command::command_path;

const DATA_KEY: &str = "data";

pub fn insert_command(data: Value) -> Result<Value> {
    let id = Uuid::new_v4();
    let path = command_path::DIR.to_string() + "/" + id.to_string().as_str();
    match File::create(path) {
        Ok(mut file) => {
            match file.write_all(data[DATA_KEY].to_string().as_bytes()) {
                Ok(_) =>     return serde_json::from_str(r#"
                {
                    "result": "run insert"
                }"#),
                Err(_) => return serde_json::from_str(r#"
                {
                    "result": "error running insert"
                }"#),
            }
        },
        Err(_) => return serde_json::from_str(r#"
        {
            "result": "error running insert"
        }"#),
    }



}
