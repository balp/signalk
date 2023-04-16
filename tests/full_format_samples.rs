use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use signalk::signalk::full::V1FullFormat;
use signalk::signalk::{
    V1ACBus, V1CommonValueFields, V1Electrical, V1ElectricalACQualities, V1ElectricalIdentity,
    V1Environment, V1EnvironmentDepth, V1EnvironmentTime, V1Navigation, V1Notification,
    V1NotificationValue, V1NumberValue, V1PositionType, V1PositionValue, V1Propulsion, V1Source,
    V1SourceProperty, V1Sources, V1Vessel,
};

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

fn read_signalk_from_file(path: PathBuf) -> V1FullFormat {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1FullFormat = serde_json::from_reader(reader).unwrap();
    sk_data
}

#[test]
fn test_sample_full_0183_rmc_export() {
    let expected = V1FullFormat::builder()
        .version("1.0.0".into())
        .self_("urn:mrn:imo:mmsi:366982330".into())
        .add_vessel(
            "urn:mrn:imo:mmsi:366982330".into(),
            V1Vessel::builder()
                .mmsi("366982330".into())
                .navigation(
                    V1Navigation::builder()
                        .course_over_ground_true(
                            V1NumberValue::builder()
                                .value(245.69)
                                .timestamp("2015-03-06T16:57:53.643Z".into())
                                .source("sources.gps_0183_RMC".into())
                                .build(),
                        )
                        .position(
                            V1PositionType::builder()
                                .value(V1PositionValue::new_2d(-41.156426, 173.1693))
                                .timestamp("2015-03-06T16:57:53.643Z".into())
                                .source("sources.gps_0183_RMC".into())
                                .build(),
                        )
                        .build(),
                )
                .build(),
        )
        .build();
    let folder = Path::new("tests/specification/examples/full/");
    let sk_data = read_signalk_from_file(folder.join("0183-RMC-export.json"));
    assert_eq!(sk_data, expected);
}

#[test]
fn test_sample_docs_full_example() {
    let expected = V1FullFormat::builder()
        .version("1.0.0".into())
        .self_("urn:mrn:signalk:uuid:705f5f1a-efaf-44aa-9cb8-a0fd6305567c".into())
        .add_vessel(
            "urn:mrn:signalk:uuid:705f5f1a-efaf-44aa-9cb8-a0fd6305567c".into(),
            V1Vessel::builder()
                .uuid("urn:mrn:signalk:uuid:705f5f1a-efaf-44aa-9cb8-a0fd6305567c".into())
                .name("Motu".into())
                .navigation(
                    V1Navigation::builder()
                        .speed_over_ground(
                            V1NumberValue::builder()
                                .value(4.32693662)
                                .timestamp("2017-05-16T05:15:50.007Z".into())
                                .source("ttyUSB0.GP".into())
                                .sentence("RMC".into())
                                .build(),
                        )
                        .heading_magnetic(
                            V1NumberValue::builder()
                                .value(5.55014702)
                                .timestamp("2017-05-16T05:15:54.006Z".into())
                                .source("ttyUSB0.II".into())
                                .sentence("HDM".into())
                                .build(),
                        )
                        .position(
                            V1PositionType::builder()
                                .value(V1PositionValue::new_3d(37.81479, -122.44880152, 0.0))
                                .timestamp("2017-05-16T05:15:50.007Z".into())
                                .source("ttyUSB0.GP".into())
                                .sentence("RMC".into())
                                .build(),
                        )
                        .build(),
                )
                .build(),
        )
        .sources(
            V1Sources::builder()
                .add_field(
                    "ttyUSB0".into(),
                    V1Source::builder()
                        .label("ttyUSB0".into())
                        .type_("NMEA0183".into())
                        .add_property(
                            "II".into(),
                            V1SourceProperty::builder()
                                .talker("II".into())
                                .add_sentence("HDM".into(), "2017-05-16T05:15:54.006Z".into())
                                .build(),
                        )
                        .add_property(
                            "GP".into(),
                            V1SourceProperty::builder()
                                .talker("GP".into())
                                .add_sentence("RMC".into(), "2017-04-03T06:14:04.451Z".into())
                                .build(),
                        )
                        .build(),
                )
                .build(),
        )
        .build();
    let folder = Path::new("tests/specification/examples/full/");
    let sk_data = read_signalk_from_file(folder.join("docs-full-example.json"));

    assert_eq!(sk_data, expected);
}

