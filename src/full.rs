use log::debug;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::helper_functions::{get_path, Path};
use crate::{SignalKGetError, V1DeltaFormat, V1Sources, V1UpdateType, V1Vessel};

/// These items can be updated by a V1UpdateType
///
/// Given the GetContext Trait one can fine a matching object
/// to apply the update into from the V1DeltaFormat's context
pub trait Updatable {
    /// This will change the found object to have a a new value
    fn apply_update(&mut self, update: &V1UpdateType);

    /// This will return the ID if possible for the object
    ///
    /// At the moment this is mainly to make sure we can easily
    /// verify the GetContext functionality
    fn id(&self) -> String;

    /// This will return the "type" of the context
    ///
    /// At the moment this is mainly to make sure we can easily
    /// verify the GetContext functionality
    fn type_name(&self) -> String;
}

/// Root structure for Full Signal K data
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct V1FullFormat {
    /// Version of the schema and APIs that this data is using in Canonical format i.e. V1.7.0.
    pub version: String,

    /// This holds the context (prefix + UUID, MMSI or URL in dot notation) of the server's self object.
    #[serde(rename = "self")]
    pub self_: String,

    /// A wrapper object for vessel objects, each describing vessels in range, including this vessel.
    pub vessels: Option<HashMap<String, V1Vessel>>,
    // TODO: Add aircraft
    // TODO: Add aton
    // TODO: Add sar
    // Metadata about the data sources; physical interface, address, protocol, etc.
    // pub sources: Option<V1Sources>,
}

impl Path<f64> for V1FullFormat {
    fn get_path(&self, path: &[&str]) -> Result<f64, SignalKGetError> {
        debug!("get_path({:?})", path);
        match path[0] {
            "version" => Err(SignalKGetError::WrongDataType),
            "self" => get_path(path, &self.get_self()),
            "vessels" => Err(SignalKGetError::TBD),
            "aircraft" => Err(SignalKGetError::TBD),
            "aton" => Err(SignalKGetError::TBD),
            "sar" => Err(SignalKGetError::TBD),
            &_ => Err(SignalKGetError::NoSuchPath),
        }
    }
}

impl Default for V1FullFormat {
    fn default() -> Self {
        V1FullFormat {
            version: "1.7.0".to_string(),
            self_: "".to_string(),
            vessels: None,
            // sources: None,
        }
    }
}

impl V1FullFormat {
    /// Returns a builder for the Full Formal Signal K structure
    ///
    /// As the structure is a bit complex to create it's recommended to
    ///use this builder pattern to create new instances.
    pub fn builder() -> V1FullFormatBuilder {
        V1FullFormatBuilder::default()
    }

    pub fn apply_delta(&mut self, delta: &V1DeltaFormat) {
        if let Some(ref context) = delta.context {
            let v: Vec<&str> = context.split('.').collect();
            if v.len() < 2 {
                return;
            }
            if v[0] == "vessels" {
                if self.vessels.is_none() {
                    self.vessels = Some(HashMap::new());
                }
                if let Some(ref mut vessels) = self.vessels {
                    let id = v[1].to_string();
                    if !vessels.contains_key(&id) {
                        vessels.insert(id.clone(), V1Vessel::new_with_id(&id));
                    }
                    let mut t = vessels.get_mut(&id);
                    if let Some(ref mut vessel) = t {
                        for update in &delta.updates {
                            vessel.apply_update(update);
                        }
                    }
                }
            }
        }
    }

    pub fn get_f64_for_path(&self, path: String) -> Result<f64, SignalKGetError> {
        let mut path_que: Vec<&str> = path.split('.').collect();
        log::debug!("get_f64_for_path(&self, {:?})", path_que);
        match path_que[0] {
            "version" => Err(SignalKGetError::WrongDataType),
            "self" => {
                let self_path: Vec<&str> = self.self_.split('.').collect();
                if self_path[0] == "vessels" {
                    if let Some(ref vessels) = self.vessels {
                        if let Some(vessel) = vessels.get(self_path[1]) {
                            path_que.remove(0);
                            vessel.get_f64_for_path(&mut path_que)
                        } else {
                            Err(SignalKGetError::NoSuchPath)
                        }
                    } else {
                        Err(SignalKGetError::NoSuchPath)
                    }
                } else {
                    Err(SignalKGetError::NoSuchPath)
                }
            }
            "vessels" => {
                if let Some(ref vessels) = self.vessels {
                    if let Some(vessel) = vessels.get(path_que[1]) {
                        path_que.remove(0);
                        path_que.remove(0);
                        vessel.get_f64_for_path(&mut path_que)
                    } else {
                        Err(SignalKGetError::NoSuchPath)
                    }
                } else {
                    Err(SignalKGetError::NoSuchPath)
                }
            }
            &_ => Err(SignalKGetError::NoSuchPath),
        }
    }

