use serde_json::{json, Value};

pub fn process(data: Option<Vec<u8>>) -> Value {
    match data {
        Some(v) => {
            let json_data = json!(String::from_utf8(v).unwrap());
            return json_data;
        },
        None => return json!("{}".to_string())
    }
}