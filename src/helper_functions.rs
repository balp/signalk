use crate::definitions::{F64Compatible, V2NumberValue};
use crate::{SignalKGetError, V1NumberValue};
use serde_json::Value;

pub fn json_as_optional_string(value: &Value) -> Option<String> {
    if let serde_json::Value::String(ref string) = value {
        Some(string.to_string())
    } else {
        None
    }
}

pub fn get_f64_value(value: &Option<impl F64Compatible>) -> Result<f64, SignalKGetError> {
    if let Some(ref number_value) = value {
        if let Some(value) = number_value.as_f64() {
            Ok(value)
        } else {
            Err(SignalKGetError::ValueNotSet)
        }
    } else {
        Err(SignalKGetError::ValueNotSet)
    }
}
