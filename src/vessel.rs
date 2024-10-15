use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::communication::V1Communication;
use crate::design::V1Design;
use crate::electrical::V1Electrical;
use crate::environment::V1Environment;
use crate::full::Updatable;
use crate::notification::V1Notification;
use crate::{SignalKGetError, V1Navigation, V1Propulsion, V1UpdateType};

/// An object describing an individual vessel. It should be an object in vessels,
/// named using MMSI or a UUID
#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct V1Vessel {
    /// MMSI number of the vessel, if available.
    pub mmsi: Option<String>,

    /// URL based identity of the vessel, if available.
    pub url: Option<String>,

    /// A unique Signal K flavoured maritime resource identifier, assigned by the server.
    pub uuid: Option<String>,

    /// MMSI number of the mothership of this vessel, if available.
    pub mothership_mmsi: Option<String>,

    /// The common name of the vessel
    pub name: Option<String>,

    /// The home port of the vessel
    pub port: Option<String>,

    /// The country of ship registration, or flag state of the vessel
    pub flag: Option<String>,

    /// Navigation data including Position, Course to next WP information, etc.
    pub navigation: Option<V1Navigation>,

    // pub registrations: Option<HashMap<String, V1Registration>>,
    pub communication: Option<V1Communication>,
    /// Environmental data measured locally including Depth, Wind, Temp, etc.
    pub environment: Option<V1Environment>,

    /// Electrical data, each electrical device indentified by a unique name i.e. Battery_1
    pub electrical: Option<V1Electrical>,

    /// Notifications currently raised. Major categories have well-defined names, but the tree can be extended by any hierarchical structure
    // pub notifications: Option<V1Notification>,

    // pub steering: Option<V1Steering>,
    // pub tanks: Option<V1Tanks>,
    pub design: Option<V1Design>,
    // pub sails: Option<V1Sails>,
    // pub sensors: Option<V1Sensors>,
    // pub performance: Option<V1Performance>,
    /// Engine data, each engine identified by a unique name i.e. Port_Engine
    pub propulsion: Option<HashMap<String, V1Propulsion>>,
}

impl Updatable for V1Vessel {
    fn apply_update(&mut self, update: &V1UpdateType) {
        log::debug!("Apply update: {:?}", update);
        if let Some(ref values) = update.values {
            for value in values.iter() {
                let mut path: Vec<&str> = value.path.split('.').collect();
                self.update(&mut path, &value.value);
            }
        }
    }

    fn id(&self) -> String {
        if let Some(ref id) = self.mmsi {
            return id.clone();
        }
        if let Some(ref id) = self.uuid {
            return id.clone();
        }
        "".into()
    }

    fn type_name(&self) -> String {
        "V1Vessel".to_string()
    }
}

impl V1Vessel {
    pub fn builder() -> V1VesselBuilder {
        V1VesselBuilder::default()
    }

    pub fn new_with_id(id: &str) -> Self {
        let id_parts: Vec<&str> = id.split(':').collect();
        if id_parts.len() != 5 && id_parts[0] != "urn" && id_parts[1] != "mrn" {
            Self::default()
        } else if id_parts[2] == "signalk" {
            Self::builder().uuid(id_parts[4].to_string()).build()
        } else if id_parts[2] == "imo" {
            Self::builder().mmsi(id_parts[4].to_string()).build()
        } else {
            Self::default()
        }
    }
    pub fn update(&mut self, path: &mut Vec<&str>, value: &serde_json::value::Value) {
        if path.is_empty() {
            return;
        }
        match path[0] {
            "mmsi" => {
                if let serde_json::Value::String(ref string) = value {
                    self.mmsi = Some(string.to_string());
                }
            }
            "url" => {
                if let serde_json::Value::String(ref string) = value {
                    self.url = Some(string.to_string());
                }
            }
            "uuid" => {
                if let serde_json::Value::String(ref string) = value {
                    self.uuid = Some(string.to_string());
                }
            }
            "mothership_mmsi" => {
                if let serde_json::Value::String(ref string) = value {
                    self.mothership_mmsi = Some(string.to_string());
                }
            }
            "name" => {
                if let serde_json::Value::String(ref string) = value {
                    self.name = Some(string.to_string());
                }
            }
            "port" => {
                if let serde_json::Value::String(ref string) = value {
                    self.port = Some(string.to_string());
                }
            }
            "flag" => {
                if let serde_json::Value::String(ref string) = value {
                    self.flag = Some(string.to_string());
                }
            }
            "navigation" => {
                if self.navigation.is_none() {
                    self.navigation = Some(V1Navigation::default());
                }
                if let Some(ref mut navigation) = self.navigation {
                    path.remove(0);
                    navigation.update(path, value);
                }
            }
            "environment" => {
                if self.environment.is_none() {
                    self.environment = Some(V1Environment::default());
                }
                if let Some(ref mut environment) = self.environment {
                    path.remove(0);
                    environment.update(path, value);
                }
            }
            "electrical" => {
                if self.electrical.is_none() {
                    self.electrical = Some(V1Electrical::default());
                }
                if let Some(ref mut electrical) = self.electrical {
                    path.remove(0);
                    electrical.update(path, value);
                }
            }
            "communication" => {
                if self.communication.is_none() {
                    self.communication = Some(V1Communication::default());
                }
                if let Some(ref mut communication) = self.communication {
                    path.remove(0);
                    communication.update(path, value);
                }
            }
            "design" => {
                if self.design.is_none() {
                    self.design = Some(V1Design::default());
                }
                if let Some(ref mut design) = self.design {
                    path.remove(0);
                    design.update(path, value);
                }
            }
            "" => {
                log::debug!("root path to vessel: {:?}::{:?}", path, value);
                if let serde_json::Value::Object(ref map) = value {
                    for (k, v) in map.iter() {
                        log::debug!(" key: {:?} value: {:?}", k, v);
                        let mut path = vec![k.as_str()];
                        self.update(&mut path, v);
                    }
                }
            }
            &_ => {
                log::warn!("V1Vessel: Unknown update pattern: {:?}::{:?}", path, value);
            }
        }
    }

