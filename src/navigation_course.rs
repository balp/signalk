use crate::definitions::{V1DateTime, V1StringValue};
use crate::{V1CommonValueFields, V1NumberValue, V1PositionType};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct V1Course {
    pub cross_track_error: Option<V1NumberValue>,
    pub bearing_track_true: Option<V1NumberValue>,
    pub bearing_track_magnetic: Option<V1NumberValue>,
    pub active_route: Option<V1ActiveRoute>,
    pub next_point: Option<V1CourseNextPoint>,
    // pub previous_point: Option<V1CourseNextPoint>,
}

impl V1Course {
    pub fn builder() -> V1CourseBuilder {
        V1CourseBuilder::default()
    }

    pub fn update(&mut self, path: &mut Vec<&str>, value: &serde_json::value::Value) {
        match path[0] {
            "crossTrackError" => {
                self.cross_track_error = Some(V1NumberValue::builder().json_value(value).build());
            }
            "bearingTrackTrue" => {
                self.bearing_track_true = Some(V1NumberValue::builder().json_value(value).build());
            }
            "bearingTrackMagnetic" => {
                self.bearing_track_magnetic =
                    Some(V1NumberValue::builder().json_value(value).build());
            }
            "activeRoute" => {
                if self.active_route.is_none() {
                    self.active_route = Some(V1ActiveRoute::default());
                }
                if let Some(ref mut route) = self.active_route {
                    path.remove(0);
                    route.update(path, value);
                }
            }
            "nextPoint" => {
                if self.next_point.is_none() {
                    self.next_point = Some(V1CourseNextPoint::default());
                }
                if let Some(ref mut netxt_point) = self.next_point {
                    path.remove(0);
                    netxt_point.update(path, value);
                }
            }
            &_ => {
                log::warn!("V1Course: Unknown value to update {:?}::{:?}", path, value);
            }
        }
    }
}

#[derive(Default)]
pub struct V1CourseBuilder {
    cross_track_error: Option<V1NumberValue>,
    bearing_track_true: Option<V1NumberValue>,
    bearing_track_magnetic: Option<V1NumberValue>,
    active_route: Option<V1ActiveRoute>,
    next_point: Option<V1CourseNextPoint>,
    // pub previous_point: Option<V1CoursePreviousPoint>,
}

