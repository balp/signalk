use crate::definitions::{V1DateTime, V1StringValue, V2NumberValue};
use crate::helper_functions::{get_f64_value, get_path, Path};
use crate::{SignalKGetError, V1CommonValueFields, V1NumberValue, V1PositionType, V1PositionValue};
use log::debug;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct V1CourseApi {
    pub active_route: Option<V1CourseApiActiveRouteModel>,
    pub next_point: Option<V1CourseApiPointModel>,
    pub previous_point: Option<V1CourseApiPointModel>,
    pub start_time: Option<V1DateTime>,
    pub target_arrival_time: Option<V1DateTime>,
    pub arrival_circle: Option<i64>,
    pub calc_values: Option<V1CourseCalculationsModel>,
}

impl Path<f64> for V1CourseApi {
    fn get_path(&self, path: &[&str]) -> Result<f64, SignalKGetError> {
        debug!("V1CourseApi::get_path({:?})", path);
        match path[0] {
            "activeRoute" => get_path(path, &(self.active_route.as_ref())),
            "nextPoint" => get_path(path, &(self.next_point.as_ref())),
            "previousPoint" => get_path(path, &(self.previous_point.as_ref())),
            "startTime" => Err(SignalKGetError::WrongDataType),
            "targetArrivalTime" => Err(SignalKGetError::WrongDataType),
            "arrivalCircle" => Err(SignalKGetError::WrongDataType),
            "calcValues" => get_path(path, &(self.calc_values.as_ref())),
            &_ => Err(SignalKGetError::NoSuchPath),
        }
    }
}

