use crate::definitions::{V1DateTime, V1NumberValue, V1StringValue};
use crate::helper_functions::{get_f64_value, get_path, F64CompatiblePath, Path};
use crate::navigation_course::{V1Course, V1CourseApi};
use crate::navigation_gnss::V1gnss;
use crate::SignalKGetError;
use log::debug;
use serde::{Deserialize, Serialize};
use serde_json::value::Value;

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct V1Navigation {
    // pub lights: Option<V1Lights>,
    pub course_over_ground_magnetic: Option<V1NumberValue>,
    pub course_over_ground_true: Option<V1NumberValue>,
    pub course: Option<V1CourseApi>,
    pub course_rhumbline: Option<V1Course>,
    pub course_great_circle: Option<V1Course>,
    // pub closest_approach: Option<V1ClosestApproach>,
    // pub racing: Option<V1Racing>,
    pub magnetic_variation: Option<V1NumberValue>,
    pub magnetic_variation_age_of_service: Option<V1NumberValue>,
    // pub destination: Option<V1Destination>,
    pub gnss: Option<V1gnss>,
    pub heading_magnetic: Option<V1NumberValue>,
    pub magnetic_deviation: Option<V1NumberValue>,
    pub heading_compass: Option<V1NumberValue>,
    pub heading_true: Option<V1NumberValue>,
    pub position: Option<V1PositionType>,
    // pub attitude: Option<V1AttitudeType>,
    // pub maneuver: Option<V1Maneuver>,
    pub rate_of_turn: Option<V1NumberValue>,
    pub speed_over_ground: Option<V1NumberValue>,
    pub speed_through_water: Option<V1NumberValue>,
    pub speed_through_water_reference_type: Option<V1StringValue>,
    pub speed_through_water_transverse: Option<V1NumberValue>,
    pub speed_through_water_longitudinal: Option<V1NumberValue>,
    pub leeway_angle: Option<V1NumberValue>,
    pub log: Option<V1NumberValue>,
    pub trip: Option<V1Trip>,
    // pub state: Option<V1State>,
    // pub anchor: Option<V1Anchor>,
    pub datetime: Option<V1DateTime>,
}

impl Path<f64> for V1Navigation {
    fn get_path(&self, path: &[&str]) -> Result<f64, SignalKGetError> {
        debug!("get_path({:?}): ...", path);
        match path[0] {
            "lights" => Err(SignalKGetError::TBD),
            "courseOverGroundMagnetic" => get_f64_value(&self.course_over_ground_magnetic),
            "courseOverGroundTrue" => get_f64_value(&self.course_over_ground_true),
            "course" => get_path(path, &(self.course.as_ref())),
            "courseRhumbline" => get_path(path, &(self.course_rhumbline.as_ref())),
            "courseGreatCircle" => get_path(path, &(self.course_great_circle.as_ref())),
            "closestApproach" => Err(SignalKGetError::TBD),
            "racing" => Err(SignalKGetError::TBD),
            "magneticVariation" => get_f64_value(&self.magnetic_variation),
            "magneticVariationAgeOfService" => {
                get_f64_value(&self.magnetic_variation_age_of_service)
            }
            "destination" => Err(SignalKGetError::TBD),
            "gnss" => get_path(path, &(self.gnss.as_ref())),
            "headingMagnetic" => get_f64_value(&self.heading_magnetic),
            "magneticDeviation" => get_f64_value(&self.magnetic_deviation),
            "headingCompass" => get_f64_value(&self.heading_compass),
            "headingTrue" => get_f64_value(&self.heading_true),
            "position" => get_path(path, &(self.position.as_ref())),
            "attitude" => Err(SignalKGetError::TBD),
            "maneuver" => Err(SignalKGetError::TBD),
            "rateOfTurn" => get_f64_value(&self.rate_of_turn),
            "speedOverGround" => get_f64_value(&self.speed_over_ground),
            "speedThroughWater" => get_f64_value(&self.speed_through_water),
            "speedThroughWaterReferenceType" => Err(SignalKGetError::WrongDataType),
            "speedThroughWaterTransverse" => get_f64_value(&self.speed_through_water_transverse),
            "speedThroughWaterLongitudinal" => {
                get_f64_value(&self.speed_through_water_longitudinal)
            }
            "leewayAngle" => get_f64_value(&self.speed_through_water_longitudinal),
            "log" => get_f64_value(&self.speed_through_water_longitudinal),
            "trip" => Err(SignalKGetError::TBD), // get_path(path, &(self.trip.as_ref())),
            "state" => Err(SignalKGetError::TBD),
            "anchor" => Err(SignalKGetError::TBD),
            "datetime" => Err(SignalKGetError::WrongDataType),

            &_ => Err(SignalKGetError::NoSuchPath),
        }
    }
}

