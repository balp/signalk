use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use serde_json::{json, Number, Value};

use signalk::signalk::delta::{V1DeltaFormat, V1UpdateMeta, V1UpdateType, V1UpdateValue};
use signalk::signalk::{V1DefSource, V1Meta, V1MetaZone};

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

fn read_signalk_from_file(path: PathBuf) -> V1DeltaFormat {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1DeltaFormat = serde_json::from_reader(reader).unwrap();
    sk_data
}

#[test]
fn test_0183_rmc_export_delta() {
    let folder = Path::new("tests/specification/examples/delta/");
    let sk_data = read_signalk_from_file(folder.join("0183-RMC-export-delta.json"));
    let expected = V1DeltaFormat::builder()
        .context("vessels.366982330.navigation".into())
        .add_update(
            V1UpdateType::builder()
                .add(V1UpdateValue::new(
                    "position.timestamp".into(),
                    serde_json::value::Value::String("2015-03-07T12:37:10.523+13:00".into()),
                ))
                .add(V1UpdateValue::new(
                    "position.longitude".into(),
                    serde_json::value::Value::Number(Number::from_f64(173.1693).unwrap()),
                ))
                .add(V1UpdateValue::new(
                    "position.latitude".into(),
                    serde_json::value::Value::Number(Number::from_f64(-41.156426).unwrap()),
                ))
                .add(V1UpdateValue::new(
                    "position.source".into(),
                    serde_json::value::Value::String("sources.gps_0183_RMC".into()),
                ))
                .add(V1UpdateValue::new(
                    "position.altitude".into(),
                    serde_json::value::Value::Number(Number::from(0)),
                ))
                .add(V1UpdateValue::new(
                    "courseOverGroundTrue".into(),
                    serde_json::value::Value::Number(Number::from_f64(245.69).unwrap()),
                ))
                .ref_source("sources.gps_0183_RMC".into())
                .build(),
        )
        .build();
    assert_eq!(sk_data, expected)
}

#[test]
fn test_0183_rmc_export_min_delta() {
    let folder = Path::new("tests/specification/examples/delta/");
    let sk_data = read_signalk_from_file(folder.join("0183-RMC-export-min-delta.json"));
    let expected = V1DeltaFormat::builder()
        .context("vessels.366982330.navigation".into())
        .add_update(
            V1UpdateType::builder()
                .add(V1UpdateValue::new(
                    "position.longitude".into(),
                    serde_json::value::Value::Number(Number::from_f64(173.1693).unwrap()),
                ))
                .add(V1UpdateValue::new(
                    "position.latitude".into(),
                    serde_json::value::Value::Number(Number::from_f64(-41.156426).unwrap()),
                ))
                .add(V1UpdateValue::new(
                    "position.altitude".into(),
                    serde_json::value::Value::Number(Number::from(0)),
                ))
                .add(V1UpdateValue::new(
                    "courseOverGroundTrue".into(),
                    serde_json::value::Value::Number(Number::from_f64(245.69).unwrap()),
                ))
                .build(),
        )
        .build();
    assert_eq!(sk_data, expected)
}

#[test]
fn test_docs_data_model() {
    let folder = Path::new("tests/specification/examples/delta/");
    let sk_data = read_signalk_from_file(folder.join("docs-data_model.json"));
    let map = serde_json::Map::from_iter([(
        "name".to_string(),
        serde_json::value::Value::String("WRANGO".into()),
    )]);
    let expected = V1DeltaFormat::builder()
        .context("vessels.urn:mrn:imo:mmsi:234567890".into())
        .add_update(
            V1UpdateType::builder()
                .source(
                    V1DefSource::builder()
                        .label("N2000-01".into())
                        .type_("NMEA2000".into())
                        .src("017".into())
                        .pgn(127488)
                        .build(),
                )
                .add(V1UpdateValue::new(
                    "propulsion.0.revolutions".into(),
                    serde_json::value::Value::Number(Number::from_f64(16.341667).unwrap()),
                ))
                .add(V1UpdateValue::new(
                    "propulsion.0.boostPressure".into(),
                    serde_json::value::Value::Number(Number::from_f64(45500.0).unwrap()),
                ))
                .timestamp("2010-01-07T07:18:44Z".into())
                .build(),
        )
        .add_update(
            V1UpdateType::builder()
                .source(
                    V1DefSource::builder()
                        .label("N2000-01".into())
                        .type_("NMEA2000".into())
                        .src("115".into())
                        .pgn(128267)
                        .build(),
                )
                .add(V1UpdateValue::new(
                    "navigation.courseOverGroundTrue".into(),
                    serde_json::value::Value::Number(Number::from_f64(2.971).unwrap()),
                ))
                .add(V1UpdateValue::new(
                    "navigation.speedOverGround".into(),
                    serde_json::value::Value::Number(Number::from_f64(3.85).unwrap()),
                ))
                .timestamp("2014-08-15T16:00:00.081Z".into())
                .build(),
        )
        .add_update(
            V1UpdateType::builder()
                .source(
                    V1DefSource::builder()
                        .label("N2000-01".into())
                        .type_("NMEA2000".into())
                        .src("115".into())
                        .pgn(128267)
                        .build(),
                )
                .add(V1UpdateValue::new(
                    "".into(),
                    serde_json::value::Value::Object(map),
                ))
                .timestamp("2014-08-15T19:02:31.507Z".into())
                .build(),
        )
        .build();
    assert_eq!(sk_data, expected)
}