#[test]
fn test_sample_full_0183_rmc_export_min() {
    let expected = V1FullFormat::builder()
        .version("1.0.0".into())
        .self_("urn:mrn:imo:mmsi:366982330".into())
        .add_vessel(
            "urn:mrn:imo:mmsi:366982330".into(),
            V1Vessel::builder()
                .mmsi("230099999".into())
                .navigation(
                    V1Navigation::builder()
                        .course_over_ground_true(
                            V1NumberValue::builder()
                                .value(245.69)
                                .timestamp("2015-01-25T12:01:01Z".into())
                                .source("a.suitable.path".into())
                                .build(),
                        )
                        .position(
                            V1PositionType::builder()
                                .value(V1PositionValue::new_3d(-41.156426, 173.1693, 0.0))
                                .timestamp("2015-01-25T12:01:01Z".into())
                                .source("a.suitable.path".into())
                                .build(),
                        )
                        .build(),
                )
                .build(),
        )
        .build();
    let folder = Path::new("tests/specification/examples/full/");
    let sk_data = read_signalk_from_file(folder.join("0183-RMC-export-min.json"));

    assert_eq!(sk_data, expected);
}

#[test]
fn test_sample_full_0183_rmc_full() {
    let expected = V1FullFormat::builder()
        .version("0.1.0".into())
        .self_("urn:mrn:imo:mmsi:366982330".into())
        .add_vessel(
            "urn:mrn:imo:mmsi:366982330".into(),
            V1Vessel::builder()
                .mmsi("366982330".into())
                .navigation(
                    V1Navigation::builder()
                        .course_over_ground_true(
                            V1NumberValue::builder()
                                .value(245.69)
                                .timestamp("2015-03-06T16:57:53.643Z".into())
                                .source("sources.gps_0183_RMC".into())
                                .build(),
                        )
                        .position(
                            V1PositionType::builder()
                                .value(V1PositionValue::new_3d(-41.156426, 173.1693, 0.0))
                                .timestamp("2015-03-06T16:57:53.643Z".into())
                                .source("sources.gps_0183_RMC".into())
                                .build(),
                        )
                        .build(),
                )
                .build(),
        )
        .build();
    let folder = Path::new("tests/specification/examples/full/");
    let sk_data = read_signalk_from_file(folder.join("0183-RMC-full.json"));
    assert_eq!(sk_data, expected);
}

#[test]
fn test_sample_docs_data_model() {
    let expected = V1FullFormat::builder()
        .version("1.0.0".into())
        .self_("urn:mrn:signalk:uuid:705f5f1a-efaf-44aa-9cb8-a0fd6305567c".into())
        .add_vessel(
            "urn:mrn:signalk:uuid:705f5f1a-efaf-44aa-9cb8-a0fd6305567c".into(),
            V1Vessel::builder()
                .uuid("urn:mrn:signalk:uuid:705f5f1a-efaf-44aa-9cb8-a0fd6305567c".into())
                .name("Motu".into())
                .navigation(
                    V1Navigation::builder()
                        .speed_over_ground(
                            V1NumberValue::builder()
                                .value(4.32693662)
                                .timestamp("2017-05-16T05:15:50.007Z".into())
                                .source("ttyUSB0.GP".into())
                                .sentence("RMC".into())
                                .build(),
                        )
                        .heading_magnetic(
                            V1NumberValue::builder()
                                .value(5.55014702)
                                .timestamp("2017-05-16T05:15:54.006Z".into())
                                .source("ttyUSB0.II".into())
                                .sentence("HDM".into())
                                .build(),
                        )
                        .position(
                            V1PositionType::builder()
                                .value(V1PositionValue::new_3d(37.81479, -122.44880152, 0.0))
                                .timestamp("2017-05-16T05:15:50.007Z".into())
                                .source("ttyUSB0.GP".into())
                                .sentence("RMC".into())
                                .build(),
                        )
                        .build(),
                )
                .build(),
        )
        .sources(
            V1Sources::builder()
                .add_field(
                    "ttyUSB0".into(),
                    V1Source::builder()
                        .label("ttyUSB0".into())
                        .type_("NMEA0183".into())
                        .add_property(
                            "II".into(),
                            V1SourceProperty::builder()
                                .talker("II".into())
                                .add_sentence("HDM".into(), "2017-05-16T05:15:54.006Z".into())
                                .build(),
                        )
                        .add_property(
                            "GP".into(),
                            V1SourceProperty::builder()
                                .talker("GP".into())
                                .add_sentence("RMC".into(), "2017-04-03T06:14:04.451Z".into())
                                .build(),
                        )
                        .build(),
                )
                .build(),
        )
        .build();
    let folder = Path::new("tests/specification/examples/full/");
    let sk_data = read_signalk_from_file(folder.join("docs-data_model.json"));
    assert_eq!(sk_data, expected);
}

