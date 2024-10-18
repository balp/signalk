use signalk::V1Put;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[test]
fn delta_put_array() {
    let path = Path::new("tests/specification/test_data/put-valid/delta-put-array.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1Put = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}
