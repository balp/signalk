use crate::definitions::F64Compatible;
use crate::SignalKGetError;
use log::debug;
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
        if let Some(f64_value) = number_value.as_f64() {
            Ok(f64_value)
        } else {
            Err(SignalKGetError::ValueNotSet)
        }
    } else {
        Err(SignalKGetError::ValueNotSet)
    }
}

pub trait F64CompatiblePath {
    fn get_f64_for_path(&self, path: &mut Vec<&str>) -> Result<f64, SignalKGetError>;
}

pub fn get_f64_value_for_path(
    path: &mut Vec<&str>,
    value: &Option<impl F64CompatiblePath>,
) -> Result<f64, SignalKGetError> {
    debug!("get_f64_value_for_path({:?}, ...)", path);
    if let Some(ref item) = value {
        path.remove(0);
        item.get_f64_for_path(path)
    } else {
        Err(SignalKGetError::ValueNotSet)
    }
}