#[test]
fn test_sample_docs_data_model_metadata() {
    let expected = V1FullFormat::builder()
        .version("1.0.0".into())
        .self_("urn:mrn:signalk:uuid:c0d79334-4e25-4245-8892-54e8ccc8021d".into())
        .add_vessel(
            "urn:mrn:signalk:uuid:c0d79334-4e25-4245-8892-54e8ccc8021d".into(),
            V1Vessel::builder()
                .uuid("urn:mrn:signalk:uuid:c0d79334-4e25-4245-8892-54e8ccc8021d".into())
                .add_propulsion(
                    "instance0".into(),
                    V1Propulsion::builder()
                        .label("Port Engine".into())
                        .revolutions(
                            V1NumberValue::builder()
                                .value(1280.0)
                                .timestamp("2014-08-15T19:00:15.402Z".into())
                                .source("foo.bar".into())
                                .build(),
                        )
                        .build(),
                )
                .build(),
        )
        .build();
    let folder = Path::new("tests/specification/examples/full/");
    let sk_data = read_signalk_from_file(folder.join("docs-data_model_metadata.json"));
    assert_eq!(sk_data, expected);
}

#[test]
fn test_sample_docs_data_model_multiple_values_metadata() {
    let expected = V1FullFormat::builder()
        .version("0.9.0".into())
        .self_("urn:mrn:signalk:uuid:c0d79334-4e25-4245-8892-54e8ccc8021d".into())
        .add_vessel(
            "urn:mrn:signalk:uuid:c0d79334-4e25-4245-8892-54e8ccc8021d".into(),
            V1Vessel::builder()
                .uuid("urn:mrn:signalk:uuid:c0d79334-4e25-4245-8892-54e8ccc8021d".into())
                .navigation(
                    V1Navigation::builder()
                        .course_over_ground_true(
                            V1NumberValue::builder()
                                .value(3.61562407843144)
                                .timestamp("2017-04-03T06:14:04.451Z".into())
                                .source("ttyUSB0.GP".into())
                                .build(),
                        )
                        .build(),
                )
                .build(),
        )
        .build();
    let folder = Path::new("tests/specification/examples/full/");
    let sk_data = read_signalk_from_file(folder.join("docs-data_model_multiple_values.json"));
    assert_eq!(sk_data, expected);
}