impl V1Navigation {
    pub fn builder() -> V1NavigationBuilder {
        V1NavigationBuilder::default()
    }

    pub fn update(&mut self, path: &mut Vec<&str>, value: &serde_json::value::Value) {
        match path[0] {
            "courseOverGroundMagnetic" => {
                self.course_over_ground_magnetic =
                    Some(V1NumberValue::builder().json_value(value).build())
            }
            "courseOverGroundTrue" => {
                self.course_over_ground_true =
                    Some(V1NumberValue::builder().json_value(value).build())
            }
            "course" => {
                if self.course.is_none() {
                    self.course = Some(V1CourseApi::default());
                }
                if let Some(ref mut course) = self.course {
                    path.remove(0);
                    course.update(path, value);
                }
            }
            "courseRhumbline" => {
                if self.course_rhumbline.is_none() {
                    self.course_rhumbline = Some(V1Course::default());
                }
                if let Some(ref mut course) = self.course_rhumbline {
                    path.remove(0);
                    course.update(path, value);
                }
            }
            "courseGreatCircle" => {
                if self.course_great_circle.is_none() {
                    self.course_great_circle = Some(V1Course::default());
                }
                if let Some(ref mut course) = self.course_great_circle {
                    path.remove(0);
                    course.update(path, value);
                }
            }
            "magneticVariation" => {
                self.magnetic_variation = Some(V1NumberValue::builder().json_value(value).build())
            }
            "magneticVariationAgeOfService" => {
                self.magnetic_variation_age_of_service =
                    Some(V1NumberValue::builder().json_value(value).build())
            }
            "gnss" => {
                if self.gnss.is_none() {
                    self.gnss = Some(V1gnss::default());
                }
                if let Some(ref mut gnss) = self.gnss {
                    path.remove(0);
                    gnss.update(path, value);
                }
            }

            "headingMagnetic" => {
                self.heading_magnetic = Some(V1NumberValue::builder().json_value(value).build())
            }
            "magneticDeviation" => {
                self.magnetic_deviation = Some(V1NumberValue::builder().json_value(value).build())
            }
            "headingCompass" => {
                self.heading_compass = Some(V1NumberValue::builder().json_value(value).build())
            }
            "headingTrue" => {
                self.heading_true = Some(V1NumberValue::builder().json_value(value).build())
            }
            "position" => self.position = Some(V1PositionType::builder().json_value(value).build()),
            "rateOfTurn" => {
                self.rate_of_turn = Some(V1NumberValue::builder().json_value(value).build())
            }
            "speedOverGround" => {
                self.speed_over_ground = Some(V1NumberValue::builder().json_value(value).build())
            }
            "speedThroughWater" => {
                self.speed_through_water = Some(V1NumberValue::builder().json_value(value).build())
            }
            "speedThroughWaterReferenceType" => {
                if let Some(s) = value.as_str() {
                    self.speed_through_water_reference_type =
                        Some(V1StringValue::builder().value(s.to_string()).build())
                }
            }
            "speedThroughWaterTransverse" => {
                self.speed_through_water_transverse =
                    Some(V1NumberValue::builder().json_value(value).build())
            }
            "speedThroughWaterLongitudinal" => {
                self.speed_through_water_longitudinal =
                    Some(V1NumberValue::builder().json_value(value).build())
            }
            "leewayAngle" => {
                self.leeway_angle = Some(V1NumberValue::builder().json_value(value).build())
            }
            "log" => self.log = Some(V1NumberValue::builder().json_value(value).build()),
            "trip" => {
                if self.trip.is_none() {
                    self.trip = Some(V1Trip::default());
                }
                if let Some(ref mut trip) = self.trip {
                    path.remove(0);
                    trip.update(path, value);
                }
            }
            "datetime" => {
                let datetime: Result<V1DateTime, serde_json::Error> =
                    serde_json::from_value(value.clone());
                if let Ok(datetime) = datetime {
                    self.datetime = Some(datetime);
                } else {
                    log::error!("Invalid datetime value: {:?}", datetime);
                    self.datetime = None;
                }
            }
            &_ => {
                log::warn!(
                    "V1Navigation: Unknown value to update: {:?}::{:?}",
                    path,
                    value
                );
            }
        }
    }

