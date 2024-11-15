use crate::definitions::F64Compatible;
use crate::SignalKGetError;
use log::{debug, warn};
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

pub trait Path<T> {
    fn get_path(&self, path: &[&str]) -> Result<T, SignalKGetError>;
}

pub fn get_path<T>(path: &[&str], value: &Option<&impl Path<T>>) -> Result<T, SignalKGetError> {
    debug!("get_path({:?}, ...)", path);
    if let Some((_start, elements)) = path.split_first() {
        if let Some(item) = value {
            item.get_path(elements)
        } else {
            warn!("Value not set for path {:?}", path);
            Err(SignalKGetError::ValueNotSet)
        }
    } else {
        Err(SignalKGetError::NoSuchPath)
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

#[cfg(test)]
mod context_tests {
    use crate::full::V1FullFormat;
    use crate::helper_functions::get_path;
    use std::fs::File;
    use std::io::BufReader;
    use std::path::Path;
    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }
    #[test]
    fn test_demo_230416() {
        init();

        let path = Path::new("tests/demo_data/anno_230809.json");
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let sk_data: V1FullFormat = serde_json::from_reader(reader).unwrap();
        // println!("{:?}", sk_data);
        let path: Vec<&str> = ".self.navigation.speedThroughWater".split(".").collect();
        println!("{:?}", path);
        let _m: f64 = get_path(&path, &Some(&sk_data)).expect("Failed to get path");
    }
}
