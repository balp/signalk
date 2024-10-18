use signalk::full::V1FullFormat;
use signalk::{V1DeltaFormat, V1Discovery};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[test]
fn endpoints_sample() {
    let path = Path::new("tests/specification/test_data/discovery-valid/endpoints-sample.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1Discovery = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}
