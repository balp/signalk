use serde::{Deserialize, Serialize};
use serde_json::value::Value;

use crate::definitions::{V1DateTime, V1NumberValue};
use crate::helper_functions::get_f64_value;
use crate::SignalKGetError;

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct V1Navigation {
    // pub lights: Option<V1Lights>,
    pub course_over_ground_magnetic: Option<V1NumberValue>,
    pub course_over_ground_true: Option<V1NumberValue>,
    pub course_rhumbline: Option<V1Course>,
    pub course_great_circle: Option<V1Course>,
    // pub closest_approach: Option<V1ClosestApproach>,
    // pub racing: Option<V1Racing>,
    pub magnetic_variation: Option<V1NumberValue>,
    pub magnetic_variation_age_of_service: Option<V1NumberValue>,
    // pub destination: Option<V1Destination>,
    // pub gnss: Option<V1gnss>,
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
    pub speed_through_water_transverse: Option<V1NumberValue>,
    pub speed_through_water_longitudinal: Option<V1NumberValue>,
    pub leeway_angle: Option<V1NumberValue>,
    pub log: Option<V1NumberValue>,
    // pub trip: Option<V1Trip>,
    // pub state: Option<V1State>,
    // pub anchor: Option<V1Anchor>,
    // pub datetime: Option<V1Datetime>,
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
            "courseRhumbline" => {
                path.remove(0);
                self.course_rhumbline = Some(V1Course::builder().json_value(path, value).build())
            }
            "courseGreatCircle" => {
                path.remove(0);
                self.course_great_circle = Some(V1Course::builder().json_value(path, value).build())
            }
            "magneticVariation" => {
                self.magnetic_variation = Some(V1NumberValue::builder().json_value(value).build())
            }
            "magneticVariationAgeOfService" => {
                self.magnetic_variation_age_of_service =
                    Some(V1NumberValue::builder().json_value(value).build())
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
            &_ => {
                log::warn!("Unknown value to update: {:?}::{:?}", path, value);
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
    course_rhumbline: Option<V1Course>,
    course_great_circle: Option<V1Course>,
    // pub closest_approach: Option<V1ClosestApproach>,
    // pub racing: Option<V1Racing>,
    magnetic_variation: Option<V1NumberValue>,
    magnetic_variation_age_of_service: Option<V1NumberValue>,
    // pub destination: Option<V1Destination>,
    // pub gnss: Option<V1gnss>,
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
    speed_through_water_transverse: Option<V1NumberValue>,
    speed_through_water_longitudinal: Option<V1NumberValue>,
    leeway_angle: Option<V1NumberValue>,
    log: Option<V1NumberValue>,
    // pub trip: Option<V1Trip>,
    // pub state: Option<V1State>,
    // pub anchor: Option<V1Anchor>,
    // pub datetime: Option<V1Datetime>,
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
    pub fn build(self) -> V1Navigation {
        V1Navigation {
            course_over_ground_magnetic: self.course_over_ground_magnetic,
            speed_over_ground: self.speed_over_ground,
            speed_through_water: self.speed_through_water,
            speed_through_water_transverse: self.speed_through_water_transverse,
            speed_through_water_longitudinal: self.speed_through_water_longitudinal,
            leeway_angle: self.leeway_angle,
            course_over_ground_true: self.course_over_ground_true,
            course_rhumbline: self.course_rhumbline,
            course_great_circle: self.course_great_circle,
            magnetic_variation: self.magnetic_variation,
            magnetic_variation_age_of_service: self.magnetic_variation_age_of_service,
            heading_magnetic: self.heading_magnetic,
            magnetic_deviation: self.magnetic_deviation,
            heading_compass: self.heading_compass,
            heading_true: self.heading_true,
            position: self.position,
            rate_of_turn: self.rate_of_turn,
            log: self.log,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct V1Course {
    pub cross_track_error: Option<V1NumberValue>,
    pub bearing_track_true: Option<V1NumberValue>,
    pub bearing_track_magnetic: Option<V1NumberValue>,
    pub active_route: Option<V1ActiveRoute>,
    // pub next_point: Option<V1CourseNextPoint>,
    // pub previous_point: Option<V1CourseNextPoint>,
}

impl V1Course {
    pub fn builder() -> V1CourseBuilder {
        V1CourseBuilder::default()
    }
}
#[derive(Default)]
pub struct V1CourseBuilder {
    pub cross_track_error: Option<V1NumberValue>,
    pub bearing_track_true: Option<V1NumberValue>,
    pub bearing_track_magnetic: Option<V1NumberValue>,
    pub active_route: Option<V1ActiveRoute>,
    // pub next_point: Option<V1CourseNextPoint>,
    // pub previous_point: Option<V1CourseNextPoint>,
}
impl V1CourseBuilder {
    pub fn json_value(mut self, path: &mut Vec<&str>, value: &serde_json::Value) -> V1CourseBuilder {
        match path[0] {
            "crossTrackError" => {
                self.cross_track_error = Some(V1NumberValue::builder().json_value(value).build());
            }
            "bearingTrackTrue" => {
                self.bearing_track_true = Some(V1NumberValue::builder().json_value(value).build());
            }
            "bearingTrackMagnetic" => {
                self.bearing_track_magnetic = Some(V1NumberValue::builder().json_value(value).build());
            }
            &_ => {
                log::warn!("Unknown value to update: {:?}::{:?}", path, value);
            }
        }
        self
    }
    pub fn build(self) -> V1Course {
        V1Course {
            cross_track_error: self.cross_track_error,
            bearing_track_true: self.bearing_track_true,
            bearing_track_magnetic: self.bearing_track_magnetic,
            active_route: self.active_route,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct V1ActiveRoute {
    pub href: Option<String>,
    pub estimated_time_of_arrival: Option<V1DateTime>,
    pub start_time: Option<V1DateTime>,
}
impl V1ActiveRoute {
    pub fn builder() -> V1ActiveRouteBuilder { V1ActiveRouteBuilder::default() }
}
#[derive(Default)]
pub struct V1ActiveRouteBuilder {
    pub href: Option<String>,
    pub estimated_time_of_arrival: Option<V1DateTime>,
    pub start_time: Option<V1DateTime>,
}

impl crate::navigation::V1ActiveRouteBuilder {
    pub fn href(mut self, value: String) -> V1ActiveRouteBuilder {
        self.href = Some(value);
        self
    }
    pub fn estimated_time_of_arrival(mut self, value: V1DateTime) -> V1ActiveRouteBuilder {
        self.estimated_time_of_arrival = Some(value);
        self
    }
    pub fn start_time(mut self, value: V1DateTime) -> V1ActiveRouteBuilder {
        self.start_time = Some(value);
        self
    }
    pub fn build(self) -> V1ActiveRoute {
        V1ActiveRoute {
            href: self.href,
            estimated_time_of_arrival: self.estimated_time_of_arrival,
            start_time: self.start_time,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct V1PositionType {
    pub value: V1PositionValue,
    pub timestamp: String,
    #[serde(rename = "$source")]
    pub source: String,
    pub pgn: Option<f64>,
    pub sentence: Option<String>,
}

impl V1PositionType {
    pub fn builder() -> V1PositionTypeBuilder {
        V1PositionTypeBuilder::default()
    }
}

#[derive(Default)]
pub struct V1PositionTypeBuilder {
    pub value: V1PositionValue,
    pub timestamp: String,
    pub source: String,
    pub pgn: Option<f64>,
    pub sentence: Option<String>,
}

impl V1PositionTypeBuilder {
    pub fn json_value(mut self, value: &serde_json::Value) -> V1PositionTypeBuilder {
        if let Value::Object(ref map) = value {
            if let Some(longitude) = map.get("longitude") {
                if let Some(lon) = longitude.as_f64() {
                    self.value.longitude = lon;
                }
            }
            if let Some(latitude) = map.get("latitude") {
                if let Some(lat) = latitude.as_f64() {
                    self.value.latitude = lat;
                }
            }
            if let Some(altitude) = map.get("altitude") {
                if let Some(alt) = altitude.as_f64() {
                    self.value.altitude = Some(alt);
                }
            }
        }
        self
    }

    pub fn value(mut self, value: V1PositionValue) -> V1PositionTypeBuilder {
        self.value = value;
        self
    }
    pub fn timestamp(mut self, timestamp: String) -> V1PositionTypeBuilder {
        self.timestamp = timestamp;
        self
    }
    pub fn source(mut self, source: String) -> V1PositionTypeBuilder {
        self.source = source;
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
    use serde_json::{Number, Value};
    use crate::navigation::{V1Course, V1Navigation};

    #[test]
    fn update_navigation_course_rl_xte() {
        let mut navigation = V1Navigation::builder()
            .course_rhumbline(V1Course::builder()
                .build())
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
                .unwrap()
            ,
            1.5
        )
    }

    #[test]
    fn update_navigation_course_rl_bearing_track_true() {
        let mut navigation = V1Navigation::builder()
            .course_rhumbline(V1Course::builder()
                .build())
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
                .unwrap()
            ,
            0.1234
        )
    }
    #[test]
    fn update_navigation_course_rl_bearing_track_magnetic() {
        let mut navigation = V1Navigation::builder()
            .course_rhumbline(V1Course::builder()
                .build())
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
                .unwrap()
            ,
            1.2345
        )
    }


}