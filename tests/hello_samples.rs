use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use signalk_rserver::signalk::hello::V1Hello;

trait OptionExt {
    type Value;
    fn unwrap_ref(&self) -> &Self::Value;
}

impl<T> OptionExt for Option<T> {
    type Value = T;
    fn unwrap_ref(&self) -> &T {
        self.as_ref().unwrap()
    }
}

fn read_signalk_from_file(path: PathBuf) -> V1Hello {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1Hello = serde_json::from_reader(reader).unwrap();
    sk_data
}

#[test]
fn test_hello() {
    let expected = V1Hello::builder()
        .name("foobar marine server".into())
        .version("1.0.4".into())
        .timestamp("2018-06-21T15:09:16.704Z".into())
        .self_("vessels.urn:mrn:signalk:uuid:c0d79334-4e25-4245-8892-54e8ccc8021d".into())
        .role("master".into())
        .role("main".into())
        .build();
    let folder = Path::new("tests/specification/examples/hello/");
    let sk_data = read_signalk_from_file(folder.join("docs-hello.json"));
    assert_eq!(sk_data, expected);
}

#[test]
fn test_hello_minimal() {
    let expected = V1Hello::builder()
        .version("1.0.2".into())
        .role("slave".into())
        .build();
    let folder = Path::new("tests/specification/examples/hello/");
    let sk_data = read_signalk_from_file(folder.join("docs-hello-minimal.json"));
    assert_eq!(sk_data, expected);
}

#[test]
fn test_hello_playback() {
    let expected = V1Hello::builder()
        .name("foobar marine server".into())
        .version("1.1.4".into())
        .start_time("2018-08-24T15:19:09Z".into())
        .playback_rate(1.0)
        .self_("vessels.urn:mrn:signalk:uuid:c0d79334-4e25-4245-8892-54e8ccc8021d".into())
        .role("master".into())
        .role("main".into())
        .build();
    let folder = Path::new("tests/specification/examples/hello/");
    let sk_data = read_signalk_from_file(folder.join("docs-hello-playback.json"));
    assert_eq!(sk_data, expected);
}
