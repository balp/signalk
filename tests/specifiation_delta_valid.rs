use signalk::V1DeltaFormat;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[test]
fn delta_object_with_only_an_empty_updates_validates() {
    let path = Path::new("tests/specification/test_data/delta-valid/delta-object_with_only_an_empty_updates_validates.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1DeltaFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
fn delta_simple() {
    let path = Path::new("tests/specification/test_data/delta-valid/delta-simple.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1DeltaFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
fn delta_without_context_source_and_timestamp_is_valid() {
    let path = Path::new("tests/specification/test_data/delta-valid/delta-without_context_source_and_timestamp_is_valid.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1DeltaFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
fn identity_blank_ie_self() {
    let path = Path::new("tests/specification/test_data/delta-valid/identity-blank_ie_self.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1DeltaFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
fn identity_mmsi() {
    let path = Path::new("tests/specification/test_data/delta-valid/identity-mmsi.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1DeltaFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
fn identity_url() {
    let path = Path::new("tests/specification/test_data/delta-valid/identity-url.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1DeltaFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
fn identity_uuid() {
    let path = Path::new("tests/specification/test_data/delta-valid/identity-uuid.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1DeltaFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
fn meta_and_values() {
    let path = Path::new("tests/specification/test_data/delta-valid/meta-and-values.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1DeltaFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
fn meta_delta() {
    let path = Path::new("tests/specification/test_data/delta-valid/meta-delta.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1DeltaFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
fn value_can_be_boolean() {
    let path = Path::new("tests/specification/test_data/delta-valid/value-can_be_boolean.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1DeltaFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
fn value_can_be_null() {
    let path = Path::new("tests/specification/test_data/delta-valid/value-can_be_null.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1DeltaFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
fn value_can_be_number() {
    let path = Path::new("tests/specification/test_data/delta-valid/value-can_be_number.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1DeltaFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
fn value_can_be_object() {
    let path = Path::new("tests/specification/test_data/delta-valid/value-can_be_object.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1DeltaFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}
