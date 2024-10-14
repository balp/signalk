use crate::definitions::{V1DateTime, V1StringValue};
use crate::V1NumberValue;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
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
                self.active_route = Some(V1ActiveRoute::builder().json_value(value).build());
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
}

#[derive(Default)]
pub struct V1ActiveRouteBuilder {
    pub href: Option<V1StringValue>,
    pub estimated_time_of_arrival: Option<V1DateTime>,
    pub start_time: Option<V1DateTime>,
}

impl V1ActiveRouteBuilder {
    pub fn json_value(mut self, value: &serde_json::Value) -> V1ActiveRouteBuilder {
        if let Value::Object(ref map) = value {
            if let Some(href) = map.get("href") {
                if let Some(_href) = href.as_str() {
                    self.href = Some(V1StringValue::builder().value(_href.to_string()).build());
                }
            }
            if let Some(eta) = map.get("estimatedTimeOfArrival") {
                //self.estimated_time_of_arrival = V1Timestamp::builder().value(value).build();
            }
        }
        self
    }
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
