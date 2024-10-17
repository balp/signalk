use signalk::full::V1FullFormat;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

// TODO: Aircraft not supported
/*
#[test]
#[should_panic]
fn aircraft_mmsi_bad() {
    let path = Path::new("tests/specification/test_data/full-invalid/aircraft-mmsi_bad.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1FullFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}
*/

#[test]
// #[should_panic]
// TODO: No validation of AIS Types
fn ais_ais_ship_type_bad() {
    let path = Path::new("tests/specification/test_data/full-invalid/ais-aisShipType_bad.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1FullFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

// TODO: Source not supported
/*
#[test]
#[should_panic]
fn ais_source0() {
    let path = Path::new("tests/specification/test_data/full-invalid/ais-source0.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1FullFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
#[should_panic]
fn ais_source28() {
    let path = Path::new("tests/specification/test_data/full-invalid/ais-source28.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1FullFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}
*/

// TODO: Alarm not supported
/*
#[test]
#[should_panic]
fn alarms_must_not_have_additional_keys_in_notifications() {
    let path = Path::new("tests/specification/test_data/full-invalid/alarms-must_not_have_additional_keys_in_notifications.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1FullFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
#[should_panic]
fn alarms_must_not_have_dot_notation_in_branch_names() {
    let path = Path::new("tests/specification/test_data/full-invalid/alarms-must_not_have_dot_notation_in_branch_names.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1FullFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
#[should_panic]
fn alarms_must_not_have_incorrectly_spelled_keys() {
    let path = Path::new("tests/specification/test_data/full-invalid/alarms-must_not_have_incorrectly_spelled_keys.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1FullFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
#[should_panic]
fn alarms_must_not_have_spaces_in_branch_names() {
    let path = Path::new("tests/specification/test_data/full-invalid/alarms-must_not_have_spaces_in_branch_names.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1FullFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
#[should_panic]
fn alarms_must_not_have_sub_trees_beneath_well_defined_notifications() {
    let path = Path::new("tests/specification/test_data/full-invalid/alarms-must_not_have_sub_trees_beneath_well_defined_notifications.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1FullFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
#[should_panic]
fn alarms_must_not_use_well_defined_notifications_at_deep_levels() {
    let path = Path::new("tests/specification/test_data/full-invalid/alarms-must_not_use_well_defined_notifications_at_deep_levels.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1FullFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
#[should_panic]
fn alarms_must_use_alarm_method_array_with_enum() {
    let path = Path::new("tests/specification/test_data/full-invalid/alarms-must_use_alarmMethod_array_with_enum.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1FullFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
#[should_panic]
fn alarms_must_use_alarm_state_enum() {
    let path = Path::new("tests/specification/test_data/full-invalid/alarms-must_use_alarmStateEnum.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1FullFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
#[should_panic]
fn alarms_must_use_array_of_states_not_string() {
    let path = Path::new("tests/specification/test_data/full-invalid/alarms-must_use_array_of_states_not_string.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1FullFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}
*/

// TODO: Aton not supported
/*
#[test]
#[should_panic]
fn aton_aton_type_bad() {
    let path = Path::new("tests/specification/test_data/full-invalid/aton-atonType_bad.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1FullFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
#[should_panic]
fn aton_has_sails() {
    let path = Path::new("tests/specification/test_data/full-invalid/aton-has_sails.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1FullFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
#[should_panic]
fn aton_mmsi_bad() {
    let path = Path::new("tests/specification/test_data/full-invalid/aton-mmsi_bad.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1FullFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}
*/

// TODO: Timezone info not supported
/*
#[test]
#[should_panic]
fn datetime_timezone_region_invalid() {
    let path = Path::new("tests/specification/test_data/full-invalid/datetime-timezone_region_invalid.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1FullFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}


#[test]
#[should_panic]
fn datetime_timezone_region_but_no_timezone() {
    let path = Path::new("tests/specification/test_data/full-invalid/datetime-timezoneRegion_but_no_timezone.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1FullFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}
*/

#[test]
#[should_panic]
fn gnss_method_quality_bad() {
    let path = Path::new("tests/specification/test_data/full-invalid/gnss-methodQuality_bad.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1FullFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
#[should_panic]
fn gnss_type_bad() {
    let path = Path::new("tests/specification/test_data/full-invalid/gnss-type_bad.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1FullFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

// TODO: Timestapms not validated
/*
#[test]
#[should_panic]
fn invalid_timestamp_ending_with_z() {
    let path = Path::new("tests/specification/test_data/full-invalid/invalid-timestamp-ending-with-Z.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1FullFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
#[should_panic]
fn invalid_timestamp_subtle() {
    let path = Path::new("tests/specification/test_data/full-invalid/invalid-timestamp-subtle.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1FullFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}
*/

// TODO: Navigation Maneuver not supported
/*
#[test]
#[should_panic]
fn maneuver_bad() {
    let path = Path::new("tests/specification/test_data/full-invalid/maneuver-bad.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1FullFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}
*/

// MMSI not verified
/*
#[test]
#[should_panic]
fn mothership_bad() {
    let path = Path::new("tests/specification/test_data/full-invalid/mothership-bad.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1FullFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}
*/

// TODO: Navigation destination not supported
/*
#[test]
#[should_panic]
fn nav_destination_bad() {
    let path = Path::new("tests/specification/test_data/full-invalid/nav-destination_bad.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1FullFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}
*/

// TODO: Sar not supported
/*

#[test]
#[should_panic]
fn sar_mmsi_bad() {
    let path = Path::new("tests/specification/test_data/full-invalid/sar-mmsi_bad.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1FullFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
#[should_panic]
fn sar_sails() {
    let path = Path::new("tests/specification/test_data/full-invalid/sar-sails.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1FullFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}
*/

#[test]
#[should_panic]
fn sources_both_n2k_and_ais() {
    let path =
        Path::new("tests/specification/test_data/full-invalid/sources-both_n2k_and_ais.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1FullFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

/* TODO: No sources
#[test]
#[should_panic]
fn sources_valid_sources_with_no_0183_n2k_or_ais_and_other_items() {
    let path = Path::new("tests/specification/test_data/full-invalid/sources-valid_sources_with_no_0183_n2k_or_ais_and_other_items.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1FullFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}
*/

// Invalid keys are just ignored
#[test]
//#[should_panic]
fn trip_key_bad() {
    let path = Path::new("tests/specification/test_data/full-invalid/trip-key_bad.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1FullFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
#[should_panic]
fn trip_value_is_a_string() {
    let path = Path::new("tests/specification/test_data/full-invalid/trip-value_is_a_string.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1FullFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

/* TODO: MMSI are not validated
#[test]
#[should_panic]
fn vessel_mmsi_bad() {
    let path = Path::new("tests/specification/test_data/full-invalid/vessel-mmsi_bad.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1FullFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

*/
