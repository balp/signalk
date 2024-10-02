use signalk::full::V1FullFormat;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[test]
fn test_navigation_course_to_pos() {
    let path = Path::new("tests/demo_data/v1_full_230416.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1FullFormat = serde_json::from_reader(reader).unwrap();
    //println!("{:?}", sk_data);
    let self_vessel = sk_data.get_self();
    println!("0: {:?}", self_vessel);
    if let Some(_self_vessel) = self_vessel {
        println!("1: {:?}", _self_vessel.navigation);
    } else {
        println!("2: {:?}", self_vessel);
    }
    assert_eq!(
        self_vessel
            .unwrap()
            .navigation
            .as_ref()
            .unwrap()
            .course_rhumbline
            .as_ref()
            .unwrap()
            .bearing_track_magnetic
            .as_ref()
            .unwrap()
            .value
            .unwrap(),
        0.1)
}
