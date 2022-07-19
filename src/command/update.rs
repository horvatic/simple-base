use serde_json::{Value, Result};

pub fn update_command(data: Value) -> Result<Value> {
    return serde_json::from_str(r#"
    {
        "result": "run update"
    }"#)
}