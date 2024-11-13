use crate::definitions::V2NumberValue;
use crate::helper_functions::{get_f64_value, get_f64_value_for_path, F64CompatiblePath};
use crate::{SignalKGetError, V1CommonValueFields};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct V1Steering {
    pub rudder_angle: Option<V2NumberValue>,
    pub rudder_angle_target: Option<V2NumberValue>,
    pub autopilot: Option<V1SteeringAutopilot>,
}

impl F64CompatiblePath for V1Steering {
    fn get_f64_for_path(&self, path: &mut Vec<&str>) -> Result<f64, SignalKGetError> {
        match path[0] {
            "rudderAngle" => get_f64_value(&self.rudder_angle),
            "rudderAngleTarget" => get_f64_value(&self.rudder_angle_target),
            "autopilot" => get_f64_value_for_path(path, &self.autopilot),
            &_ => Err(SignalKGetError::NoSuchPath),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct V1SteeringAutopilot {
    pub state: Option<V1SteeringAutopilotState>,
    pub mode: Option<V1SteeringAutopilotMode>,
    pub target: Option<V1SteeringAutopilotTarget>,
    pub dead_zone: Option<V2NumberValue>,
    pub backlash: Option<V2NumberValue>,
    pub gain: Option<V2NumberValue>,
    pub max_drive_current: Option<V2NumberValue>,
    pub max_drive_rate: Option<V2NumberValue>,
    pub port_lock: Option<V2NumberValue>,
    pub starboard_lock: Option<V2NumberValue>,
}

impl F64CompatiblePath for V1SteeringAutopilot {
    fn get_f64_for_path(&self, path: &mut Vec<&str>) -> Result<f64, SignalKGetError> {
        match path[0] {
            &_ => Err(SignalKGetError::NoSuchPath),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(untagged)]
pub enum V1SteeringAutopilotState {
    Expanded(V1SteeringAutopilotStateExpandedValue),
    Value(V1SteeringAutopilotStateValue),
}

impl Default for V1SteeringAutopilotState {
    fn default() -> Self {
        V1SteeringAutopilotState::Value(V1SteeringAutopilotStateValue::default())
    }
}
#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct V1SteeringAutopilotStateExpandedValue {
    pub value: Option<V1SteeringAutopilotStateValue>,
    #[serde(flatten)]
    pub common_value_fields: Option<V1CommonValueFields>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub enum V1SteeringAutopilotStateValue {
    #[default]
    Auto,
    Standby,
    Alarm,
    NoDrift,
    Wind,
    DepthContour,
    Route,
    DirectControl,
}
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(untagged)]
pub enum V1SteeringAutopilotMode {
    Expanded(V1SteeringAutopilotModeExpandedValue),
    Value(V1SteeringAutopilotModeValue),
}

impl Default for V1SteeringAutopilotMode {
    fn default() -> Self {
        V1SteeringAutopilotMode::Value(V1SteeringAutopilotModeValue::default())
    }
}
#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct V1SteeringAutopilotModeExpandedValue {
    pub value: Option<V1SteeringAutopilotModeValue>,
    #[serde(flatten)]
    pub common_value_fields: Option<V1CommonValueFields>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub enum V1SteeringAutopilotModeValue {
    #[default]
    PowerSave,
    Normal,
    Accurate,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct V1SteeringAutopilotTarget {
    pub wind_angle_apparent: Option<V2NumberValue>,
    pub wind_angle_true: Option<V2NumberValue>,
    pub heading_true: Option<V2NumberValue>,
    pub heading_magnetic: Option<V2NumberValue>,
}

impl F64CompatiblePath for V1SteeringAutopilotTarget {
    fn get_f64_for_path(&self, path: &mut Vec<&str>) -> Result<f64, SignalKGetError> {
        match path[0] {
            "windAngleApparent" => get_f64_value(&self.wind_angle_apparent),
            "windAngleTrue" => get_f64_value(&self.wind_angle_true),
            "headingTrue" => get_f64_value(&self.heading_true),
            "headingMagnetic" => get_f64_value(&self.heading_magnetic),
            &_ => Err(SignalKGetError::NoSuchPath),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::steering::{
        V1SteeringAutopilotState, V1SteeringAutopilotStateExpandedValue,
        V1SteeringAutopilotStateValue,
    };

    #[test]
    fn steering_autopilot_value() {
        let j = r#"{
        "value" : "auto",
        "$source": "foo.bar",
        "timestamp": "2014-08-15T19:00:15.402Z"
      }"#;
        println!("{:?}", j);
        let autopilot: V1SteeringAutopilotState = serde_json::from_str(j).unwrap();
        println!("{:?}", autopilot);
    }
    #[test]
    fn steering_autopilot_expanded_value() {
        let j = r#"{
        "value" : "auto",
        "$source": "foo.bar",
        "timestamp": "2014-08-15T19:00:15.402Z"
      }"#;
        println!("{:?}", j);
        let autopilot: V1SteeringAutopilotStateExpandedValue = serde_json::from_str(j).unwrap();
        println!("{:?}", autopilot);
    }
    #[test]
    fn steering_autopilot_state_value() {
        let j = r#" "auto" "#;
        println!("{:?}", j);
        let autopilot: V1SteeringAutopilotStateValue = serde_json::from_str(j).unwrap();
        println!("{:?}", autopilot);
    }
    #[test]
    fn serialization_of_stearing_value() {
        let j = V1SteeringAutopilotStateValue::Auto;
        println!("{:?}", j);
        let autopilot = serde_json::to_string(&j).unwrap();
        println!("{:?}", autopilot);
    }
}
