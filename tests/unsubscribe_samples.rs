use serde_json::json;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use signalk_rserver::signalk::{V1Unsubscribe, V1Unsubscription};

fn read_signalk_from_file(path: PathBuf) -> V1Unsubscribe {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1Unsubscribe = serde_json::from_reader(reader).unwrap();
    sk_data
}

#[test]
fn test_docs_protocol() {
    let expected = V1Unsubscribe::builder()
        .context("*".into())
        .unsubscribe(V1Unsubscription::builder().path("*".into()).build())
        .build();
    let folder = Path::new("tests/specification/examples/unsubscribe/");
    let sk_data = read_signalk_from_file(folder.join("docs-subscription_protocol.json"));
    assert_eq!(sk_data, expected);
}

#[test]
fn test_unsubscribe() {
    let expected = V1Unsubscribe::builder()
        .context("vessels.self".into())
        .unsubscribe(
            V1Unsubscription::builder()
                .path("navigation.speedThroughWater".into())
                .period(1000)
                .format("delta".into())
                .policy("ideal".into())
                .min_period(200)
                .build(),
        )
        .unsubscribe(
            V1Unsubscription::builder()
                .path("navigation.logTrip".into())
                .period(10000)
                .build(),
        )
        .build();
    let folder = Path::new("tests/specification/examples/unsubscribe/");
    let sk_data = read_signalk_from_file(folder.join("signalk-unsubscribe.json"));
    assert_eq!(sk_data, expected);
}

#[test]
fn test_unsubscribe_mqtt() {
    let expected = V1Unsubscribe::builder()
        .context("vessels.self".into())
        .websocket_connectionkey("d2f691ac-a5ed-4cb7-b361-9072a24ce6bc".into())
        .unsubscribe(
            V1Unsubscription::builder()
                .path("navigation.speedThroughWater".into())
                .period(1000)
                .format("delta".into())
                .policy("ideal".into())
                .min_period(200)
                .build(),
        )
        .unsubscribe(
            V1Unsubscription::builder()
                .path("navigation.logTrip".into())
                .period(10000)
                .build(),
        )
        .build();
    let folder = Path::new("tests/specification/examples/unsubscribe/");
    let sk_data = read_signalk_from_file(folder.join("signalk-unsubscribe-mqtt.json"));
    assert_eq!(sk_data, expected);
}
