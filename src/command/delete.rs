use serde_json::{Value, Result};

pub fn delete_command(data: Value) -> Result<Value> {
    return serde_json::from_str(r#"
    {
        "result": "run delete"
    }"#)
}