    pub fn get_self(&self) -> Option<&V1Vessel> {
        if let Some(vessels) = self.vessels.as_ref() {
            let v: Vec<&str> = self.self_.split('.').collect();
            if v.len() >= 2 {
                return vessels.get(v[1]);
            } else {
                return vessels.get(v[0]);
            }
        };
        None
    }
}

/// Builder for the Signal K Full format structure
pub struct V1FullFormatBuilder {
    version: String,
    self_: String,
    vessels: Option<HashMap<String, V1Vessel>>,
    sources: Option<V1Sources>,
}

impl Default for V1FullFormatBuilder {
    fn default() -> Self {
        V1FullFormatBuilder {
            version: "1.7.0".to_string(),
            self_: "".to_string(),
            vessels: None,
            sources: None,
        }
    }
}

impl V1FullFormatBuilder {
    pub fn version(mut self, version: String) -> V1FullFormatBuilder {
        self.version = version;
        self
    }
    pub fn self_(mut self, self_: String) -> V1FullFormatBuilder {
        self.self_ = self_;
        self
    }
    pub fn add_vessel(mut self, key: String, vessel: V1Vessel) -> V1FullFormatBuilder {
        if self.vessels.is_none() {
            self.vessels = Some(HashMap::new());
        }
        if let Some(ref mut x) = self.vessels {
            x.insert(key, vessel);
        }
        self
    }
    pub fn sources(mut self, sources: V1Sources) -> V1FullFormatBuilder {
        self.sources = Some(sources);
        self
    }
    pub fn build(self) -> V1FullFormat {
        V1FullFormat {
            version: self.version,
            self_: self.self_,
            vessels: self.vessels,
            // sources: self.sources,
        }
    }
}

#[cfg(test)]
mod context_tests {
    use serde_json::{Number, Value};

    use crate::{
        V1DeltaFormat, V1FullFormat, V1Navigation, V1NumberValue, V1UpdateType, V1UpdateValue,
        V1Vessel,
    };

    #[test]
    fn update_existing_mmsi() {
        let mut data = make_366982330_vessel();
        let delta = make_speed_delta_for_366982330();
        data.apply_delta(&delta);
        assert_speed_is_5_1(&mut data);
    }

    #[test]
    fn update_new_mmsi() {
        let mut data = V1FullFormat::builder().build();

        let delta = make_speed_delta_for_366982330();
        data.apply_delta(&delta);
        assert_speed_is_5_1(&mut data);
    }

    #[test]
    fn get_self_speed_by_path() {
        let mut data = make_366982330_vessel();
        data.self_ = "vessels.urn:mrn:imo:mmsi:366982330".to_string();
        let speed = data.get_f64_for_path("self.navigation.speedOverGround".to_string());
        assert_eq!(speed, Ok(5.6))
    }

    fn assert_speed_is_5_1(data: &mut V1FullFormat) {
        assert_eq!(
            data.vessels
                .as_ref()
                .unwrap()
                .get("urn:mrn:imo:mmsi:366982330")
                .as_ref()
                .unwrap()
                .navigation
                .as_ref()
                .unwrap()
                .speed_over_ground
                .as_ref()
                .unwrap()
                .value
                .unwrap(),
            5.1
        )
    }

    fn make_speed_delta_for_366982330() -> V1DeltaFormat {
        let delta = V1DeltaFormat::builder()
            .context("vessels.urn:mrn:imo:mmsi:366982330".into())
            .add_update(
                V1UpdateType::builder()
                    .add_update(V1UpdateValue::new(
                        "navigation.speedOverGround".into(),
                        Value::Number(Number::from_f64(5.1).unwrap()),
                    ))
                    .build(),
            )
            .build();
        delta
    }

    fn make_366982330_vessel() -> V1FullFormat {
        let data = V1FullFormat::builder()
            .add_vessel(
                "urn:mrn:imo:mmsi:366982330".into(),
                V1Vessel::builder()
                    .mmsi("366982330".into())
                    .navigation(
                        V1Navigation::builder()
                            .speed_over_ground(V1NumberValue::builder().value(5.6).build())
                            .build(),
                    )
                    .build(),
            )
            .build();
        data
    }
}
