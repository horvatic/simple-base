use serde_json::{Value, Result};

pub fn unknown_command(data: Value) -> Result<Value> {
    return serde_json::from_str(r#"
    {
        "result": "command not found"
    }"#)
}