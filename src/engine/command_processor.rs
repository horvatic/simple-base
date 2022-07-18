use serde_json::{Value, Result};

pub fn process(data: Option<Vec<u8>>) -> Result<Value> {
    match data {
        Some(v) => {
            let json_data: Value = serde_json::from_str(String::from_utf8(v).unwrap().as_str())?;
            let command = json_data["command"].as_str().unwrap();
            if command == "where" {
                return serde_json::from_str(r#"
                {
                    "result": "run where"
                }"#)
            }
            return serde_json::from_str(r#"
            {
                "result": "command not found"
            }"#)
        },
        None => return serde_json::from_str("{}")
    }
}