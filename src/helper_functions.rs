use serde_json::Value;
use crate::{SignalKGetError, V1NumberValue};

pub fn json_as_optional_string(value: &Value) -> Option<String> {
    if let serde_json::Value::String(ref string) = value {
        Some(string.to_string())
    } else {
        None
    }
}

pub fn get_f64_value(value: &Option<V1NumberValue>) -> Result<f64, SignalKGetError> {
    if let Some(ref number_value) = value {
        if let Some(value) = number_value.value {
            Ok(value)
        } else {
            Err(SignalKGetError::ValueNotSet)
        }
    } else {
        Err(SignalKGetError::ValueNotSet)
    }
}
