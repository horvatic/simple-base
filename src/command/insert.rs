use serde_json::{Value, Result};

pub fn insert_command(_data: Value) -> Result<Value> {
    return serde_json::from_str(r#"
    {
        "result": "run insert"
    }"#)
}