#[test]
fn test_docs_data_model_meta_deltas() {
    let folder = Path::new("tests/specification/examples/delta/");
    let sk_data = read_signalk_from_file(folder.join("docs-data_model_meta_deltas.json"));
    let expected = V1DeltaFormat::builder()
        .add_update(
            V1UpdateType::builder()
                .meta(V1UpdateMeta::new(
                    "environment.wind.speedApparent".into(),
                    V1Meta::builder()
                        .description("Apparent wind speed".into())
                        .display_name("Apparent Wind Speed".into())
                        .short_name("AWS".into())
                        .units("m/s".into())
                        .zones(
                            V1MetaZone::builder()
                                .upper(15.4333)
                                .state("warn".into())
                                .message("high wind speed".into())
                                .build(),
                        )
                        .build(),
                ))
                .timestamp("2014-08-15T19:02:31.507Z".into())
                .build(),
        )
        .context("vessels.urn:mrn:imo:mmsi:234567890".into())
        .build();
    assert_eq!(sk_data, expected)
}

#[test]
fn test_docs_data_model_multiple_values() {
    let folder = Path::new("tests/specification/examples/delta/");
    let sk_data = read_signalk_from_file(folder.join("docs-data_model_multiple_values.json"));
    let expected = V1DeltaFormat::builder()
        .context("vessels.urn:mrn:signalk:uuid:c0d79334-4e25-4245-8892-54e8ccc8021d".into())
        .add_update(
            V1UpdateType::builder()
                .timestamp("2017-04-03T06:14:04.451Z".into())
                .source(
                    V1DefSource::builder()
                        .label("GPS-1".into())
                        .type_("NMEA0183".into())
                        .talker("GP".into())
                        .sentence("RMC".into())
                        .build(),
                )
                .add(V1UpdateValue::new(
                    "navigation.courseOverGroundTrue".into(),
                    Value::Number(Number::from_f64(3.615624078431440).unwrap()),
                ))
                .build(),
        )
        .add_update(
            V1UpdateType::builder()
                .timestamp("2017-04-03T06:14:04.451Z".into())
                .source(
                    V1DefSource::builder()
                        .label("actisense".into())
                        .type_("NMEA2000".into())
                        .src("115".into())
                        .pgn(128267)
                        .build(),
                )
                .add(V1UpdateValue::new(
                    "navigation.courseOverGroundTrue".into(),
                    Value::Number(Number::from_f64(3.615624078431453).unwrap()),
                ))
                .build(),
        )
        .build();
    assert_eq!(sk_data, expected)
}

#[test]
fn test_docs_notifications() {
    let folder = Path::new("tests/specification/examples/delta/");
    let sk_data = read_signalk_from_file(folder.join("docs-notifications.json"));
    let expected = V1DeltaFormat::builder()
        .context("vessels.urn:mrn:signalk:uuid:c0d79334-4e25-4245-8892-54e8ccc8021d".into())
        .add_update(
            V1UpdateType::builder()
                .timestamp("2017-08-15T16:00:05.200Z".into())
                .source(
                    V1DefSource::builder()
                        .label("ttyUSB0".into())
                        .type_("NMEA0183".into())
                        .talker("GP".into())
                        .sentence("MOB".into())
                        .build(),
                )
                .add(V1UpdateValue::new(
                    "notifications.mob".into(),
                    json!({
                        "message": "MOB",
                        "state": "emergency",
                        "method": ["visual", "sound"],
                    }),
                ))
                .build(),
        )
        .add_update(
            V1UpdateType::builder()
                .timestamp("2017-08-15T16:00:05.538Z".into())
                .source(
                    V1DefSource::builder()
                        .label("ttyUSB0".into())
                        .type_("NMEA0183".into())
                        .talker("GP".into())
                        .sentence("MOB".into())
                        .build(),
                )
                .add(V1UpdateValue::new("notifications.mob".into(), Value::Null))
                .build(),
        )
        .build();
    assert_eq!(sk_data, expected)
}

#[test]
fn test_docs_subscription_protocol() {
    let folder = Path::new("tests/specification/examples/delta/");
    let sk_data = read_signalk_from_file(folder.join("docs-subscription_protocol.json"));
    let expected = V1DeltaFormat::builder()
        .context("vessels.urn:mrn:imo:mmsi:234567890".into())
        .add_update(
            V1UpdateType::builder()
                .source(
                    V1DefSource::builder()
                        .label("N2000-01".into())
                        .type_("NMEA2000".into())
                        .src("115".into())
                        .pgn(128275)
                        .build(),
                )
                .add(V1UpdateValue::new(
                    "navigation.trip.log".into(),
                    json!(43374),
                ))
                .add(V1UpdateValue::new("navigation.log".into(), json!(17404540)))
                .build(),
        )
        .build();
    assert_eq!(sk_data, expected)
}

#[test]
fn test_mob_alarm_delta() {
    let folder = Path::new("tests/specification/examples/delta/");
    let sk_data = read_signalk_from_file(folder.join("MOB-alarm-delta.json"));
    let expected = V1DeltaFormat::builder()
        .context("vessels.urn:mrn:signalk:uuid:c0d79334-4e25-4245-8892-54e8ccc8021d".into())
        .add_update(
            V1UpdateType::builder()
                .source(
                    V1DefSource::builder()
                        .label("ttyUSB0".into())
                        .type_("NMEA0183".into())
                        .talker("GP".into())
                        .sentence("MOB".into())
                        .build(),
                )
                .timestamp("2014-08-15T16:00:05.538Z".into())
                .add(V1UpdateValue::new(
                    "notifications.mob".into(),
                    json!({
                        "message": "MOB",
                        "state": "emergency",
                        "method": ["visual", "sound"],
                    }),
                ))
                .build(),
        )
        .build();
    assert_eq!(sk_data, expected)
}
