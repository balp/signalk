//! # Signal K
//!
//! `signalk` is a collections of types to serialize and deserialize the
//! signal-k protocol.

use serde::{Deserialize, Serialize};

pub use definitions::{
    V1Attr, V1CommonValueFields, V1DefSource, V1Meta, V1MetaZone, V1NumberValue,
};
pub use delta::{V1DeltaFormat, V1UpdateMeta, V1UpdateType, V1UpdateValue, V1UpdateValueType};
pub use discovery::{V1Discovery, V1DiscoveryEndpoint, V1DiscoveryServer};
pub use electrical::{V1ACBus, V1Electrical, V1ElectricalACQualities, V1ElectricalIdentity};
pub use environment::{
    V1Environment, V1EnvironmentCurrent, V1EnvironmentCurrentValue, V1EnvironmentDepth,
    V1EnvironmentInside, V1EnvironmentTime,
};
pub use full::V1FullFormat;
pub use hello::V1Hello;
pub use navigation::{V1Navigation, V1PositionType, V1PositionValue};
pub use notification::{V1Notification, V1NotificationValue};
pub use propulsion::V1Propulsion;
pub use put::{V1Put, V1PutValue};
pub use sources::{V1Source, V1SourceProperty, V1Sources};
pub use subscribe::{V1Subscribe, V1Subscription};
pub use unsubscribe::{V1Unsubscribe, V1Unsubscription};
pub use vessel::V1Vessel;

pub mod communication;
pub mod definitions;
pub mod delta;
mod design;
pub mod discovery;
pub mod electrical;
pub mod environment;
pub mod full;
pub mod hello;
mod helper_functions;
pub mod navigation;
mod navigation_course;
mod navigation_gnss;
pub mod notification;
mod performance;
pub mod propulsion;
pub mod put;
pub mod sources;
mod steering;
pub mod subscribe;
pub mod unsubscribe;
pub mod vessel;

/// Type for messages that can be received over the signal-k stream
#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(untagged)]
pub enum SignalKStreamMessage {
    Hello(V1Hello),
    Full(V1FullFormat),
    Delta(V1DeltaFormat),
    #[default]
    BadData,
}

/// Possible Errors gen getting data from SignalK storage
#[derive(Debug, PartialEq)]
pub enum SignalKGetError {
    NoSuchPath,
    WrongDataType,
    ValueNotSet,
    TBD,
}

/// Keep data from Signal-K
///
/// This struct keeps data for a signalk environment, it can be reviced by a full
/// message and update with a delta message.
///
/// # Examples
///
/// ```
/// use signalk::{Storage, V1FullFormat};
/// let j = r#"
///         { "self":"vessels.urn:mrn:imo:mmsi:366982330",
///           "vessels": {
///              "urn:mrn:imo:mmsi:366982330": {
///                "mmsi": "366982330"
///              }
///           },
///           "version": "1.0.0"
///         }"#;
/// let full_data: V1FullFormat = serde_json::from_str(j).unwrap();
/// let storage = Storage::new(full_data);
/// assert_eq!(
///    storage.data().get_self().unwrap().mmsi.clone().unwrap(),
///    "366982330".to_string())
/// ```
///
#[derive(Debug, Default)]
pub struct Storage {
    data: V1FullFormat,
}

impl Storage {
    /// Set the id of the self vessel.
    pub fn set_self(&mut self, value: &str) {
        self.data.self_ = value.to_string();
    }

    /// Apply a Delta change to the structure
    ///
    /// # Examples
    /// ```
    /// use signalk::{Storage, V1DeltaFormat, V1FullFormat};
    /// let mut storage = Storage::new(V1FullFormat::default());
    /// let j = r#"{"updates": [
    ///                {"values": [{"path": "navigation.courseOverGroundTrue", "value": 123.45 }]}
    ///             ],
    ///             "context": "vessels.urn:mrn:signalk:uuid:7980c650-6871-45d5-be0f-205d2efadacc"
    /// }"#;
    /// let delta_message: V1DeltaFormat = serde_json::from_str(j).unwrap();
    /// storage.update(&delta_message);
    /// println!("{:?}", storage);
    /// assert_eq!(
    ///   storage.get_f64_for_path(
    ///     "vessels.urn:mrn:signalk:uuid:7980c650-6871-45d5-be0f-205d2efadacc.navigation.courseOverGroundTrue".to_string())
    ///     .unwrap(),
    ///   123.45)
    /// ```
    pub fn update(&mut self, delta: &V1DeltaFormat) {
        self.data.apply_delta(delta);
    }