    pub fn get_f64_for_path(&self, path: &mut Vec<&str>) -> Result<f64, SignalKGetError> {
        match path[0] {
            "mmsi" => Err(SignalKGetError::WrongDataType),
            "url" => Err(SignalKGetError::WrongDataType),
            "uuid" => Err(SignalKGetError::WrongDataType),
            "mothership_mmsi" => Err(SignalKGetError::WrongDataType),
            "name" => Err(SignalKGetError::WrongDataType),
            "port" => Err(SignalKGetError::WrongDataType),
            "flag" => Err(SignalKGetError::WrongDataType),
            "navigation" => {
                if let Some(ref navigation) = self.navigation {
                    path.remove(0);
                    navigation.get_f64_for_path(path)
                } else {
                    Err(SignalKGetError::NoSuchPath)
                }
            }
            "environment" => {
                if let Some(ref environment) = self.environment {
                    path.remove(0);
                    environment.get_f64_for_path(path)
                } else {
                    Err(SignalKGetError::NoSuchPath)
                }
            }
            "electrical" => {
                log::info!("get: path {:?}", path);
                if let Some(ref electrical) = self.electrical {
                    path.remove(0);
                    electrical.get_f64_for_path(path)
                } else {
                    Err(SignalKGetError::NoSuchPath)
                }
            }
            "notifications" => Err(SignalKGetError::TBD),
            "steering" => Err(SignalKGetError::TBD),
            "tanks" => Err(SignalKGetError::TBD),
            "design" => Err(SignalKGetError::TBD),
            "sails" => Err(SignalKGetError::TBD),
            "sensors" => Err(SignalKGetError::TBD),
            "performance" => Err(SignalKGetError::TBD),
            "propulsion" => Err(SignalKGetError::TBD),
            &_ => Err(SignalKGetError::NoSuchPath),
        }
        // Ok(5.1)
    }
}

#[derive(Default)]
pub struct V1VesselBuilder {
    mmsi: Option<String>,
    url: Option<String>,
    uuid: Option<String>,
    mothership_mmsi: Option<String>,
    name: Option<String>,
    flag: Option<String>,
    port: Option<String>,
    navigation: Option<V1Navigation>,
    communication: Option<V1Communication>,
    environment: Option<V1Environment>,
    electrical: Option<V1Electrical>,
    notifications: Option<V1Notification>,
    propulsion: Option<HashMap<String, V1Propulsion>>,
    design: Option<V1Design>,
}

