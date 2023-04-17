use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use signalk::{V1Discovery, V1DiscoveryEndpoint, V1DiscoveryServer};

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

fn read_signalk_from_file(path: PathBuf) -> V1Discovery {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1Discovery = serde_json::from_reader(reader).unwrap();
    sk_data
}

#[test]
fn test_discovery() {
    let folder = Path::new("tests/specification/examples/discovery/");
    let sk_data = read_signalk_from_file(folder.join("discovery.json"));

    let expected = V1Discovery::builder()
        .server(V1DiscoveryServer::new(
            "signalk-server-node".into(),
            "0.1.33".into(),
        ))
        .endpoint(
            "v1".into(),
            V1DiscoveryEndpoint::builder()
                .version("1.0.0".into())
                .signalk_http("http://localhost:3000/signalk/v1/api/".into())
                .signalk_ws("ws://localhost:3000/signalk/v1/stream".into())
                .build(),
        )
        .endpoint(
            "v3".into(),
            V1DiscoveryEndpoint::builder()
                .version("3.0.0-Alpha".into())
                .signalk_http("http://localhost/signalk/v3/api/".into())
                .signalk_ws("ws://localhost/signalk/v3/stream".into())
                .signalk_tcp("tcp://localhost:8367".into())
                .build(),
        )
        .build();
    assert_eq!(sk_data, expected);
}

#[test]
fn test_docs_rest_api() {
    let folder = Path::new("tests/specification/examples/discovery/");
    let sk_data = read_signalk_from_file(folder.join("docs-rest_api.json"));

    let expected = V1Discovery::builder()
        .server(V1DiscoveryServer::new(
            "signalk-server-node".into(),
            "0.1.33".into(),
        ))
        .endpoint(
            "v1".into(),
            V1DiscoveryEndpoint::builder()
                .version("1.0.0-alpha1".into())
                .signalk_http("http://localhost:3000/signalk/v1/api/".into())
                .signalk_ws("ws://localhost:3000/signalk/v1/stream".into())
                .build(),
        )
        .endpoint(
            "v3".into(),
            V1DiscoveryEndpoint::builder()
                .version("3.0.0".into())
                .signalk_http("http://localhost/signalk/v3/api/".into())
                .signalk_ws("ws://localhost/signalk/v3/stream".into())
                .signalk_tcp("tcp://localhost:8367".into())
                .build(),
        )
        .build();
    assert_eq!(sk_data, expected);
}