    /// Get a clone of the stored data
    ///
    /// # Examples
    /// ```
    /// use signalk::{Storage, V1FullFormat};
    /// let data = V1FullFormat::default();
    /// let storage = Storage::new(data.clone());
    ///
    /// assert_eq!(data, storage.get());
    /// ```
    pub fn get(&self) -> V1FullFormat {
        (self.data).clone()
    }

    /// Get a reference to the stored data
    ///
    /// # Examples
    /// ```
    /// use signalk::{Storage, V1FullFormat};
    /// let data = V1FullFormat::default();
    /// let storage = Storage::new(data.clone());
    ///
    /// assert_eq!(&data, storage.data());
    /// ```
    pub fn data(&self) -> &V1FullFormat {
        &self.data
    }

    /// Create a new structure with data.
    ///
    /// # Examples
    /// ```
    /// use signalk::{Storage, V1FullFormat};
    /// let data = V1FullFormat::default();
    /// let storage = Storage::new(data.clone());
    ///
    /// assert_eq!(data, storage.get());
    /// ```
    pub fn new(data: V1FullFormat) -> Self {
        Self { data }
    }

    /// Return the f64 value stored for a SignalK path
    ///
    /// # Examples
    /// ```
    /// use signalk::{Storage, V1FullFormat};
    /// let j = r#"
    ///         { "self":"vessels.urn:mrn:imo:mmsi:366982330",
    ///           "vessels": {
    ///              "urn:mrn:imo:mmsi:366982330": {
    ///                "navigation": {
    ///                  "courseOverGroundTrue": {
    ///                    "value": 245.69,
    ///                    "timestamp": "2015-01-25T12:01:01Z",
    ///                    "$source": "a.suitable.path"
    ///                  }
    ///                }
    ///              }
    ///           },
    ///           "version": "1.0.0"
    ///         }"#;
    /// let storage = Storage::new(serde_json::from_str(j).unwrap());
    ///
    /// assert_eq!(
    ///   storage.get_f64_for_path("self.navigation.courseOverGroundTrue".to_string())
    ///     .unwrap(),
    ///   245.69)
    /// ```
    pub fn get_f64_for_path(&self, path: String) -> Result<f64, SignalKGetError> {
        self.data.get_f64_for_path(path)
    }
}

#[cfg(test)]
mod storage_tests {
    use serde_json::{Number, Value};

    use crate::{
        Storage, V1DeltaFormat, V1FullFormat, V1Navigation, V1NumberValue, V1UpdateType,
        V1UpdateValue, V1Vessel,
    };

    #[test]
    fn get_gives_default() {
        let storage = Storage::default();
        let expected = V1FullFormat::default();
        assert_eq!(expected, storage.get())
    }

    #[test]
    fn apply_delta_for_sog_5_6() {
        let mut storage = Storage::default();
        let expected = V1FullFormat::builder()
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
        let delta = V1DeltaFormat::builder()
            .context("vessels.urn:mrn:imo:mmsi:366982330".into())
            .add_update(
                V1UpdateType::builder()
                    .add_update(V1UpdateValue::new(
                        "navigation.speedOverGround".into(),
                        Value::Number(Number::from_f64(5.6).unwrap()),
                    ))
                    .build(),
            )
            .build();
        storage.update(&delta);
        assert_eq!(expected, storage.get())
    }

    #[test]
    fn apply_delta_for_sog_15_8() {
        let mut storage = Storage::default();
        let expected = V1FullFormat::builder()
            .add_vessel(
                "urn:mrn:imo:mmsi:366982330".into(),
                V1Vessel::builder()
                    .mmsi("366982330".into())
                    .navigation(
                        V1Navigation::builder()
                            .speed_over_ground(V1NumberValue::builder().value(15.8).build())
                            .build(),
                    )
                    .build(),
            )
            .build();
        let delta = V1DeltaFormat::builder()
            .context("vessels.urn:mrn:imo:mmsi:366982330".into())
            .add_update(
                V1UpdateType::builder()
                    .add_update(V1UpdateValue::new(
                        "navigation.speedOverGround".into(),
                        Value::Number(Number::from_f64(15.8).unwrap()),
                    ))
                    .build(),
            )
            .build();
        storage.update(&delta);
        assert_eq!(expected, storage.get())
    }
}
