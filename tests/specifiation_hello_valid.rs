use signalk::V1Hello;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[test]
fn master() {
    let path = Path::new("tests/specification/test_data/hello-valid/master.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1Hello = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
fn slave_minimal() {
    let path = Path::new("tests/specification/test_data/hello-valid/slave_minimal.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1Hello = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}
