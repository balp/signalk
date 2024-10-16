use signalk::full::V1FullFormat;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[test]
fn test_demo_230416() {
    let path = Path::new("tests/demo_data/demo_2_10_n2k_with_nav.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1FullFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}