impl V1VesselBuilder {
    pub fn uuid(mut self, value: String) -> V1VesselBuilder {
        self.uuid = Some(value);
        self
    }
    pub fn url(mut self, value: String) -> V1VesselBuilder {
        self.url = Some(value);
        self
    }
    pub fn mmsi(mut self, value: String) -> V1VesselBuilder {
        self.mmsi = Some(value);
        self
    }
    pub fn mothership_mmsi(mut self, value: String) -> V1VesselBuilder {
        self.mothership_mmsi = Some(value);
        self
    }
    pub fn name(mut self, value: String) -> V1VesselBuilder {
        self.name = Some(value);
        self
    }
    pub fn port(mut self, value: String) -> V1VesselBuilder {
        self.port = Some(value);
        self
    }
    pub fn flag(mut self, value: String) -> V1VesselBuilder {
        self.flag = Some(value);
        self
    }
    pub fn navigation(mut self, value: V1Navigation) -> V1VesselBuilder {
        self.navigation = Some(value);
        self
    }
    pub fn communication(mut self, value: V1Communication) -> V1VesselBuilder {
        self.communication = Some(value);
        self
    }
    pub fn electrical(mut self, value: V1Electrical) -> V1VesselBuilder {
        self.electrical = Some(value);
        self
    }
    pub fn environment(mut self, value: V1Environment) -> V1VesselBuilder {
        self.environment = Some(value);
        self
    }
    pub fn notifications(mut self, value: V1Notification) -> V1VesselBuilder {
        self.notifications = Some(value);
        self
    }
    pub fn design(mut self, value: V1Design) -> V1VesselBuilder {
        self.design = Some(value);
        self
    }
    pub fn add_propulsion(mut self, key: String, value: V1Propulsion) -> V1VesselBuilder {
        if self.propulsion.is_none() {
            self.propulsion = Some(HashMap::new());
        }
        if let Some(ref mut x) = self.propulsion {
            x.insert(key, value);
        }
        self
    }
    pub fn build(self) -> V1Vessel {
        V1Vessel {
            uuid: self.uuid,
            mmsi: self.mmsi,
            name: self.name,
            port: self.port,
            flag: self.flag,
            navigation: self.navigation,
            communication: self.communication,
            environment: self.environment,
            electrical: self.electrical,
            // notifications: self.notifications,
            design: self.design,
            propulsion: self.propulsion,
            url: self.url,
            mothership_mmsi: self.mothership_mmsi,
        }
    }
}

#[cfg(test)]
mod context_tests {
    use serde_json::{Number, Value};

    use crate::full::Updatable;
    use crate::{V1Navigation, V1NumberValue, V1UpdateType, V1UpdateValue, V1Vessel};

    #[test]
    fn update_navigation_sog_12_6_in_existing_tree() {
        let mut vessel = V1Vessel::builder()
            .navigation(
                V1Navigation::builder()
                    .speed_over_ground(V1NumberValue::builder().value(10.0).build())
                    .build(),
            )
            .build();
        let update = V1UpdateType::builder()
            .add_update(V1UpdateValue::new(
                "navigation.speedOverGround".into(),
                Value::Number(Number::from_f64(12.6).unwrap()),
            ))
            .build();

        vessel.apply_update(&update);

        assert_eq!(
            vessel
                .navigation
                .unwrap()
                .speed_over_ground
                .unwrap()
                .value
                .unwrap(),
            12.6
        );
    }

    #[test]
    fn update_navigation_sog_5_1_in_existing_tree() {
        let mut vessel = V1Vessel::builder()
            .navigation(
                V1Navigation::builder()
                    .speed_over_ground(V1NumberValue::builder().value(10.0).build())
                    .build(),
            )
            .build();
        let update = V1UpdateType::builder()
            .add_update(V1UpdateValue::new(
                "navigation.speedOverGround".into(),
                Value::Number(Number::from_f64(5.1).unwrap()),
            ))
            .build();

        vessel.apply_update(&update);

        assert_eq!(
            vessel
                .navigation
                .unwrap()
                .speed_over_ground
                .unwrap()
                .value
                .unwrap(),
            5.1
        );
    }

    #[test]
    fn update_navigation_sog_and_cog_true() {
        let mut vessel = V1Vessel::builder()
            .navigation(
                V1Navigation::builder()
                    .speed_over_ground(V1NumberValue::builder().value(10.0).build())
                    .course_over_ground_true(V1NumberValue::builder().value(0.0).build())
                    .build(),
            )
            .build();
        let update = V1UpdateType::builder()
            .add_update(V1UpdateValue::new(
                "navigation.speedOverGround".into(),
                Value::Number(Number::from_f64(7.2).unwrap()),
            ))
            .add_update(V1UpdateValue::new(
                "navigation.courseOverGroundTrue".into(),
                Value::Number(Number::from_f64(4.71238898).unwrap()),
            ))
            .build();

        vessel.apply_update(&update);

        assert_eq!(
            vessel
                .navigation
                .as_ref()
                .unwrap()
                .speed_over_ground
                .as_ref()
                .unwrap()
                .value
                .unwrap(),
            7.2
        );
        assert_eq!(
            vessel
                .navigation
                .as_ref()
                .unwrap()
                .course_over_ground_true
                .as_ref()
                .unwrap()
                .value
                .unwrap(),
            4.71238898
        );
    }

    #[test]
    fn new_from_id() {
        let vessel = V1Vessel::new_with_id("urn:mrn:imo:mmsi:366982330");
        assert_eq!(vessel.mmsi, Some("366982330".to_string()));
    }
}