#[test]
fn test_sample_docs_notifications() {
    let expected = V1FullFormat::builder()
        .version("1.0.0".into())
        .self_("urn:mrn:signalk:uuid:c0d79334-4e25-4245-8892-54e8ccc8021d".into())
        .add_vessel(
            "urn:mrn:signalk:uuid:c0d79334-4e25-4245-8892-54e8ccc8021d".into(),
            V1Vessel::builder()
                .uuid("urn:mrn:signalk:uuid:c0d79334-4e25-4245-8892-54e8ccc8021d".into())
                .notifications(
                    V1Notification::builder()
                        .add_child(
                            "mob".into(),
                            V1Notification::builder()
                                .value(
                                    V1NotificationValue::builder()
                                        .method("visual".into())
                                        .method("sound".into())
                                        .state("emergency".into())
                                        .message("Man Overboard!".into())
                                        .build(),
                                )
                                .common_value_fields(
                                    V1CommonValueFields::builder()
                                        .timestamp("2017-04-10T08:33:53Z".into())
                                        .source("nmea1.II".into())
                                        .build(),
                                )
                                .build(),
                        )
                        .add_child(
                            "navigation".into(),
                            V1Notification::builder()
                                .add_child(
                                    "gnss".into(),
                                    V1Notification::builder()
                                        .value(
                                            V1NotificationValue::builder()
                                                .method("visual".into())
                                                .state("alert".into())
                                                .message("GPS signal lost!".into())
                                                .build(),
                                        )
                                        .common_value_fields(
                                            V1CommonValueFields::builder()
                                                .timestamp("2017-04-10T08:33:53Z".into())
                                                .source("nmea1.II".into())
                                                .build(),
                                        )
                                        .build(),
                                )
                                .add_child(
                                    "anchor".into(),
                                    V1Notification::builder()
                                        .add_child(
                                            "currentRadius".into(),
                                            V1Notification::builder()
                                                .value(
                                                    V1NotificationValue::builder()
                                                        .method("sound".into())
                                                        .state("alarm".into())
                                                        .message("Dragging anchor!".into())
                                                        .build(),
                                                )
                                                .common_value_fields(
                                                    V1CommonValueFields::builder()
                                                        .timestamp("2017-04-10T08:33:53Z".into())
                                                        .source("nmea1.II".into())
                                                        .build(),
                                                )
                                                .build(),
                                        )
                                        .build(),
                                )
                                .build(),
                        )
                        .build(),
                )
                .build(),
        )
        .build();
    let folder = Path::new("tests/specification/examples/full/");
    let sk_data = read_signalk_from_file(folder.join("docs-notifications.json"));
    assert_eq!(sk_data, expected);
}

