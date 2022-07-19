use serde_json::{Value, Result};

pub fn update_command(_data: Value) -> Result<Value> {
    return serde_json::from_str(r#"
    {
        "result": "can not run update. Please use where, delete, insert"
    }"#)
}