impl V1CourseBuilder {
    pub fn json_value(
        mut self,
        path: &mut Vec<&str>,
        value: &serde_json::Value,
    ) -> V1CourseBuilder {
        match path[0] {
            "crossTrackError" => {
                self.cross_track_error = Some(V1NumberValue::builder().json_value(value).build());
            }
            "bearingTrackTrue" => {
                self.bearing_track_true = Some(V1NumberValue::builder().json_value(value).build());
            }
            "bearingTrackMagnetic" => {
                self.bearing_track_magnetic =
                    Some(V1NumberValue::builder().json_value(value).build());
            }
            "activeRoute" => {
                self.active_route = V1ActiveRoute::from_value(value);
            }
            &_ => {
                log::warn!(
                    "V1CourseBuilder: Unknown value to update: {:?}::{:?}",
                    path,
                    value
                );
            }
        }
        self
    }
    pub fn cross_track_error(mut self, value: V1NumberValue) -> V1CourseBuilder {
        self.cross_track_error = Some(value);
        self
    }
    pub fn bearing_track_true(mut self, value: V1NumberValue) -> V1CourseBuilder {
        self.bearing_track_true = Some(value);
        self
    }
    pub fn bearing_track_magnetic(mut self, value: V1NumberValue) -> V1CourseBuilder {
        self.bearing_track_magnetic = Some(value);
        self
    }
    pub fn active_route(mut self, value: V1ActiveRoute) -> V1CourseBuilder {
        self.active_route = Some(value);
        self
    }
    pub fn next_point(mut self, value: V1CourseNextPoint) -> V1CourseBuilder {
        self.next_point = Some(value);
        self
    }
    pub fn build(self) -> V1Course {
        V1Course {
            cross_track_error: self.cross_track_error,
            bearing_track_true: self.bearing_track_true,
            bearing_track_magnetic: self.bearing_track_magnetic,
            active_route: self.active_route,
            next_point: self.next_point,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct V1ActiveRoute {
    pub href: Option<V1StringValue>,
    pub estimated_time_of_arrival: Option<V1DateTime>,
    pub start_time: Option<V1DateTime>,
}

impl V1ActiveRoute {
    pub fn builder() -> V1ActiveRouteBuilder {
        V1ActiveRouteBuilder::default()
    }

    pub fn from_value(value: &Value) -> Option<V1ActiveRoute> {
        if value.is_null() {
            None
        } else {
            let route_result: Result<V1ActiveRoute, serde_json::Error> =
                serde_json::from_value(value.clone());
            if let Ok(route_value) = route_result {
                Some(route_value)
            } else {
                None
            }
        }
    }

    pub fn update(&mut self, path: &mut Vec<&str>, value: &serde_json::value::Value) {
        match path[0] {
            "href" => {
                let type_result: Result<V1StringValue, serde_json::Error> =
                    serde_json::from_value(value.clone());
                if let Ok(type_value) = type_result {
                    self.href = Some(type_value);
                } else {
                    log::error!(
                        "V1ActiveRoute:href: Invalid StringValue type: {:?}",
                        type_result
                    );
                    self.href = None;
                }
            }
            "estimatedTimeOfArrival" => {
                let type_result: Result<V1DateTime, serde_json::Error> =
                    serde_json::from_value(value.clone());
                if let Ok(type_value) = type_result {
                    self.estimated_time_of_arrival = Some(type_value);
                } else {
                    log::error!(
                        "V1ActiveRoute:estimatedTimeOfArrival: Invalid StringValue type: {:?}",
                        type_result
                    );
                    self.estimated_time_of_arrival = None;
                }
            }
            "startTime" => {
                let type_result: Result<V1DateTime, serde_json::Error> =
                    serde_json::from_value(value.clone());
                if let Ok(type_value) = type_result {
                    self.start_time = Some(type_value);
                } else {
                    log::error!(
                        "V1ActiveRoute:estimatedTimeOfArrival: Invalid StringValue type: {:?}",
                        type_result
                    );
                    self.start_time = None;
                }
            }
            &_ => {
                log::warn!(
                    "V1ActiveRoute: Unknown value to update {:?}::{:?}",
                    path,
                    value
                );
            }
        }
    }
}

#[derive(Default)]
pub struct V1ActiveRouteBuilder {
    pub href: Option<V1StringValue>,
    pub estimated_time_of_arrival: Option<V1DateTime>,
    pub start_time: Option<V1DateTime>,
}

impl V1ActiveRouteBuilder {
    pub fn href(mut self, value: V1StringValue) -> V1ActiveRouteBuilder {
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
#[serde(rename_all = "camelCase")]
pub struct V1CourseNextPoint {
    value: Option<V1CourseNextPointValue>,
    distance: Option<V1NumberValue>,
    bearing_true: Option<V1NumberValue>,
    bearing_magnetic: Option<V1NumberValue>,
    velocity_made_good: Option<V1NumberValue>,
    time_to_go: Option<V1NumberValue>,
    position: Option<V1PositionType>,
    estimated_time_of_arrival: Option<V1DateTime>,
    arrival_circle: Option<V1NumberValue>,
    #[serde(flatten)]
    pub common_value_fields: Option<V1CommonValueFields>,
}

impl V1CourseNextPoint {
    pub fn update(&mut self, path: &mut Vec<&str>, value: &serde_json::value::Value) {
        match path[0] {
            "value" => {
                if self.value.is_none() {
                    self.value = Some(V1CourseNextPointValue::default());
                }
                if let Some(ref mut point_value) = self.value {
                    path.remove(0);
                    point_value.update(path, value);
                }
            }
            "distance" => {
                self.distance = V1NumberValue::from_value(value);
            }
            "bearingTrue" => {
                self.bearing_true = V1NumberValue::from_value(value);
            }
            "bearingMagnetic" => {
                self.bearing_magnetic = V1NumberValue::from_value(value);
            }
            "velocityMadeGood" => {
                self.velocity_made_good = V1NumberValue::from_value(value);
            }
            "timeToGo" => {
                self.time_to_go = V1NumberValue::from_value(value);
            }
            "position" => {
                if path.len() == 1 {
                    self.position = V1PositionType::from_value(value);
                } else {
                    if self.position.is_none() {
                        self.position = Some(V1PositionType::default());
                    }
                    if let Some(ref mut position) = self.position {
                        path.remove(0);
                        position.update(path, value);
                    }
                }
            }
            "arrivalCircle" => {
                self.arrival_circle = V1NumberValue::from_value(value);
            }
            &_ => {
                log::warn!(
                    "V1CourseNextPoint: Unknown value to update {:?}::{:?}",
                    path,
                    value
                );
            }
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct V1CourseNextPointValue {
    #[serde(rename = "type")]
    type_: Option<V1StringValue>,
    href: Option<V1StringValue>,
}

impl V1CourseNextPointValue {
    pub fn update(&mut self, path: &mut Vec<&str>, value: &serde_json::Value) {
        match path[0] {
            "type" => {
                if value.is_null() {
                    self.href = None;
                } else {
                    let type_result: Result<V1StringValue, serde_json::Error> =
                        serde_json::from_value(value.clone());
                    if let Ok(type_value) = type_result {
                        self.type_ = Some(type_value);
                    } else {
                        log::error!(
                            "V1CourseNextPointValue:type: Invalid StringValue type: {:?}",
                            type_result
                        );
                        self.type_ = None;
                    }
                }
            }
            "href" => {
                if value.is_null() {
                    self.href = None;
                } else {
                    let type_result: Result<V1StringValue, serde_json::Error> =
                        serde_json::from_value(value.clone());
                    if let Ok(type_value) = type_result {
                        self.href = Some(type_value);
                    } else {
                        log::error!(
                            "V1CourseNextPointValue:href: Invalid StringValue type:{:?}::{:?}",
                            value,
                            type_result
                        );
                        self.href = None;
                    }
                }
            }
            &_ => {
                log::warn!(
                    "V1CourseNextPointValue: Unknown value to update {:?}::{:?}",
                    path,
                    value
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::navigation_course::{V1Course, V1CourseNextPoint};

    #[test]
    fn course_full_valid() {
        let j = r#"
        {
          "crossTrackError": {
            "value": -8.67,
            "timestamp": "2017-01-25T00:23:05.154Z",
            "$source": "a.suitable.path"
          },
          "bearingTrackTrue": {
            "value": 0.9162978572970231,
            "timestamp": "2017-01-25T00:23:05.154Z",
            "$source": "a.suitable.path"
          },
          "bearingTrackMagnetic": {
            "value": 1.1986521,
            "timestamp": "2017-01-25T00:23:05.158Z",
            "$source": "a.suitable.path"
          },
          "activeRoute": {
            "href": "/vessels/vessels.urn:mrn:imo:mmsi:230099999/resources/routes/urn:mrn:signalk:uuid:3dd34dcc-36bf-4d61-ba80-233799b25672",
            "estimatedTimeOfArrival": {
              "value": "2017-04-24T06:24:18.632Z",
              "timestamp": "2017-01-25T00:23:05.158Z",
              "$source": "a.suitable.path"
            },
            "startTime": {
              "value": "2017-01-25T00:20:15.385Z",
              "timestamp": "2017-01-25T00:23:05.305Z",
              "$source": "a.suitable.path"
            }
          },
          "nextPoint": {
            "value": {
              "type": "Waypoint",
              "href": "/vessels/vessels.urn:mrn:imo:mmsi:230099999/resources/waypoints/urn:mrn:signalk:uuid:4fe4ff38-879a-46ed-9a7d-7caeea475241"
            },
            "timestamp": "2017-01-25T00:23:05Z",
            "$source": "a.suitable.path",
            "distance": {
              "value": 2407.6000020320143,
              "timestamp": "2017-01-25T00:23:05Z",
              "$source": "a.suitable.path"
            },
            "bearingTrue": {
              "value": 0.9162978572970231,
              "timestamp": "2017-01-25T00:23:05Z",
              "$source": "a.suitable.path"
            },
            "velocityMadeGood": {
              "value": 0.2572222873852017,
              "timestamp": "2017-01-25T00:23:05Z",
              "$source": "a.suitable.path"
            },
            "timeToGo": {
              "value": 9628,
              "timestamp": "2017-01-25T00:23:05Z",
              "$source": "a.suitable.path"
            },
            "position": {
              "value": {
                "latitude": 49.287333333333336,
                "longitude": -123.1595
              },
              "timestamp": "2017-01-25T00:23:05.230Z",
              "$source": "a.suitable.path"
            },
            "estimatedTimeOfArrival": {
              "value": "2017-01-26T00:03:29.340Z",
              "timestamp": "2017-01-25T00:23:05.230Z",
              "$source": "a.suitable.path"
            }
          },
          "previousPoint": {
            "value": {
              "type": "Waypoint",
              "href": "/vessels/vessels.urn:mrn:imo:mmsi:230099999/resources/waypoints/urn:mrn:signalk:uuid:dd4a4c06-0c1d-4b5e-90c3-963649ee5e6d"
            },
            "timestamp": "2017-01-25T00:23:05.385Z",
            "$source": "a.suitable.path",
            "position": {
              "timestamp": "2017-01-25T00:23:05.385Z",
              "$source": "a.suitable.path",
              "value": {
                "latitude": 49.287333333333336,
                "longitude": -123.1595
              }
            }
          }
        }"#;
        let course_with_point: V1Course = serde_json::from_str(j).unwrap();
        println!("{:?}", course_with_point);
    }

    #[test]
    fn next_point_valid() {
        let j = r#"
        {
          "value": {
            "type": "Waypoint",
            "href": "/vessels/vessels.urn:mrn:imo:mmsi:230099999/resources/waypoints/urn:mrn:signalk:uuid:4fe4ff38-879a-46ed-9a7d-7caeea475241"
          },
          "timestamp": "2017-01-25T00:23:05Z",
          "$source": "a.suitable.path",
          "distance": {
            "value": 2407.6000020320143,
            "timestamp": "2017-01-25T00:23:05Z",
            "$source": "a.suitable.path"
          },
          "bearingTrue": {
            "value": 0.9162978572970231,
            "timestamp": "2017-01-25T00:23:05Z",
            "$source": "a.suitable.path"
          },
          "velocityMadeGood": {
            "value": 0.2572222873852017,
            "timestamp": "2017-01-25T00:23:05Z",
            "$source": "a.suitable.path"
          },
          "timeToGo": {
            "value": 9628,
            "timestamp": "2017-01-25T00:23:05Z",
            "$source": "a.suitable.path"
          },
          "position": {
            "value": {
              "latitude": 49.287333333333336,
              "longitude": -123.1595
            },
            "timestamp": "2017-01-25T00:23:05.230Z",
            "$source": "a.suitable.path"
          },
          "estimatedTimeOfArrival": {
            "value": "2017-01-26T00:03:29.340Z",
            "timestamp": "2017-01-25T00:23:05.230Z",
            "$source": "a.suitable.path"
          }
        }"#;
        let course_with_point: V1CourseNextPoint = serde_json::from_str(j).unwrap();
        println!("{:?}", course_with_point);
    }
}
