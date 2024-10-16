use crate::definitions::{V1DateTime, V1NumberValue, V1StringValue};
use crate::helper_functions::get_f64_value;
use crate::navigation_course::{V1Course, V1CourseApi};
use crate::navigation_gnss::V1gnss;
use crate::SignalKGetError;
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

    pub fn get_f64_for_path(&self, path: &mut [&str]) -> Result<f64, SignalKGetError> {
        match path[0] {
            "course" => Err(SignalKGetError::TBD),
            "lights" => Err(SignalKGetError::TBD),
            "courseOverGroundMagnetic" => {
                let value = &self.course_over_ground_magnetic;
                get_f64_value(value)
            }
            "courseOverGroundTrue" => {
                let value = &self.course_over_ground_true;
                get_f64_value(value)
            }
            "courseRhumbline" => Err(SignalKGetError::WrongDataType),
            "courseGreatCircle" => Err(SignalKGetError::WrongDataType),
            "closestApproach" => Err(SignalKGetError::TBD),
            "racing" => Err(SignalKGetError::TBD),
            "magneticVariation" => {
                let value = &self.magnetic_variation;
                get_f64_value(value)
            }
            "magneticVariationAgeOfService" => {
                let value = &self.magnetic_variation_age_of_service;
                get_f64_value(value)
            }
            "destination" => Err(SignalKGetError::TBD),
            "gnss" => Err(SignalKGetError::TBD),
            "headingMagnetic" => {
                let value = &self.heading_magnetic;
                get_f64_value(value)
            }
            "magneticDeviation" => {
                let value = &self.magnetic_deviation;
                get_f64_value(value)
            }
            "headingCompass" => {
                let value = &self.heading_compass;
                get_f64_value(value)
            }
            "headingTrue" => {
                let value = &self.heading_true;
                get_f64_value(value)
            }
            "position" => Err(SignalKGetError::TBD),
            "rateOfTurn" => {
                let value = &self.rate_of_turn;
                get_f64_value(value)
            }
            "speedOverGround" => {
                let value = &self.speed_over_ground;
                get_f64_value(value)
            }
            "speedThroughWater" => {
                let value = &self.speed_through_water;
                get_f64_value(value)
            }
            "speedThroughWaterTransverse" => {
                let value = &self.speed_through_water_transverse;
                get_f64_value(value)
            }
            "speedThroughWaterLongitudinal" => {
                let value = &self.speed_through_water_longitudinal;
                get_f64_value(value)
            }
            "leewayAngle" => {
                let value = &self.leeway_angle;
                get_f64_value(value)
            }
            "log" => {
                let value = &self.log;
                get_f64_value(value)
            }
            "trip" => Err(SignalKGetError::TBD),
            "state" => Err(SignalKGetError::TBD),
            "anchor" => Err(SignalKGetError::TBD),
            "datetime" => Err(SignalKGetError::TBD),
            &_ => Err(SignalKGetError::NoSuchPath),
        }
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
                        let mut pos_ = V1PositionValue::default();
                        pos_.longitude = lon;
                        self.value = Some(pos_);
                    }
                }
            }
            if let Some(latitude) = map.get("latitude") {
                if let Some(lat) = latitude.as_f64() {
                    if let Some(val) = self.value.as_mut() {
                        val.latitude = lat;
                    } else {
                        let mut pos_ = V1PositionValue::default();
                        pos_.latitude = lat;
                        self.value = Some(pos_);
                    }
                }
            }
            if let Some(altitude) = map.get("altitude") {
                if let Some(alt) = altitude.as_f64() {
                    if let Some(val) = self.value.as_mut() {
                        val.altitude = Some(alt);
                    } else {
                        let mut pos_ = V1PositionValue::default();
                        pos_.altitude = Some(alt);
                        self.value = Some(pos_);
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

    use crate::navigation::V1Navigation;
    use crate::navigation_course::V1Course;
    use serde_json::{Number, Value};

    #[test]
    fn update_navigation_course_rl_xte() {
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
                .value
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
}
