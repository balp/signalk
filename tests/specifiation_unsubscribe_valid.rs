use signalk::V1Unsubscribe;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[test]
fn subscribe_sample() {
    let path =
        Path::new("tests/specification/test_data/unsubscribe-valid/unsubscribe-from_all.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1Unsubscribe = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}
