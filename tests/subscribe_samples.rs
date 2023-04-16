use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use signalk_rserver::signalk::{V1Subscribe, V1Subscription};

fn read_signalk_from_file(path: PathBuf) -> V1Subscribe {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1Subscribe = serde_json::from_reader(reader).unwrap();
    sk_data
}

#[test]
fn test_docs_protocol_1() {
    let expected = V1Subscribe::builder()
        .context("vessels.self".into())
        .subscribe(
            V1Subscription::builder()
                .path("navigation.speedThroughWater".into())
                .period(1000)
                .format("delta".into())
                .policy("ideal".into())
                .min_period(200)
                .build(),
        )
        .subscribe(
            V1Subscription::builder()
                .path("navigation.logTrip".into())
                .period(10000)
                .build(),
        )
        .build();
    let folder = Path::new("tests/specification/examples/subscribe/");
    let sk_data = read_signalk_from_file(folder.join("docs-subscription_protocol1.json"));
    assert_eq!(sk_data, expected);
}

#[test]
fn test_docs_protocol_2() {
    let expected = V1Subscribe::builder()
        .context("vessels.self".into())
        .subscribe(
            V1Subscription::builder()
                .path("environment.depth.belowTransducer".into())
                .build(),
        )
        .subscribe(
            V1Subscription::builder()
                .path("navigation.speedThroughWater".into())
                .build(),
        )
        .build();
    let folder = Path::new("tests/specification/examples/subscribe/");
    let sk_data = read_signalk_from_file(folder.join("docs-subscription_protocol2.json"));
    assert_eq!(sk_data, expected);
}

#[test]
fn test_docs_protocol_3() {
    let expected = V1Subscribe::builder()
        .context("vessels.*".into())
        .subscribe(
            V1Subscription::builder()
                .path("navigation.position".into())
                .period(120000)
                .policy("fixed".into())
                .build(),
        )
        .subscribe(
            V1Subscription::builder()
                .path("navigation.courseOverGround".into())
                .period(120000)
                .policy("fixed".into())
                .build(),
        )
        .build();
    let folder = Path::new("tests/specification/examples/subscribe/");
    let sk_data = read_signalk_from_file(folder.join("docs-subscription_protocol3.json"));
    assert_eq!(sk_data, expected);
}

#[test]
fn test_docs_protocol_4() {
    let expected = V1Subscribe::builder()
        .context("vessels.230029970".into())
        .subscribe(
            V1Subscription::builder()
                .path("navigation.position".into())
                .min_period(60000)
                .policy("instant".into())
                .build(),
        )
        .build();
    let folder = Path::new("tests/specification/examples/subscribe/");
    let sk_data = read_signalk_from_file(folder.join("docs-subscription_protocol4.json"));
    assert_eq!(sk_data, expected);
}

#[test]
fn test_subscribe() {
    let expected = V1Subscribe::builder()
        .context("vessels.self".into())
        .subscribe(
            V1Subscription::builder()
                .path("navigation.speedThroughWater".into())
                .period(1000)
                .min_period(200)
                .format("delta".into())
                .policy("ideal".into())
                .build(),
        )
        .subscribe(
            V1Subscription::builder()
                .path("navigation.logTrip".into())
                .period(10000)
                .build(),
        )
        .build();
    let folder = Path::new("tests/specification/examples/subscribe/");
    let sk_data = read_signalk_from_file(folder.join("signalk-subscribe.json"));
    assert_eq!(sk_data, expected);
}

#[test]
fn test_subscribe_mqtt() {
    let expected = V1Subscribe::builder()
        .context("vessels.self".into())
        .websocket_connectionkey("d2f691ac-a5ed-4cb7-b361-9072a24ce6bc".into())
        .reply_to("signalk.3202a939-1681-4a74-ad4b-3a90212e4f33.vessels.self.navigation".into())
        .subscribe(
            V1Subscription::builder()
                .path("navigation.speedThroughWater".into())
                .period(1000)
                .min_period(200)
                .format("delta".into())
                .policy("ideal".into())
                .build(),
        )
        .subscribe(
            V1Subscription::builder()
                .path("navigation.logTrip".into())
                .period(10000)
                .build(),
        )
        .build();
    let folder = Path::new("tests/specification/examples/subscribe/");
    let sk_data = read_signalk_from_file(folder.join("signalk-subscribe-mqtt.json"));
    assert_eq!(sk_data, expected);
}
