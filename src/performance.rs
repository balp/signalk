use crate::definitions::{V1StringValue, V2NumberValue};
use crate::helper_functions::{get_f64_value, F64CompatiblePath};
use crate::{SignalKGetError, V1CommonValueFields, V1NumberValue, V1Source};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct V1Performance {
    pub polars: Option<V1PolarUuid>,
    pub active_polar: Option<V1StringValue>,
    pub active_polar_data: Option<V1Polar>,
    pub polar_speed: Option<V1NumberValue>,
    pub polar_speed_ratio: Option<V1NumberValue>,
    pub velocity_made_good: Option<V1NumberValue>,
    pub velocity_made_good_to_waypoint: Option<V1NumberValue>,
    pub beat_angle: Option<V1NumberValue>,
    pub beat_angle_velocity_made_good: Option<V1NumberValue>,
    pub beat_angle_target_speed: Option<V1NumberValue>,
    pub gybe_angle: Option<V1NumberValue>,
    pub gybe_angle_velocity_made_good: Option<V1NumberValue>,
    pub gybe_angle_target_speed: Option<V1NumberValue>,
    pub target_angle: Option<V1NumberValue>,
    pub target_speed: Option<V1NumberValue>,
    pub leeway: Option<V1NumberValue>,
    pub tack_magnetic: Option<V1NumberValue>,
    pub tack_true: Option<V1NumberValue>,
}

impl F64CompatiblePath for V1Performance {
    fn get_f64_for_path(&self, path: &mut Vec<&str>) -> Result<f64, SignalKGetError> {
        match path[0] {
            "polars" => Err(SignalKGetError::WrongDataType),
            "activePolar" => Err(SignalKGetError::WrongDataType),
            "activePolarData" => Err(SignalKGetError::WrongDataType),
            "polarSpeed" => get_f64_value(&self.polar_speed),
            "polarSpeedRatio" => get_f64_value(&self.polar_speed_ratio),
            "velocityMadeGood" => get_f64_value(&self.velocity_made_good),
            "velocityMadeGoodToWaypoint" => get_f64_value(&self.velocity_made_good_to_waypoint),
            "beatAngle" => get_f64_value(&self.beat_angle),
            "beatAngleVelocityMadeGood" => get_f64_value(&self.beat_angle_velocity_made_good),
            "beatAngleTargetSpeed" => get_f64_value(&self.beat_angle_target_speed),
            "gybeAngle" => get_f64_value(&self.gybe_angle),
            "gybeAngleVelocityMadeGood" => get_f64_value(&self.gybe_angle_velocity_made_good),
            "gybeAngleTargetSpeed" => get_f64_value(&self.gybe_angle_target_speed),
            "targetAngle" => get_f64_value(&self.target_angle),
            "targetSpeed" => get_f64_value(&self.target_speed),
            "leeway" => get_f64_value(&self.leeway),
            "tackMagnetic" => get_f64_value(&self.tack_magnetic),
            "tackTrue" => get_f64_value(&self.tack_true),
            &_ => Err(SignalKGetError::NoSuchPath),
        }
    }
}

impl V1Performance {
    pub fn update(&mut self, path: &mut Vec<&str>, value: &serde_json::value::Value) {
        if path.is_empty() {
            return;
        }
        match path[0] {
            // "polars" => self.polars = V1NumberValue::from_value(value),
            "activePolar" => self.active_polar = V1StringValue::from_value(value),
            // "activePolarData" => self.active_polar_data = V1NumberValue::from_value(value),
            "polarSpeed" => self.polar_speed = V1NumberValue::from_value(value),
            "polarSpeedRatio" => self.polar_speed_ratio = V1NumberValue::from_value(value),
            "velocityMadeGood" => self.velocity_made_good = V1NumberValue::from_value(value),
            "velocityMadeGoodToWaypoint" => {
                self.velocity_made_good_to_waypoint = V1NumberValue::from_value(value)
            }
            "beatAngle" => self.beat_angle = V1NumberValue::from_value(value),
            "beatAngleVelocityMadeGood" => {
                self.beat_angle_velocity_made_good = V1NumberValue::from_value(value)
            }
            "beatAngleTargetSpeed" => {
                self.beat_angle_target_speed = V1NumberValue::from_value(value)
            }
            "gybeAngle" => self.gybe_angle = V1NumberValue::from_value(value),
            "gybeAngleVelocityMadeGood" => {
                self.gybe_angle_velocity_made_good = V1NumberValue::from_value(value)
            }
            "gybeAngleTargetSpeed" => {
                self.gybe_angle_target_speed = V1NumberValue::from_value(value)
            }
            "targetAngle" => self.target_angle = V1NumberValue::from_value(value),
            "targetSpeed" => self.target_speed = V1NumberValue::from_value(value),
            "leeway" => self.leeway = V1NumberValue::from_value(value),
            "tackMagnetic" => self.tack_magnetic = V1NumberValue::from_value(value),
            "tackTrue" => self.tack_true = V1NumberValue::from_value(value),

            &_ => {
                log::warn!(
                    "V1Performance: Unknown update pattern: {:?}::{:?}",
                    path,
                    value
                );
            }
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct V1PolarUuid {
    #[serde(flatten)]
    pub value: Option<V1StringValue>,
    #[serde(flatten)]
    pub common_value_fields: Option<V1CommonValueFields>,
}

impl V1PolarUuid {
    pub fn from_value(value: &serde_json::value::Value) -> Option<Self> {
        if value.is_null() {
            None
        } else {
            let ship_type_result: Result<Self, serde_json::Error> =
                serde_json::from_value(value.clone());
            if let Ok(ship_type_value) = ship_type_result {
                Some(ship_type_value)
            } else {
                None
            }
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct V1Polar {
    pub id: V1StringValue,
    pub name: V1StringValue,
    // pub description: V1StringValue,
    pub source: Option<V1Source>,
    pub wind_data: Option<Vec<V1PolarWindData>>,
}

impl V1Polar {
    pub fn from_value(value: &serde_json::value::Value) -> Option<Self> {
        if value.is_null() {
            None
        } else {
            let ship_type_result: Result<Self, serde_json::Error> =
                serde_json::from_value(value.clone());
            if let Ok(ship_type_value) = ship_type_result {
                Some(ship_type_value)
            } else {
                None
            }
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct V1PolarWindData {
    pub true_wind_speed: V2NumberValue,
    pub optimal_beats: Option<Vec<Vec<V2NumberValue>>>,
    pub optimal_gybes: Option<Vec<Vec<V2NumberValue>>>,
    pub angle_data: Vec<Vec<V2NumberValue>>,
}