impl V1CourseApi {
    pub fn update(&mut self, path: &mut Vec<&str>, value: &Value) {
        match path[0] {
            "activeRoute" => self.active_route = V1CourseApiActiveRouteModel::from_value(value),
            "nextPoint" => self.next_point = V1CourseApiPointModel::from_value(value),
            "previousPoint" => self.previous_point = V1CourseApiPointModel::from_value(value),
            "startTime" => self.start_time = V1DateTime::from_value(value),
            "targetArrivalTime" => self.target_arrival_time = V1DateTime::from_value(value),
            "arrivalCircle" => self.arrival_circle = value.as_i64(),
            "calcValues" => {
                if self.calc_values.is_none() {
                    self.calc_values = Some(V1CourseCalculationsModel::default());
                }
                if let Some(ref mut calc_values) = self.calc_values {
                    path.remove(0);
                    calc_values.update(path, value);
                }
            }
            &_ => {
                log::warn!(
                    "V1CourseApi: Unknown value to update {:?}::{:?}",
                    path,
                    value
                );
            }
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct V1CourseApiActiveRouteModel {
    pub href: V1StringValue,
    pub name: Option<V1StringValue>,
    pub point_index: i64,
    pub point_total: i64,
    pub reverse: bool,
}

impl Path<f64> for V1CourseApiActiveRouteModel {
    fn get_path(&self, path: &[&str]) -> Result<f64, SignalKGetError> {
        debug!("get_path({:?})", path);
        match path[0] {
            "href" => Err(SignalKGetError::WrongDataType),
            "name" => Err(SignalKGetError::WrongDataType),
            "pointIndex" => Err(SignalKGetError::WrongDataType),
            "pointTotal" => Err(SignalKGetError::WrongDataType),
            "reverse" => Err(SignalKGetError::WrongDataType),
            &_ => Err(SignalKGetError::NoSuchPath),
        }
    }
}

impl V1CourseApiActiveRouteModel {
    pub fn from_value(value: &Value) -> Option<V1CourseApiActiveRouteModel> {
        if value.is_null() {
            None
        } else {
            let route_result: Result<V1CourseApiActiveRouteModel, serde_json::Error> =
                serde_json::from_value(value.clone());
            if let Ok(route_value) = route_result {
                Some(route_value)
            } else {
                None
            }
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct V1CourseApiPointModel {
    pub position: V1PositionValue,
    pub href: Option<V1StringValue>,
    #[serde(rename = "type")]
    pub type_: Option<V1StringValue>,
}

impl Path<f64> for V1CourseApiPointModel {
    fn get_path(&self, path: &[&str]) -> Result<f64, SignalKGetError> {
        debug!("V1CourseApiPointModel::get_path({:?})", path);
        match path[0] {
            "position" => get_path(path, &(Some(&self.position))),
            "href" => Err(SignalKGetError::WrongDataType),
            "type" => Err(SignalKGetError::WrongDataType),
            &_ => Err(SignalKGetError::NoSuchPath),
        }
    }
}

impl V1CourseApiPointModel {
    pub fn from_value(value: &Value) -> Option<V1CourseApiPointModel> {
        if value.is_null() {
            None
        } else {
            let route_result: Result<V1CourseApiPointModel, serde_json::Error> =
                serde_json::from_value(value.clone());
            if let Ok(route_value) = route_result {
                Some(route_value)
            } else {
                None
            }
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct V1CourseCalculationsModel {
    pub calc_method: V1CourseCalculationsMethod,
    pub cross_track_error: Option<V2NumberValue>,
    pub bearing_track_true: Option<V2NumberValue>,
    pub bearing_track_magnetic: Option<V2NumberValue>,
    pub estimated_time_of_arrival: Option<V1DateTime>,
    pub distance: Option<V2NumberValue>,
    pub bearing_true: Option<V2NumberValue>,
    pub bearing_magnetic: Option<V2NumberValue>,
    pub velocity_made_good: Option<V2NumberValue>,
    pub time_to_go: Option<V2NumberValue>,
    pub target_speed: Option<V2NumberValue>,
    pub previous_point: Option<V1CourseCalculationsPreviousPoint>,
}

impl Path<f64> for V1CourseCalculationsModel {
    fn get_path(&self, path: &[&str]) -> Result<f64, SignalKGetError> {
        debug!("V1CourseCalculationsModel::get_path({:?})", path);
        match path[0] {
            "calcMethod" => get_path(path, &(Some(&self.calc_method))),
            "crossTrackError" => get_f64_value(&self.cross_track_error),
            "bearingTrackTrue" => get_f64_value(&self.bearing_track_true),
            "bearingTrackMagnetic" => get_f64_value(&self.bearing_track_magnetic),
            "estimatedTimeOfArrival" => Err(SignalKGetError::WrongDataType),
            "distance" => get_f64_value(&self.distance),
            "bearingTrue" => get_f64_value(&self.bearing_true),
            "bearingMagnetic" => get_f64_value(&self.bearing_magnetic),
            "velocityMadeGood" => get_f64_value(&self.velocity_made_good),
            "timeToGo" => get_f64_value(&self.time_to_go),
            "targetSpeed" => get_f64_value(&self.target_speed),
            "previousPoint" => get_path(path, &(self.previous_point.as_ref())),
            &_ => Err(SignalKGetError::NoSuchPath),
        }
    }
}

impl V1CourseCalculationsModel {
    pub fn from_value(value: &Value) -> Option<V1CourseCalculationsModel> {
        if value.is_null() {
            None
        } else {
            let route_result: Result<V1CourseCalculationsModel, serde_json::Error> =
                serde_json::from_value(value.clone());
            if let Ok(route_value) = route_result {
                Some(route_value)
            } else {
                None
            }
        }
    }
    pub fn update(&mut self, path: &mut Vec<&str>, value: &Value) {
        match path[0] {
            // "activeRoute" => self.calc_method = V1CourseCalculationsMethod::from_value(value),
            "calcMethod" => {
                let val: Result<V1CourseCalculationsMethod, serde_json::Error> =
                    serde_json::from_value(value.clone());
                if let Ok(val) = val {
                    self.calc_method = val;
                } else {
                    log::warn!("V1Trip: Invalid last reset value");
                }
            }
            "crossTrackError" => self.cross_track_error = V2NumberValue::from_value(value),
            "bearingTrackTrue" => self.bearing_track_true = V2NumberValue::from_value(value),
            "bearingTrackMagnetic" => {
                self.bearing_track_magnetic = V2NumberValue::from_value(value)
            }
            "estimatedTimeOfArrival" => {
                self.estimated_time_of_arrival = V1DateTime::from_value(value)
            }
            "distance" => self.distance = V2NumberValue::from_value(value),
            "bearingTrue" => self.bearing_true = V2NumberValue::from_value(value),
            "bearingMagnetic" => self.bearing_magnetic = V2NumberValue::from_value(value),
            "velocityMadeGood" => self.velocity_made_good = V2NumberValue::from_value(value),
            "timeToGo" => self.time_to_go = V2NumberValue::from_value(value),
            "targetSpeed" => self.target_speed = V2NumberValue::from_value(value),
            "previousPoint" => {
                self.previous_point = V1CourseCalculationsPreviousPoint::from_value(value)
            }

            &_ => {
                log::warn!(
                    "V1CourseApi: Unknown value to update {:?}::{:?}",
                    path,
                    value
                );
            }
        }
    }

    pub fn get_f64_for_path(&self, path: &mut [&str]) -> Result<f64, SignalKGetError> {
        self.get_path(path)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct V1CourseCalculationsPreviousPoint {
    pub distance: Option<V2NumberValue>,
}

impl Path<f64> for V1CourseCalculationsPreviousPoint {
    fn get_path(&self, path: &[&str]) -> Result<f64, SignalKGetError> {
        debug!("V1CourseNextPoint::get_path({:?})", path);
        match path[0] {
            "distance" => get_f64_value(&self.distance),
            &_ => Err(SignalKGetError::NoSuchPath),
        }
    }
}

impl V1CourseCalculationsPreviousPoint {
    pub fn from_value(value: &Value) -> Option<V1CourseCalculationsPreviousPoint> {
        if value.is_null() {
            None
        } else {
            let route_result: Result<V1CourseCalculationsPreviousPoint, serde_json::Error> =
                serde_json::from_value(value.clone());
            if let Ok(route_value) = route_result {
                Some(route_value)
            } else {
                None
            }
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(untagged)]
pub enum V1CourseCalculationsMethod {
    Expanded(V1CourseCalculationsExpandedMethod),
    Value(V1CourseCalculationsMethodValue),
}

impl Path<f64> for V1CourseCalculationsMethod {
    fn get_path(&self, path: &[&str]) -> Result<f64, SignalKGetError> {
        debug!("V1CourseCalculationsMethod::get_path({:?})", path);
        match self {
            V1CourseCalculationsMethod::Expanded(v) => v.get_path(path),
            V1CourseCalculationsMethod::Value(v) => v.get_path(path),
        }
    }
}

impl Default for V1CourseCalculationsMethod {
    fn default() -> Self {
        V1CourseCalculationsMethod::Value(V1CourseCalculationsMethodValue::default())
    }
}
#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct V1CourseCalculationsExpandedMethod {
    pub value: Option<V1CourseCalculationsMethodValue>,
    #[serde(flatten)]
    pub common_value_fields: Option<V1CommonValueFields>,
}

impl Path<f64> for V1CourseCalculationsExpandedMethod {
    fn get_path(&self, path: &[&str]) -> Result<f64, SignalKGetError> {
        if let Some(ref v) = self.value {
            v.get_path(path)
        } else {
            Err(SignalKGetError::ValueNotSet)
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub enum V1CourseCalculationsMethodValue {
    #[default]
    GreatCircle,
    Rhumbline,
}

impl Path<f64> for V1CourseCalculationsMethodValue {
    fn get_path(&self, _path: &[&str]) -> Result<f64, SignalKGetError> {
        Err(SignalKGetError::WrongDataType)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct V1Course {
    pub cross_track_error: Option<V1NumberValue>,
    pub bearing_track_true: Option<V2NumberValue>,
    pub bearing_track_magnetic: Option<V1NumberValue>,
    pub active_route: Option<V1ActiveRoute>,
    pub next_point: Option<V1CourseNextPoint>,
    pub previous_point: Option<V1CoursePreviousPoint>,
}

impl Path<f64> for V1Course {
    fn get_path(&self, path: &[&str]) -> Result<f64, SignalKGetError> {
        debug!("V1Course::get_path({:?})", path);
        match path[0] {
            "crossTrackError" => get_f64_value(&self.cross_track_error),
            "bearingTrackTrue" => get_f64_value(&self.bearing_track_true),
            "bearingTrackMagnetic" => get_f64_value(&self.bearing_track_magnetic),
            "activeRoute" => get_path(path, &(self.active_route.as_ref())),
            "nextPoint" => get_path(path, &(self.next_point.as_ref())),
            "previousPoint" => get_path(path, &(self.previous_point.as_ref())),
            &_ => Err(SignalKGetError::NoSuchPath),
        }
    }
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
                self.bearing_track_true = V2NumberValue::from_value(value);
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
            "previousPoint" => {
                if self.previous_point.is_none() {
                    self.previous_point = Some(V1CoursePreviousPoint::default());
                }
                if let Some(ref mut netxt_point) = self.previous_point {
                    path.remove(0);
                    netxt_point.update(path, value);
                }
            }
            &_ => {
                log::warn!("V1Course: Unknown value to update {:?}::{:?}", path, value);
            }
        }
    }

    pub fn get_f64_for_path(&self, path: &mut Vec<&str>) -> Result<f64, SignalKGetError> {
        self.get_path(path)
    }
}

#[derive(Default)]
pub struct V1CourseBuilder {
    cross_track_error: Option<V1NumberValue>,
    bearing_track_true: Option<V2NumberValue>,
    bearing_track_magnetic: Option<V1NumberValue>,
    active_route: Option<V1ActiveRoute>,
    next_point: Option<V1CourseNextPoint>,
    previous_point: Option<V1CoursePreviousPoint>,
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
                self.bearing_track_true = V2NumberValue::from_value(value);
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
    pub fn bearing_track_true(mut self, value: V2NumberValue) -> V1CourseBuilder {
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
    pub fn previous_point(mut self, value: V1CoursePreviousPoint) -> V1CourseBuilder {
        self.previous_point = Some(value);
        self
    }
    pub fn build(self) -> V1Course {
        V1Course {
            cross_track_error: self.cross_track_error,
            bearing_track_true: self.bearing_track_true,
            bearing_track_magnetic: self.bearing_track_magnetic,
            active_route: self.active_route,
            next_point: self.next_point,
            previous_point: self.previous_point,
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

impl Path<f64> for V1ActiveRoute {
    fn get_path(&self, path: &[&str]) -> Result<f64, SignalKGetError> {
        debug!("V1ActiveRoute::get_path({:?})", path);
        match path[0] {
            "href" => Err(SignalKGetError::WrongDataType),
            "estimatedTimeOfArrival" => Err(SignalKGetError::WrongDataType),
            "startTime" => Err(SignalKGetError::WrongDataType),
            &_ => Err(SignalKGetError::NoSuchPath),
        }
    }
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

impl Path<f64> for V1CourseNextPoint {
    fn get_path(&self, path: &[&str]) -> Result<f64, SignalKGetError> {
        debug!("V1CourseNextPoint::get_path({:?})", path);
        match path[0] {
            "value" => Err(SignalKGetError::WrongDataType),
            "distance" => get_f64_value(&self.distance),
            "bearingTrue" => get_f64_value(&self.bearing_true),
            "bearingMagnetic" => get_f64_value(&self.bearing_magnetic),
            "velocityMadeGood" => get_f64_value(&self.velocity_made_good),
            "timeToGo" => get_f64_value(&self.time_to_go),
            "position" => get_path(path, &(self.position.as_ref())),
            "estimatedTimeOfArrival" => Err(SignalKGetError::WrongDataType),
            "arrivalCircle" => get_f64_value(&self.arrival_circle),
            &_ => Err(SignalKGetError::NoSuchPath),
        }
    }
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

    pub fn get_f64_for_path(&self, path: &mut [&str]) -> Result<f64, SignalKGetError> {
        self.get_path(path)
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
            "type" => self.type_ = V1StringValue::from_value(value),
            "href" => self.href = V1StringValue::from_value(value),
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

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct V1CoursePreviousPoint {
    value: Option<V1CoursePreviousPointValue>,
    distance: Option<V1NumberValue>,
    position: Option<V1PositionType>,
    #[serde(flatten)]
    pub common_value_fields: Option<V1CommonValueFields>,
}

impl Path<f64> for V1CoursePreviousPoint {
    fn get_path(&self, path: &[&str]) -> Result<f64, SignalKGetError> {
        match path[0] {
            "value" => Err(SignalKGetError::WrongDataType),
            "distance" => get_f64_value(&self.distance),
            "position" => get_path(path, &(self.position.as_ref())),
            &_ => Err(SignalKGetError::NoSuchPath),
        }
    }
}

impl V1CoursePreviousPoint {
    pub fn update(&mut self, path: &mut Vec<&str>, value: &serde_json::value::Value) {
        match path[0] {
            "value" => {
                if self.value.is_none() {
                    self.value = Some(V1CoursePreviousPointValue::default());
                }
                if let Some(ref mut point_value) = self.value {
                    path.remove(0);
                    point_value.update(path, value);
                }
            }
            "distance" => {
                self.distance = V1NumberValue::from_value(value);
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

            &_ => {
                log::warn!(
                    "V1CoursePreviousPoint: Unknown value to update {:?}::{:?}",
                    path,
                    value
                );
            }
        }
    }
    pub fn get_f64_for_path(&self, path: &mut [&str]) -> Result<f64, SignalKGetError> {
        self.get_path(path)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct V1CoursePreviousPointValue {
    #[serde(rename = "type")]
    type_: Option<V1StringValue>,
    href: Option<V1StringValue>,
}

impl V1CoursePreviousPointValue {
    pub fn update(&mut self, path: &mut Vec<&str>, value: &serde_json::Value) {
        match path[0] {
            "type" => self.type_ = V1StringValue::from_value(value),
            "href" => self.href = V1StringValue::from_value(value),
            &_ => {
                log::warn!(
                    "V1CoursePreviousPointValue: Unknown value to update {:?}::{:?}",
                    path,
                    value
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::helper_functions::get_path;
    use crate::navigation_course::{
        V1Course, V1CourseApi, V1CourseCalculationsModel, V1CourseNextPoint, V1CoursePreviousPoint,
    };
    use crate::SignalKGetError;
    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn course_full_valid() {
        let course_with_point = full_valid_course();
        println!("{:?}", course_with_point);
    }

    fn full_valid_course() -> V1Course {
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
        course_with_point
    }

    fn get_path_from_course_api(path_string: &str) -> Result<f64, SignalKGetError> {
        let course_with_point = make_course_api();
        let path: Vec<&str> = path_string.split('.').collect();
        let result = get_path(&path, &Some(&course_with_point));
        result
    }

    #[test]
    fn course_api_get_active_route_href_as_f64() {
        assert_eq!(
            get_path_from_course_api(".activeRoute.href"),
            Err(SignalKGetError::WrongDataType)
        );
    }
    #[test]
    fn course_api_get_active_route_name_as_f64() {
        assert_eq!(
            get_path_from_course_api(".activeRoute.name"),
            Err(SignalKGetError::WrongDataType)
        );
    }
    #[test]
    fn course_api_get_active_route_point_index_as_f64() {
        assert_eq!(
            get_path_from_course_api(".activeRoute.pointIndex"),
            Err(SignalKGetError::WrongDataType)
        );
    }
    #[test]
    fn course_api_get_active_route_point_total_as_f64() {
        assert_eq!(
            get_path_from_course_api(".activeRoute.pointTotal"),
            Err(SignalKGetError::WrongDataType)
        );
    }
    #[test]
    fn course_api_get_active_route_reverse_as_f64() {
        assert_eq!(
            get_path_from_course_api(".activeRoute.reverse"),
            Err(SignalKGetError::WrongDataType)
        );
    }
    #[test]
    fn course_api_get_next_point_position_latitude_as_f64() {
        init();

        assert_eq!(
            get_path_from_course_api(".nextPoint.position.latitude"),
            Ok(65.4567)
        );
    }
    #[test]
    fn course_api_get_next_point_position_longitude_as_f64() {
        assert_eq!(
            get_path_from_course_api(".nextPoint.position.longitude"),
            Ok(3.3452)
        );
    }
    #[test]
    fn course_api_get_previous_point_position_latitude_as_f64() {
        assert_eq!(
            get_path_from_course_api(".previousPoint.position.latitude"),
            Ok(65.4567)
        );
    }
    #[test]
    fn course_api_get_previous_point_position_longitude_as_f64() {
        assert_eq!(
            get_path_from_course_api(".previousPoint.position.longitude"),
            Ok(3.3452)
        );
    }
    #[test]
    fn course_api_get_start_time_as_f64() {
        assert_eq!(
            get_path_from_course_api(".startTime"),
            Err(SignalKGetError::WrongDataType)
        );
    }
    #[test]
    fn course_api_get_target_arrival_time_as_f64() {
        assert_eq!(
            get_path_from_course_api(".targetArrivalTime"),
            Err(SignalKGetError::WrongDataType)
        );
    }
    #[test]
    fn course_api_get_arrival_circle_as_f64() {
        assert_eq!(
            get_path_from_course_api(".arrivalCircle"),
            Err(SignalKGetError::WrongDataType)
        );
    }

    fn make_course_api() -> V1CourseApi {
        let j = r#"
        {
          "activeRoute": {
            "href": "/resources/routes/ac3a3b2d-07e8-4f25-92bc-98e7c92f7f1a",
            "name": "Here to eternity.",
            "pointIndex": 0,
            "pointTotal": 0,
            "reverse": true
          },
          "nextPoint": {
            "position": {
              "latitude": 65.4567,
              "longitude": 3.3452
            }
          },
          "previousPoint": {
            "position": {
              "latitude": 65.4567,
              "longitude": 3.3452
            }
          },
          "startTime": "2022-04-22T05:02:56.484Z",
          "targetArrivalTime": "2022-04-22T05:02:56.484Z",
          "arrivalCircle": 500
        }"#;
        let course_with_point: V1CourseApi = serde_json::from_str(j).unwrap();
        course_with_point
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

    #[test]
    fn previous_point_valid() {
        let j = r#"
        {
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
        }"#;
        let course_with_point: V1CoursePreviousPoint = serde_json::from_str(j).unwrap();
        println!("{:?}", course_with_point);
    }

    #[test]
    fn course_api_valid() {
        let course_with_point = make_course_api();
        println!("{:?}", course_with_point);
    }

    #[test]
    fn course_calc_values_valid() {
        let j = r#"
        {
          "calcMethod": "Rhumbline",
          "crossTrackError": 458.784,
          "bearingTrackTrue": 4.58491,
          "bearingTrackMagnetic": 4.51234,
          "estimatedTimeOfArrival": "2022-04-22T05:02:56.484Z",
          "distance": 10157,
          "bearingTrue": 4.58491,
          "bearingMagnetic": 4.51234,
          "velocityMadeGood": 7.2653,
          "timeToGo": 8491,
          "targetSpeed": 2.2653,
          "previousPoint": {
            "distance": 10157
          }
        }"#;
        println!("{:?}", j);
        let course_with_point: V1CourseCalculationsModel = serde_json::from_str(j).unwrap();
        println!("{:?}", course_with_point);
    }
}