#[test]
fn test_sample_electrical_full() {
    let expected = V1FullFormat::builder()
        .version("1.0.0".into())
        .self_("urn:mrn:signalk:uuid:c0d79334-4e25-4245-8892-54e8ccc8021d".into())
        .add_vessel(
            "urn:mrn:signalk:uuid:c0d79334-4e25-4245-8892-54e8ccc8021d".into(),
            V1Vessel::builder()
                .uuid("urn:mrn:signalk:uuid:c0d79334-4e25-4245-8892-54e8ccc8021d".into())
                .electrical(
                    V1Electrical::builder()
                        .add_ac(
                            "bus1".into(),
                            V1ACBus::builder()
                                .identity(
                                    V1ElectricalIdentity::builder()
                                        .name("Primary AC Bus".into())
                                        .location("Engine Room".into())
                                        .date_installed("2010-01-17T00:00:00Z".into())
                                        .build(),
                                )
                                .add_phase(
                                    "A".into(),
                                    V1ElectricalACQualities::builder()
                                        .line_line_voltage(
                                            V1NumberValue::builder()
                                                .value(480.0)
                                                .timestamp("2014-04-10T08:33:53Z".into())
                                                .source("sources.ac_monitor_01".into())
                                                .build(),
                                        )
                                        .line_neutral_voltage(
                                            V1NumberValue::builder()
                                                .value(277.0)
                                                .timestamp("2014-04-10T08:33:53Z".into())
                                                .source("sources.ac_monitor_01".into())
                                                .build(),
                                        )
                                        .frequency(
                                            V1NumberValue::builder()
                                                .value(60.0)
                                                .timestamp("2014-04-10T08:33:53Z".into())
                                                .source("sources.ac_monitor_01".into())
                                                .build(),
                                        )
                                        .build(),
                                )
                                .add_phase(
                                    "B".into(),
                                    V1ElectricalACQualities::builder()
                                        .line_line_voltage(
                                            V1NumberValue::builder()
                                                .value(480.0)
                                                .timestamp("2014-04-10T08:33:53Z".into())
                                                .source("sources.ac_monitor_01".into())
                                                .build(),
                                        )
                                        .line_neutral_voltage(
                                            V1NumberValue::builder()
                                                .value(277.0)
                                                .timestamp("2014-04-10T08:33:53Z".into())
                                                .source("sources.ac_monitor_01".into())
                                                .build(),
                                        )
                                        .frequency(
                                            V1NumberValue::builder()
                                                .value(60.0)
                                                .timestamp("2014-04-10T08:33:53Z".into())
                                                .source("sources.ac_monitor_01".into())
                                                .build(),
                                        )
                                        .build(),
                                )
                                .add_phase(
                                    "C".into(),
                                    V1ElectricalACQualities::builder()
                                        .line_line_voltage(
                                            V1NumberValue::builder()
                                                .value(480.0)
                                                .timestamp("2014-04-10T08:33:53Z".into())
                                                .source("sources.ac_monitor_01".into())
                                                .build(),
                                        )
                                        .line_neutral_voltage(
                                            V1NumberValue::builder()
                                                .value(277.0)
                                                .timestamp("2014-04-10T08:33:53Z".into())
                                                .source("sources.ac_monitor_01".into())
                                                .build(),
                                        )
                                        .frequency(
                                            V1NumberValue::builder()
                                                .value(60.0)
                                                .timestamp("2014-04-10T08:33:53Z".into())
                                                .source("sources.ac_monitor_01".into())
                                                .build(),
                                        )
                                        .build(),
                                )
                                .build(),
                        )
                        .add_ac(
                            "utility".into(),
                            V1ACBus::builder()
                                .identity(V1ElectricalIdentity::default())
                                .add_phase(
                                    "A".into(),
                                    V1ElectricalACQualities::builder()
                                        .line_line_voltage(
                                            V1NumberValue::builder()
                                                .value(480.0)
                                                .timestamp("2014-04-10T08:33:53Z".into())
                                                .source("sources.ac_monitor_01".into())
                                                .build(),
                                        )
                                        .line_neutral_voltage(
                                            V1NumberValue::builder()
                                                .value(277.0)
                                                .timestamp("2014-04-10T08:33:53Z".into())
                                                .source("sources.ac_monitor_01".into())
                                                .build(),
                                        )
                                        .frequency(
                                            V1NumberValue::builder()
                                                .value(60.0)
                                                .timestamp("2014-04-10T08:33:53Z".into())
                                                .source("sources.ac_monitor_01".into())
                                                .build(),
                                        )
                                        .current(
                                            V1NumberValue::builder()
                                                .value(10.0)
                                                .timestamp("2014-04-10T08:33:53Z".into())
                                                .source("sources.ac_monitor_01".into())
                                                .build(),
                                        )
                                        .reactive_power(
                                            V1NumberValue::builder()
                                                .value(5000.0)
                                                .timestamp("2014-04-10T08:33:53Z".into())
                                                .source("sources.ac_monitor_01".into())
                                                .build(),
                                        )
                                        .power_factor(
                                            V1NumberValue::builder()
                                                .value(0.97)
                                                .timestamp("2014-04-10T08:33:53Z".into())
                                                .source("sources.ac_monitor_01".into())
                                                .build(),
                                        )
                                        .power_factor_lagging("leading".into())
                                        .real_power(
                                            V1NumberValue::builder()
                                                .value(10500.0)
                                                .timestamp("2014-04-10T08:33:53Z".into())
                                                .source("sources.ac_monitor_01".into())
                                                .build(),
                                        )
                                        .apparent_power(
                                            V1NumberValue::builder()
                                                .value(10500.0)
                                                .timestamp("2014-04-10T08:33:53Z".into())
                                                .source("sources.ac_monitor_01".into())
                                                .build(),
                                        )
                                        .build(),
                                )
                                .add_phase(
                                    "B".into(),
                                    V1ElectricalACQualities::builder()
                                        .line_line_voltage(
                                            V1NumberValue::builder()
                                                .value(480.0)
                                                .timestamp("2014-04-10T08:33:53Z".into())
                                                .source("sources.ac_monitor_01".into())
                                                .build(),
                                        )
                                        .line_neutral_voltage(
                                            V1NumberValue::builder()
                                                .value(277.0)
                                                .timestamp("2014-04-10T08:33:53Z".into())
                                                .source("sources.ac_monitor_01".into())
                                                .build(),
                                        )
                                        .frequency(
                                            V1NumberValue::builder()
                                                .value(60.0)
                                                .timestamp("2014-04-10T08:33:53Z".into())
                                                .source("sources.ac_monitor_01".into())
                                                .build(),
                                        )
                                        .current(
                                            V1NumberValue::builder()
                                                .value(10.0)
                                                .timestamp("2014-04-10T08:33:53Z".into())
                                                .source("sources.ac_monitor_01".into())
                                                .build(),
                                        )
                                        .reactive_power(
                                            V1NumberValue::builder()
                                                .value(5000.0)
                                                .timestamp("2014-04-10T08:33:53Z".into())
                                                .source("sources.ac_monitor_01".into())
                                                .build(),
                                        )
                                        .power_factor(
                                            V1NumberValue::builder()
                                                .value(0.97)
                                                .timestamp("2014-04-10T08:33:53Z".into())
                                                .source("sources.ac_monitor_01".into())
                                                .build(),
                                        )
                                        .power_factor_lagging("leading".into())
                                        .real_power(
                                            V1NumberValue::builder()
                                                .value(10500.0)
                                                .timestamp("2014-04-10T08:33:53Z".into())
                                                .source("sources.ac_monitor_01".into())
                                                .build(),
                                        )
                                        .apparent_power(
                                            V1NumberValue::builder()
                                                .value(10500.0)
                                                .timestamp("2014-04-10T08:33:53Z".into())
                                                .source("sources.ac_monitor_01".into())
                                                .build(),
                                        )
                                        .build(),
                                )
                                .add_phase(
                                    "C".into(),
                                    V1ElectricalACQualities::builder()
                                        .line_line_voltage(
                                            V1NumberValue::builder()
                                                .value(480.0)
                                                .timestamp("2014-04-10T08:33:53Z".into())
                                                .source("sources.ac_monitor_01".into())
                                                .build(),
                                        )
                                        .line_neutral_voltage(
                                            V1NumberValue::builder()
                                                .value(277.0)
                                                .timestamp("2014-04-10T08:33:53Z".into())
                                                .source("sources.ac_monitor_01".into())
                                                .build(),
                                        )
                                        .frequency(
                                            V1NumberValue::builder()
                                                .value(60.0)
                                                .timestamp("2014-04-10T08:33:53Z".into())
                                                .source("sources.ac_monitor_01".into())
                                                .build(),
                                        )
                                        .current(
                                            V1NumberValue::builder()
                                                .value(10.0)
                                                .timestamp("2014-04-10T08:33:53Z".into())
                                                .source("sources.ac_monitor_01".into())
                                                .build(),
                                        )
                                        .reactive_power(
                                            V1NumberValue::builder()
                                                .value(5000.0)
                                                .timestamp("2014-04-10T08:33:53Z".into())
                                                .source("sources.ac_monitor_01".into())
                                                .build(),
                                        )
                                        .power_factor(
                                            V1NumberValue::builder()
                                                .value(0.97)
                                                .timestamp("2014-04-10T08:33:53Z".into())
                                                .source("sources.ac_monitor_01".into())
                                                .build(),
                                        )
                                        .power_factor_lagging("leading".into())
                                        .real_power(
                                            V1NumberValue::builder()
                                                .value(10500.0)
                                                .timestamp("2014-04-10T08:33:53Z".into())
                                                .source("sources.ac_monitor_01".into())
                                                .build(),
                                        )
                                        .apparent_power(
                                            V1NumberValue::builder()
                                                .value(10500.0)
                                                .timestamp("2014-04-10T08:33:53Z".into())
                                                .source("sources.ac_monitor_01".into())
                                                .build(),
                                        )
                                        .build(),
                                )
                                .build(),
                        )
                        .build(),
                )
                .build(),
        )
        .build();
    let folder = Path::new("tests/specification/examples/full/");
    let sk_data = read_signalk_from_file(folder.join("electrical-ac-full.json"));
    assert_eq!(sk_data, expected);
}

