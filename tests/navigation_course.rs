use log::{debug, error, info, warn};
use signalk::full::V1FullFormat;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[test]
fn test_navigation_course_to_pos() {
    let path = Path::new("tests/demo_data/ongoing_navigation.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1FullFormat = serde_json::from_reader(reader).unwrap();
    //println!("{:?}", sk_data);
    let self_vessel = sk_data.get_self();
    println!("0: {:?}", self_vessel);
    if let Some(_self_vessel) = self_vessel {
        println!("1: {:?}", _self_vessel.navigation);
        if let Some(navi) = _self_vessel.navigation.as_ref() {
            if let Some(course) = navi.course_rhumbline.as_ref() {
                println!("2: {:?}", course);
                assert_eq!(
                    course
                        .active_route
                        .as_ref()
                        .unwrap()
                        .href
                        .as_ref()
                        .unwrap()
                        .value
                        .as_ref()
                        .unwrap(),
                    "/resources/routes/f273bede-706d-40cb-bfd2-775499e40653"
                )
            } else {
                assert!(false);
            }
        }
    } else {
        println!("2: {:?}", self_vessel);
        assert!(false);
    }
    // assert_eq!(
    //     self_vessel
    //         .unwrap()
    //         .navigation
    //         .as_ref()
    //         .unwrap()
    //         .course_rhumbline
    //         .as_ref()
    //         .unwrap()
    //         .next_point
    //         .as_ref()
    //         .unwrap()
    //         .value
    //         .unwrap(),
    //     0.1)
}
