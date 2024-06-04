use serde_json::json;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use signalk::{V1Put, V1PutValue};

fn read_signalk_from_file(path: PathBuf) -> V1Put {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1Put = serde_json::from_reader(reader).unwrap();
    sk_data
}

#[test]
fn test_put() {
    let expected = V1Put::builder()
        .request_id("c0d79334-4e25-4245-8892-54e8ccc8021d".into())
        .put(V1PutValue::new(
            "electrical.switches.anchorLight.state".into(),
            json!(1),
        ))
        .build();
    let folder = Path::new("tests/specification/examples/put/");
    let sk_data = read_signalk_from_file(folder.join("put.json"));
    assert_eq!(sk_data, expected);
}

#[test]
fn test_put_with_context() {
    let expected = V1Put::builder()
        .request_id("c0d79334-4e25-4245-8892-54e8ccc8021d".into())
        .context("vessels.urn:mrn:signalk:uuid:6b0e776f-811a-4b35-980e-b93405371bc5".into())
        .put(V1PutValue::new(
            "electrical.switches.anchorLight.state".into(),
            json!(1),
        ))
        .build();
    let folder = Path::new("tests/specification/examples/put/");
    let sk_data = read_signalk_from_file(folder.join("put-with-context.json"));
    assert_eq!(sk_data, expected);
}