#[test]
fn test_sample_mob_alarm() {
    let expected = V1FullFormat::builder()
        .version("1.0.0".into())
        .self_("urn:mrn:imo:mmsi:366982330".into())
        .add_vessel(
            "urn:mrn:imo:mmsi:366982330".into(),
            V1Vessel::builder()
                .mmsi("366982330".into())
                .notifications(
                    V1Notification::builder()
                        .add_child(
                            "mob".into(),
                            V1Notification::builder()
                                .value(
                                    V1NotificationValue::builder()
                                        .method("visual".into())
                                        .method("sound".into())
                                        .state("emergency".into())
                                        .message("MOB".into())
                                        .build(),
                                )
                                .common_value_fields(
                                    V1CommonValueFields::builder()
                                        .timestamp("2014-04-10T08:33:53Z".into())
                                        .source("sources.gps_0183_MOB".into())
                                        .build(),
                                )
                                .build(),
                        )
                        .build(),
                )
                .build(),
        )
        .build();
    let folder = Path::new("tests/specification/examples/full/");
    let sk_data = read_signalk_from_file(folder.join("mob-alarm.json"));
    assert_eq!(sk_data, expected);
}

#[test]
fn test_sample_depth_alarm() {
    let expected = V1FullFormat::builder()
        .version("1.0.0".into())
        .self_("urn:mrn:signalk:uuid:b7590868-1d62-47d9-989c-32321b349fb9".into())
        .add_vessel(
            "urn:mrn:signalk:uuid:b7590868-1d62-47d9-989c-32321b349fb9".into(),
            V1Vessel::builder()
                .uuid("urn:mrn:signalk:uuid:c0d79334-4e25-4245-8892-54e8ccc8021d".into())
                .notifications(
                    V1Notification::builder()
                        .add_child(
                            "environment".into(),
                            V1Notification::builder()
                                .add_child(
                                    "depth".into(),
                                    V1Notification::builder()
                                        .add_child(
                                            "belowKeel".into(),
                                            V1Notification::builder()
                                                .value(
                                                    V1NotificationValue::builder()
                                                        .message("Running aground!".into())
                                                        .state("alarm".into())
                                                        .method("sound".into())
                                                        .build(),
                                                )
                                                .common_value_fields(
                                                    V1CommonValueFields::builder()
                                                        .timestamp("2014-04-10T08:33:53Z".into())
                                                        .source("nmea1.II".into())
                                                        .build(),
                                                )
                                                .build(),
                                        )
                                        .build(),
                                )
                                .build(),
                        )
                        .build(),
                )
                .build(),
        )
        .build();
    let folder = Path::new("tests/specification/examples/full/");
    let sk_data = read_signalk_from_file(folder.join("signalk-depth-alarm.json"));
    assert_eq!(sk_data, expected);
}