    pub fn get_f64_for_path(&self, path: &mut Vec<&str>) -> Result<f64, SignalKGetError> {
        self.get_path(path)
    }
}

#[derive(Default)]
pub struct V1NavigationBuilder {
    // pub lights: Option<V1Lights>,
    course_over_ground_magnetic: Option<V1NumberValue>,
    course_over_ground_true: Option<V1NumberValue>,
    course: Option<V1CourseApi>,
    course_rhumbline: Option<V1Course>,
    course_great_circle: Option<V1Course>,
    // pub closest_approach: Option<V1ClosestApproach>,
    // pub racing: Option<V1Racing>,
    magnetic_variation: Option<V1NumberValue>,
    magnetic_variation_age_of_service: Option<V1NumberValue>,
    // pub destination: Option<V1Destination>,
    gnss: Option<V1gnss>,
    heading_magnetic: Option<V1NumberValue>,
    magnetic_deviation: Option<V1NumberValue>,
    heading_compass: Option<V1NumberValue>,
    heading_true: Option<V1NumberValue>,
    position: Option<V1PositionType>,
    // pub attitude: Option<V1AttitudeType>,
    // pub maneuver: Option<V1Maneuver>,
    rate_of_turn: Option<V1NumberValue>,
    speed_over_ground: Option<V1NumberValue>,
    speed_through_water: Option<V1NumberValue>,
    speed_through_water_reference_type: Option<V1StringValue>,
    speed_through_water_transverse: Option<V1NumberValue>,
    speed_through_water_longitudinal: Option<V1NumberValue>,
    leeway_angle: Option<V1NumberValue>,
    log: Option<V1NumberValue>,
    trip: Option<V1Trip>,
    // pub state: Option<V1State>,
    // pub anchor: Option<V1Anchor>,
    datetime: Option<V1DateTime>,
}

