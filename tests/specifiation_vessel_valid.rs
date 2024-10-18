use signalk::V1Vessel;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[test]
fn environment_sample() {
    let path = Path::new("tests/specification/test_data/vessel-valid/environment-sample.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1Vessel = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
fn environment_with_inside_heat_index_temperature() {
    let path = Path::new("tests/specification/test_data/vessel-valid/environment-with_inside_heatIndexTemperature.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1Vessel = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
fn meta_missing_units() {
    let path = Path::new("tests/specification/test_data/vessel-valid/meta-missing-units.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1Vessel = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
fn meta_with_enum() {
    let path = Path::new("tests/specification/test_data/vessel-valid/meta-with-enum.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1Vessel = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
fn meta_with_with_display_scale() {
    let path = Path::new("tests/specification/test_data/vessel-valid/meta-with_displayScale.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1Vessel = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
fn meta_with_with_display_scale_type() {
    let path =
        Path::new("tests/specification/test_data/vessel-valid/meta-with_displayScale_type.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1Vessel = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
fn meta_with_with_display_scale_type_power() {
    let path = Path::new(
        "tests/specification/test_data/vessel-valid/meta-with_displayScale_type_power.json",
    );
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1Vessel = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
fn polar_data() {
    let path = Path::new("tests/specification/test_data/vessel-valid/polar-data.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1Vessel = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
fn propulsion_sample() {
    let path = Path::new("tests/specification/test_data/vessel-valid/propulsion-sample.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1Vessel = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
fn sails_sample() {
    let path = Path::new("tests/specification/test_data/vessel-valid/sails-sample.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1Vessel = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
fn steering_sample() {
    let path = Path::new("tests/specification/test_data/vessel-valid/steering-sample.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1Vessel = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
fn tanks_sample() {
    let path = Path::new("tests/specification/test_data/vessel-valid/tanks-sample.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1Vessel = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
fn vessel_just_mmsi_identifier() {
    let path =
        Path::new("tests/specification/test_data/vessel-valid/vessel-just_mmsi_identifier.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1Vessel = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
fn vessel_just_uuid_identifier() {
    let path =
        Path::new("tests/specification/test_data/vessel-valid/vessel-just_uuid_identifier.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1Vessel = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
fn vessel_uuid_and_mmsi_identifiers() {
    let path = Path::new(
        "tests/specification/test_data/vessel-valid/vessel-uuid_and_mmsi_identifiers.json",
    );
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1Vessel = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}