#[test]
fn test_sample_depth_meta_attr() {
    let expected = V1FullFormat::builder()
        .version("1.0.0".into())
        .self_("urn:mrn:signalk:uuid:b7590868-1d62-47d9-989c-32321b349fb9".into())
        .add_vessel(
            "urn:mrn:signalk:uuid:b7590868-1d62-47d9-989c-32321b349fb9".into(),
            V1Vessel::builder()
                .uuid("urn:mrn:signalk:uuid:c0d79334-4e25-4245-8892-54e8ccc8021d".into())
                .environment(
                    V1Environment::builder()
                        .depth(
                            V1EnvironmentDepth::builder()
                                .below_keel(
                                    V1NumberValue::builder()
                                        .value(3.4)
                                        .timestamp("2015-03-06T16:57:53.643Z".into())
                                        .source("nmea1.II".into())
                                        .build(),
                                )
                                .build(),
                        )
                        .build(),
                )
                .build(),
        )
        .build();
    let folder = Path::new("tests/specification/examples/full/");
    let sk_data = read_signalk_from_file(folder.join("signalk-depth-meta-attr.json"));
    assert_eq!(sk_data, expected);
}

#[test]
fn test_sample_vessel_time() {
    let expected = V1FullFormat::builder()
        .version("1.0.0".into())
        .self_("urn:mrn:signalk:uuid:b7590868-1d62-47d9-989c-32321b349fb9".into())
        .add_vessel(
            "urn:mrn:signalk:uuid:b7590868-1d62-47d9-989c-32321b349fb9".into(),
            V1Vessel::builder()
                .uuid("urn:mrn:signalk:uuid:b7590868-1d62-47d9-989c-32321b349fb9".into())
                .environment(
                    V1Environment::builder()
                        .time(
                            V1EnvironmentTime::builder()
                                .millis(1449648657735)
                                .timezone_offset(-1300)
                                .timestamp("2014-04-10T08:33:53Z".into())
                                .source(
                                    V1Source::builder()
                                        .label("Realtime clock".into())
                                        .type_("system".into())
                                        .build(),
                                )
                                .build(),
                        )
                        .build(),
                )
                .build(),
        )
        .build();
    let folder = Path::new("tests/specification/examples/full/");
    let sk_data = read_signalk_from_file(folder.join("vessel-time.json"));
    assert_eq!(sk_data, expected);
}