impl V1NavigationBuilder {
    pub fn course_over_ground_magnetic(mut self, value: V1NumberValue) -> V1NavigationBuilder {
        self.course_over_ground_magnetic = Some(value);
        self
    }
    pub fn course_over_ground_true(mut self, value: V1NumberValue) -> V1NavigationBuilder {
        self.course_over_ground_true = Some(value);
        self
    }
    pub fn course_rhumbline(mut self, value: V1Course) -> V1NavigationBuilder {
        self.course_rhumbline = Some(value);
        self
    }
    pub fn course_great_circle(mut self, value: V1Course) -> V1NavigationBuilder {
        self.course_great_circle = Some(value);
        self
    }
    pub fn magnetic_variation(mut self, value: V1NumberValue) -> V1NavigationBuilder {
        self.magnetic_variation = Some(value);
        self
    }
    pub fn magnetic_variation_age_of_service(
        mut self,
        value: V1NumberValue,
    ) -> V1NavigationBuilder {
        self.magnetic_variation_age_of_service = Some(value);
        self
    }
    pub fn heading_magnetic(mut self, value: V1NumberValue) -> V1NavigationBuilder {
        self.heading_magnetic = Some(value);
        self
    }
    pub fn magnetic_deviation(mut self, value: V1NumberValue) -> V1NavigationBuilder {
        self.magnetic_deviation = Some(value);
        self
    }
    pub fn heading_compass(mut self, value: V1NumberValue) -> V1NavigationBuilder {
        self.heading_compass = Some(value);
        self
    }
    pub fn heading_true(mut self, value: V1NumberValue) -> V1NavigationBuilder {
        self.heading_true = Some(value);
        self
    }
    pub fn position(mut self, position: V1PositionType) -> V1NavigationBuilder {
        self.position = Some(position);
        self
    }
    pub fn rate_of_turn(mut self, value: V1NumberValue) -> V1NavigationBuilder {
        self.rate_of_turn = Some(value);
        self
    }
    pub fn speed_over_ground(mut self, value: V1NumberValue) -> V1NavigationBuilder {
        self.speed_over_ground = Some(value);
        self
    }
    pub fn speed_through_water(mut self, value: V1NumberValue) -> V1NavigationBuilder {
        self.speed_through_water = Some(value);
        self
    }
    pub fn speed_through_water_transverse(mut self, value: V1NumberValue) -> V1NavigationBuilder {
        self.speed_through_water_transverse = Some(value);
        self
    }
    pub fn speed_through_water_longitudinal(mut self, value: V1NumberValue) -> V1NavigationBuilder {
        self.speed_through_water_longitudinal = Some(value);
        self
    }
    pub fn leeway_angle(mut self, value: V1NumberValue) -> V1NavigationBuilder {
        self.leeway_angle = Some(value);
        self
    }
    pub fn log(mut self, value: V1NumberValue) -> V1NavigationBuilder {
        self.log = Some(value);
        self
    }
    pub fn trip(mut self, value: V1Trip) -> V1NavigationBuilder {
        self.trip = Some(value);
        self
    }
    pub fn datetime(mut self, value: V1DateTime) -> V1NavigationBuilder {
        self.datetime = Some(value);
        self
    }
    pub fn build(self) -> V1Navigation {
        V1Navigation {
            course_over_ground_magnetic: self.course_over_ground_magnetic,
            speed_over_ground: self.speed_over_ground,
            speed_through_water: self.speed_through_water,
            speed_through_water_reference_type: self.speed_through_water_reference_type,
            speed_through_water_transverse: self.speed_through_water_transverse,
            speed_through_water_longitudinal: self.speed_through_water_longitudinal,
            leeway_angle: self.leeway_angle,
            course: self.course,
            course_over_ground_true: self.course_over_ground_true,
            course_rhumbline: self.course_rhumbline,
            course_great_circle: self.course_great_circle,
            magnetic_variation: self.magnetic_variation,
            magnetic_variation_age_of_service: self.magnetic_variation_age_of_service,
            gnss: self.gnss,
            heading_magnetic: self.heading_magnetic,
            magnetic_deviation: self.magnetic_deviation,
            heading_compass: self.heading_compass,
            heading_true: self.heading_true,
            position: self.position,
            rate_of_turn: self.rate_of_turn,
            log: self.log,
            trip: self.trip,
            datetime: self.datetime,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct V1Trip {
    pub log: Option<V1NumberValue>,
    pub last_reset: Option<V1DateTime>,
}

impl F64CompatiblePath for V1Trip {
    fn get_f64_for_path(&self, path: &mut Vec<&str>) -> Result<f64, SignalKGetError> {
        match path[0] {
            "log" => get_f64_value(&self.log),
            "lastReset" => Err(SignalKGetError::WrongDataType),
            &_ => Err(SignalKGetError::NoSuchPath),
        }
    }
}

impl V1Trip {
    pub fn builder() -> V1TripBuilder {
        V1TripBuilder::default()
    }
    pub fn update(&mut self, path: &mut Vec<&str>, value: &serde_json::value::Value) {
        match path[0] {
            "log" => self.log = Some(V1NumberValue::builder().json_value(value).build()),
            "lastReset" => {
                let val: Result<V1DateTime, serde_json::Error> =
                    serde_json::from_value(value.clone());
                if let Ok(val) = val {
                    self.last_reset = Some(val);
                } else {
                    log::warn!("V1Trip: Invalid last reset value");
                }
            }
            &_ => {
                log::warn!("V1Trip: Unknown value to update: {:?}::{:?}", path, value);
            }
        }
    }
}
#[derive(Default)]
pub struct V1TripBuilder {
    log: Option<V1NumberValue>,
    last_reset: Option<V1DateTime>,
}

impl V1TripBuilder {
    pub fn log(mut self, value: V1NumberValue) -> V1TripBuilder {
        self.log = Some(value);
        self
    }
    pub fn last_reset(mut self, value: V1DateTime) -> V1TripBuilder {
        self.last_reset = Some(value);
        self
    }
    pub fn build(self) -> V1Trip {
        V1Trip {
            log: self.log,
            last_reset: self.last_reset,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct V1PositionType {
    pub value: Option<V1PositionValue>,
    pub timestamp: Option<String>,
    #[serde(rename = "$source")]
    pub source: Option<String>,
    pub pgn: Option<f64>,
    pub sentence: Option<String>,
}

impl Path<f64> for V1PositionType {
    fn get_path(&self, path: &[&str]) -> Result<f64, SignalKGetError> {
        debug!("V1PositionType::get_path({:?}) {:?}", path, self);
        match path[0] {
            "longitude" => {
                if let Some(ref position) = self.value {
                    Ok(position.longitude)
                } else {
                    Err(SignalKGetError::ValueNotSet)
                }
            }
            "latitude" => {
                if let Some(ref position) = self.value {
                    Ok(position.latitude)
                } else {
                    Err(SignalKGetError::ValueNotSet)
                }
            }
            "altitude" => {
                if let Some(ref position) = self.value {
                    if let Some(altitude) = position.altitude {
                        Ok(altitude)
                    } else {
                        Err(SignalKGetError::ValueNotSet)
                    }
                } else {
                    Err(SignalKGetError::ValueNotSet)
                }
            }
            "timestamp" => Err(SignalKGetError::WrongDataType),
            "source" => Err(SignalKGetError::WrongDataType),
            "pgn" => Err(SignalKGetError::WrongDataType),
            "sentence" => Err(SignalKGetError::WrongDataType),
            &_ => Err(SignalKGetError::NoSuchPath),
        }
    }
}

impl F64CompatiblePath for V1PositionType {
    fn get_f64_for_path(&self, path: &mut Vec<&str>) -> Result<f64, SignalKGetError> {
        if path.is_empty() {
            Err(SignalKGetError::NoSuchPath)
        } else {
            match path[0] {
                "longitude" => Err(SignalKGetError::TBD),
                "latitude" => Err(SignalKGetError::TBD),
                &_ => Err(SignalKGetError::NoSuchPath),
            }
        }
    }
}

impl V1PositionType {
    pub fn builder() -> V1PositionTypeBuilder {
        V1PositionTypeBuilder::default()
    }

    pub fn from_value(value: &serde_json::Value) -> Option<V1PositionType> {
        let type_result: Result<V1PositionType, serde_json::Error> =
            serde_json::from_value(value.clone());
        if let Ok(position) = type_result {
            Some(position)
        } else {
            None
        }
    }
    pub fn update(&mut self, path: &mut Vec<&str>, value: &serde_json::value::Value) {
        if path.is_empty() {
            log::warn!("V1PositionType: Empty path: {:?}::{:?}", path, value);
        } else {
            match path[0] {
                &_ => {
                    log::warn!(
                        "V1PositionType: Unknown value to update {:?}::{:?}",
                        path,
                        value
                    );
                }
            }
        }
    }
}

#[derive(Default)]
pub struct V1PositionTypeBuilder {
    pub value: Option<V1PositionValue>,
    pub timestamp: Option<String>,
    pub source: Option<String>,
    pub pgn: Option<f64>,
    pub sentence: Option<String>,
}

impl V1PositionTypeBuilder {
    pub fn json_value(mut self, value: &serde_json::Value) -> V1PositionTypeBuilder {
        if let Value::Object(ref map) = value {
            if let Some(longitude) = map.get("longitude") {
                if let Some(lon) = longitude.as_f64() {
                    if let Some(val) = self.value.as_mut() {
                        val.longitude = lon;
                    } else {
                        self.value = Some(V1PositionValue {
                            longitude: lon,
                            ..Default::default()
                        });
                    }
                }
            }
            if let Some(latitude) = map.get("latitude") {
                if let Some(lat) = latitude.as_f64() {
                    if let Some(val) = self.value.as_mut() {
                        val.latitude = lat;
                    } else {
                        self.value = Some(V1PositionValue {
                            latitude: lat,
                            ..Default::default()
                        });
                    }
                }
            }
            if let Some(altitude) = map.get("altitude") {
                if let Some(alt) = altitude.as_f64() {
                    if let Some(val) = self.value.as_mut() {
                        val.altitude = Some(alt);
                    } else {
                        self.value = Some(V1PositionValue {
                            altitude: Some(alt),
                            ..Default::default()
                        });
                    }
                }
            }
        }
        self
    }

    pub fn value(mut self, value: V1PositionValue) -> V1PositionTypeBuilder {
        self.value = Some(value);
        self
    }
    pub fn timestamp(mut self, timestamp: String) -> V1PositionTypeBuilder {
        self.timestamp = Some(timestamp);
        self
    }
    pub fn source(mut self, source: String) -> V1PositionTypeBuilder {
        self.source = Some(source);
        self
    }
    pub fn pgn(mut self, pgn: f64) -> V1PositionTypeBuilder {
        self.pgn = Some(pgn);
        self
    }
    pub fn sentence(mut self, sentence: String) -> V1PositionTypeBuilder {
        self.sentence = Some(sentence);
        self
    }
    pub fn build(self) -> V1PositionType {
        V1PositionType {
            value: self.value,
            timestamp: self.timestamp,
            source: self.source,
            pgn: self.pgn,
            sentence: self.sentence,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct V1PositionValue {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: Option<f64>,
}

impl Path<f64> for V1PositionValue {
    fn get_path(&self, path: &[&str]) -> Result<f64, SignalKGetError> {
        debug!("get_path({:?}) {:?}", path, self);
        match path[0] {
            "longitude" => Ok(self.longitude),
            "latitude" => Ok(self.latitude),
            "altitude" => {
                if let Some(altitude) = self.altitude {
                    Ok(altitude)
                } else {
                    Err(SignalKGetError::ValueNotSet)
                }
            }
            &_ => Err(SignalKGetError::NoSuchPath),
        }
    }
}

impl V1PositionValue {
    pub fn new_2d(latitude: f64, longitude: f64) -> V1PositionValue {
        V1PositionValue {
            latitude,
            longitude,
            altitude: None,
        }
    }
    pub fn new_3d(latitude: f64, longitude: f64, altitude: f64) -> V1PositionValue {
        V1PositionValue {
            latitude,
            longitude,
            altitude: Some(altitude),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::definitions::F64Compatible;
    use crate::helper_functions::get_path;
    use crate::navigation::V1Navigation;
    use crate::navigation_course::V1Course;
    use crate::SignalKGetError;
    use serde_json::{Number, Value};
    use std::fs::File;
    use std::io::BufReader;
    use std::path::Path;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn update_navigation_course_rl_xte() {
        init();
        let mut navigation = V1Navigation::builder()
            .course_rhumbline(V1Course::builder().build())
            .build();
        let mut path = vec!["courseRhumbline", "crossTrackError"];
        let value = Value::Number(Number::from_f64(1.5).unwrap());
        navigation.update(&mut path, &value);
        assert_eq!(
            navigation
                .course_rhumbline
                .as_ref()
                .unwrap()
                .cross_track_error
                .as_ref()
                .unwrap()
                .value
                .unwrap(),
            1.5
        )
    }

    #[test]
    fn update_navigation_course_rl_bearing_track_true() {
        let mut navigation = V1Navigation::builder()
            .course_rhumbline(V1Course::builder().build())
            .build();
        let mut path = vec!["courseRhumbline", "bearingTrackTrue"];
        let value = Value::Number(Number::from_f64(0.1234).unwrap());
        navigation.update(&mut path, &value);
        assert_eq!(
            navigation
                .course_rhumbline
                .as_ref()
                .unwrap()
                .bearing_track_true
                .as_ref()
                .unwrap()
                .as_f64()
                .unwrap(),
            0.1234
        )
    }
    #[test]
    fn update_navigation_course_rl_bearing_track_magnetic() {
        let mut navigation = V1Navigation::builder()
            .course_rhumbline(V1Course::builder().build())
            .build();
        let mut path = vec!["courseRhumbline", "bearingTrackMagnetic"];
        let value = Value::Number(Number::from_f64(1.2345).unwrap());
        navigation.update(&mut path, &value);
        assert_eq!(
            navigation
                .course_rhumbline
                .as_ref()
                .unwrap()
                .bearing_track_magnetic
                .as_ref()
                .unwrap()
                .value
                .unwrap(),
            1.2345
        )
    }

    #[test]
    fn get_position_longitude_from_navigation() {
        assert_eq!(
            get_path_from_navigation_file(".position.longitude", "tests/test_data/navigation.json"),
            Ok(24.7354072)
        )
    }

    #[test]
    fn get_position_latitude_from_navigation() {
        assert_eq!(
            get_path_from_navigation_file(".position.latitude", "tests/test_data/navigation.json"),
            Ok(59.722222)
        )
    }
    #[test]
    fn get_datetime_from_navigation() {
        assert_eq!(
            get_path_from_navigation_file(".datetime", "tests/test_data/navigation.json"),
            Err(SignalKGetError::WrongDataType)
        )
    }
    #[test]
    fn get_gnss_antenna_altitude_from_navigation() {
        assert_eq!(
            get_path_from_navigation_file(
                ".gnss.antennaAltitude",
                "tests/test_data/navigation.json"
            ),
            Ok(2.17)
        )
    }
    #[test]
    fn get_gnss_satellites_from_navigation() {
        assert_eq!(
            get_path_from_navigation_file(".gnss.satellites", "tests/test_data/navigation.json"),
            Ok(11.0)
        )
    }
    #[test]
    fn get_gnss_horizontal_dilution_from_navigation() {
        assert_eq!(
            get_path_from_navigation_file(
                ".gnss.horizontalDilution",
                "tests/test_data/navigation.json"
            ),
            Ok(0.8)
        )
    }
    #[test]
    fn get_gnss_type_from_navigation() {
        assert_eq!(
            get_path_from_navigation_file(".gnss.type", "tests/test_data/navigation.json"),
            Err(SignalKGetError::WrongDataType)
        )
    }
    #[test]
    fn get_gnss_method_quality_from_navigation() {
        assert_eq!(
            get_path_from_navigation_file(".gnss.methodQuality", "tests/test_data/navigation.json"),
            Err(SignalKGetError::WrongDataType)
        )
    }
    #[test]
    fn get_gnss_integrity_from_navigation() {
        assert_eq!(
            get_path_from_navigation_file(".gnss.integrity", "tests/test_data/navigation.json"),
            Err(SignalKGetError::WrongDataType)
        )
    }
    #[test]
    fn get_gnss_satellites_in_view_from_navigation() {
        assert_eq!(
            get_path_from_navigation_file(
                ".gnss.satellitesInView",
                "tests/test_data/navigation.json"
            ),
            Err(SignalKGetError::WrongDataType)
        )
    }
    #[test]
    fn get_speed_through_water_from_navigation() {
        assert_eq!(
            get_path_from_navigation_file(".speedThroughWater", "tests/test_data/navigation.json"),
            Ok(3.34)
        )
    }
    #[test]
    fn get_speed_through_water_reference_type_from_navigation() {
        assert_eq!(
            get_path_from_navigation_file(
                ".speedThroughWaterReferenceType",
                "tests/test_data/navigation.json"
            ),
            Err(SignalKGetError::WrongDataType)
        )
    }
    #[test]
    fn get_heading_true_from_navigation() {
        assert_eq!(
            get_path_from_navigation_file(".headingTrue", "tests/test_data/navigation.json"),
            Ok(3.5535)
        )
    }
    #[test]
    fn get_magnetic_variation_from_navigation() {
        assert_eq!(
            get_path_from_navigation_file(".magneticVariation", "tests/test_data/navigation.json"),
            Ok(0.1414)
        )
    }
    #[test]
    fn get_course_great_circle_active_route_href_from_navigation() {
        assert_eq!(
            get_path_from_navigation_file(
                ".courseGreatCircle.activeRoute.href",
                "tests/test_data/navigation.json"
            ),
            Err(SignalKGetError::WrongDataType)
        )
    }
    #[test]
    fn get_course_great_circle_active_route_start_time_from_navigation() {
        assert_eq!(
            get_path_from_navigation_file(
                ".courseGreatCircle.activeRoute.startTime",
                "tests/test_data/navigation.json"
            ),
            Err(SignalKGetError::WrongDataType)
        )
    }
    #[test]
    fn get_course_great_circle_active_next_point_position_longitude_from_navigation() {
        assert_eq!(
            get_path_from_navigation_file(
                ".courseGreatCircle.nextPoint.position.longitude",
                "tests/test_data/navigation.json"
            ),
            Ok(24.69305084158094)
        )
    }
    #[test]
    fn get_course_great_circle_active_next_point_position_latitude_from_navigation() {
        assert_eq!(
            get_path_from_navigation_file(
                ".courseGreatCircle.nextPoint.position.latitude",
                "tests/test_data/navigation.json"
            ),
            Ok(59.67512773750016)
        )
    }
    #[test]
    fn get_course_great_circle_active_next_point_arrival_circle_from_navigation() {
        assert_eq!(
            get_path_from_navigation_file(
                ".courseGreatCircle.nextPoint.arrivalCircle",
                "tests/test_data/navigation.json"
            ),
            Ok(0.0)
        )
    }
    #[test]
    fn get_course_great_circle_active_previous_point_position_longitude_from_navigation() {
        assert_eq!(
            get_path_from_navigation_file(
                ".courseGreatCircle.previousPoint.position.longitude",
                "tests/test_data/navigation.json"
            ),
            Ok(24.7351314)
        )
    }
    #[test]
    fn get_course_great_circle_active_previous_point_position_latitude_from_navigation() {
        assert_eq!(
            get_path_from_navigation_file(
                ".courseGreatCircle.previousPoint.position.latitude",
                "tests/test_data/navigation.json"
            ),
            Ok(59.7217554)
        )
    }
    #[test]
    fn get_course_rhumbline_active_route_href_from_navigation() {
        assert_eq!(
            get_path_from_navigation_file(
                ".courseRhumbline.activeRoute.href",
                "tests/test_data/navigation.json"
            ),
            Err(SignalKGetError::WrongDataType)
        )
    }
    #[test]
    fn get_course_rhumbline_active_route_start_time_from_navigation() {
        assert_eq!(
            get_path_from_navigation_file(
                ".courseRhumbline.activeRoute.startTime",
                "tests/test_data/navigation.json"
            ),
            Err(SignalKGetError::WrongDataType)
        )
    }
    #[test]
    fn get_course_rhumbline_active_next_point_position_longitude_from_navigation() {
        assert_eq!(
            get_path_from_navigation_file(
                ".courseRhumbline.nextPoint.position.longitude",
                "tests/test_data/navigation.json"
            ),
            Ok(24.69305084158094)
        )
    }
    #[test]
    fn get_course_rhumbline_active_next_point_position_latitude_from_navigation() {
        assert_eq!(
            get_path_from_navigation_file(
                ".courseRhumbline.nextPoint.position.latitude",
                "tests/test_data/navigation.json"
            ),
            Ok(59.67512773750016)
        )
    }
    #[test]
    fn get_course_rhumbline_active_next_point_arrival_circle_from_navigation() {
        assert_eq!(
            get_path_from_navigation_file(
                ".courseRhumbline.nextPoint.arrivalCircle",
                "tests/test_data/navigation.json"
            ),
            Ok(0.0)
        )
    }
    #[test]
    fn get_course_rhumbline_active_previous_point_position_longitude_from_navigation() {
        assert_eq!(
            get_path_from_navigation_file(
                ".courseRhumbline.previousPoint.position.longitude",
                "tests/test_data/navigation.json"
            ),
            Ok(24.7351314)
        )
    }
    #[test]
    fn get_course_rhumbline_active_previous_point_position_latitude_from_navigation() {
        assert_eq!(
            get_path_from_navigation_file(
                ".courseRhumbline.previousPoint.position.latitude",
                "tests/test_data/navigation.json"
            ),
            Ok(59.7217554)
        )
    }
    #[test]
    fn get_course_calc_values_calc_method_from_navigation() {
        assert_eq!(
            get_path_from_navigation_file(
                ".course.calcValues.calcMethod",
                "tests/test_data/navigation.json"
            ),
            Err(SignalKGetError::WrongDataType)
        )
    }
    #[test]
    fn get_course_calc_values_bearing_track_true_from_navigation() {
        assert_eq!(
            get_path_from_navigation_file(
                ".course.calcValues.bearingTrackTrue",
                "tests/test_data/navigation.json"
            ),
            Ok(3.5692017850905313)
        )
    }
    #[test]
    fn get_course_calc_values_bearing_track_magnetic_from_navigation() {
        assert_eq!(
            get_path_from_navigation_file(
                ".course.calcValues.bearingTrackMagnetic",
                "tests/test_data/navigation.json"
            ),
            Ok(3.7106017850905313)
        )
    }
    #[test]
    fn get_course_calc_values_xte_from_navigation() {
        assert_eq!(
            get_path_from_navigation_file(
                ".course.calcValues.crossTrackError",
                "tests/test_data/navigation.json"
            ),
            Ok(7.445808293426564)
        )
    }
    #[test]
    fn get_course_calc_values_previous_point_distance_from_navigation() {
        assert_eq!(
            get_path_from_navigation_file(
                ".course.calcValues.previousPoint.distance",
                "tests/test_data/navigation.json"
            ),
            Ok(54.13862750114139)
        )
    }
    #[test]
    fn get_course_calc_values_distance_from_navigation() {
        assert_eq!(
            get_path_from_navigation_file(
                ".course.calcValues.distance",
                "tests/test_data/navigation.json"
            ),
            Ok(5750.5951760268945)
        )
    }

    fn get_path_from_navigation_file(
        path_string: &str,
        file_name: &str,
    ) -> Result<f64, SignalKGetError> {
        let path = Path::new(file_name);
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let course_with_point: V1Navigation = serde_json::from_reader(reader).unwrap();
        let path: Vec<&str> = path_string.split('.').collect();
        let result = get_path(&path, &Some(&course_with_point));
        result
    }
}
