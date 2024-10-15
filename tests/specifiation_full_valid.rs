use signalk::full::V1FullFormat;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

/// Make sure we can read the files without issue
#[test]
fn aircraft_basic_nav() {
    let path = Path::new("tests/specification/test_data/full-valid/aircraft-basic_nav.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1FullFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
fn aircraft_home_base() {
    let path = Path::new("tests/specification/test_data/full-valid/aircraft-home_base.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1FullFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
fn ais() {
    let path = Path::new("tests/specification/test_data/full-valid/ais.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1FullFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
fn ais_ship_type() {
    let path = Path::new("tests/specification/test_data/full-valid/ais-aisShipType.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1FullFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
fn ais_full_cpa_tcpa() {
    let path = Path::new("tests/specification/test_data/full-valid/ais-full_cpa_tcpa.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1FullFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
fn ais_source() {
    let path = Path::new("tests/specification/test_data/full-valid/ais-source.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1FullFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
fn alarms_branches_with_names_similar_to_mob_etc() {
    let path = Path::new("tests/specification/test_data/full-valid/alarms-branches_with_names_similar_to_mob_etc.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1FullFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
fn alarms_deep_keys() {
    let path = Path::new("tests/specification/test_data/full-valid/alarms-deep_keys.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1FullFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
fn alarms_multiple_at_multiple_levels() {
    let path = Path::new(
        "tests/specification/test_data/full-valid/alarms-multiple_at_multiple_levels.json",
    );
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1FullFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
fn alarms_shallow_keys() {
    let path = Path::new("tests/specification/test_data/full-valid/alarms-shallow_keys.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1FullFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
fn alarms_simple() {
    let path = Path::new("tests/specification/test_data/full-valid/alarms-simple.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1FullFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
fn aton_basic_position() {
    let path = Path::new("tests/specification/test_data/full-valid/aton-basic_position.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1FullFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
fn aton_dimensions() {
    let path = Path::new("tests/specification/test_data/full-valid/aton-dimensions.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1FullFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
fn course_full_tree() {
    let path = Path::new("tests/specification/test_data/full-valid/course-full_tree.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1FullFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
fn datetime_in_full_tree() {
    let path = Path::new("tests/specification/test_data/full-valid/datetime-in_full_tree.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1FullFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
fn datetime_timezone_in_environment() {
    let path =
        Path::new("tests/specification/test_data/full-valid/datetime-timezone_in_environment.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1FullFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
fn electrical_full_tree() {
    let path = Path::new("tests/specification/test_data/full-valid/electrical-full_tree.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1FullFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
fn environment_mode() {
    let path = Path::new("tests/specification/test_data/full-valid/environment-mode.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1FullFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
fn gnss_sample() {
    let path = Path::new("tests/specification/test_data/full-valid/gnss-sample.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1FullFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
fn identities_various_formats() {
    let path =
        Path::new("tests/specification/test_data/full-valid/identities-various_formats.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1FullFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
fn lux_i2c() {
    let path = Path::new("tests/specification/test_data/full-valid/lux-i2c.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1FullFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
fn lux_n2k() {
    let path = Path::new("tests/specification/test_data/full-valid/lux-N2K.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1FullFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
fn maneuver_sample() {
    let path = Path::new("tests/specification/test_data/full-valid/maneuver-sample.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1FullFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
fn miscellaneous_sample() {
    let path = Path::new("tests/specification/test_data/full-valid/miscellaneous-sample.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1FullFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
fn mothership_sample() {
    let path = Path::new("tests/specification/test_data/full-valid/mothership-sample.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1FullFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
fn nav_destination() {
    let path = Path::new("tests/specification/test_data/full-valid/nav-destination.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1FullFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
fn racing_sample() {
    let path = Path::new("tests/specification/test_data/full-valid/racing-sample.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1FullFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
fn registrations_sample() {
    let path = Path::new("tests/specification/test_data/full-valid/registrations-sample.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1FullFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
fn resources_with_position_ref() {
    let path =
        Path::new("tests/specification/test_data/full-valid/resources-with_position_ref.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1FullFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
fn sar_notifications() {
    let path = Path::new("tests/specification/test_data/full-valid/sar-notifications.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1FullFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
fn sar_position() {
    let path = Path::new("tests/specification/test_data/full-valid/sar-position.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1FullFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
fn sources_multiple_sources_for_same_path() {
    let path = Path::new(
        "tests/specification/test_data/full-valid/sources-multiple_sources_for_same_path.json",
    );
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1FullFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
fn sources_sample() {
    let path = Path::new("tests/specification/test_data/full-valid/sources-sample.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1FullFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
fn temperatures_sample() {
    let path = Path::new("tests/specification/test_data/full-valid/temperatures-sample.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1FullFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
fn trip_sample() {
    let path = Path::new("tests/specification/test_data/full-valid/trip-sample.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1FullFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}

#[test]
fn vessel_basic_nav() {
    let path = Path::new("tests/specification/test_data/full-valid/vessel-basic_nav.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1FullFormat = serde_json::from_reader(reader).unwrap();
    println!("{:?}", sk_data);
}
