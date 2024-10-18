use crate::{V1CommonValueFields, V1NumberValue};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct V1gnss {
    #[serde(rename = "type")]
    type_: Option<V1gnssType>,
    method_quality: Option<V1gnssMethodQuality>,
    integrity: Option<V1gnssIntegrity>,
    satellites: Option<V1NumberValue>,
    antenna_altitude: Option<V1NumberValue>,
    horizontal_dilution: Option<V1NumberValue>,
    position_dilution: Option<V1NumberValue>,
    geoidal_separation: Option<V1NumberValue>,
    differential_age: Option<V1NumberValue>,
    satellites_in_view: Option<V1gnssSatellitesInView>,
}

impl V1gnss {
    pub fn builder() -> V1gnssBuilder {
        V1gnssBuilder::default()
    }
    pub fn update(&mut self, path: &mut Vec<&str>, value: &serde_json::value::Value) {
        match path[0] {
            "type" => {
                let type_result: Result<V1gnssType, serde_json::Error> =
                    serde_json::from_value(value.clone());
                if let Ok(type_value) = type_result {
                    self.type_ = Some(type_value);
                } else {
                    log::error!("Invalid GNSS type: {:?}", type_result);
                    self.type_ = None;
                }
            }
            "methodQuality" => {
                let quality_result: Result<V1gnssMethodQuality, serde_json::Error> =
                    serde_json::from_value(value.clone());
                if let Ok(quality_value) = quality_result {
                    self.method_quality = Some(quality_value);
                } else {
                    log::error!("Invalid GNSS Method Quality: {:?}", quality_result);
                    self.method_quality = None;
                }
            }
            "integrity" => {
                let integrity_result: Result<V1gnssIntegrity, serde_json::Error> =
                    serde_json::from_value(value.clone());
                if let Ok(integrity_value) = integrity_result {
                    self.integrity = Some(integrity_value);
                } else {
                    log::error!("Invalid GNSS Integrity: {:?}", integrity_result);
                    self.integrity = None;
                }
            }
            "satellites" => {
                self.satellites = Some(V1NumberValue::builder().json_value(value).build())
            }
            "satellitesInView" => {
                let satellites_in_view_result: Result<V1gnssSatellitesInView, serde_json::Error> =
                    serde_json::from_value(value.clone());
                if let Ok(satellites_in_view_value) = satellites_in_view_result {
                    self.satellites_in_view = Some(satellites_in_view_value);
                } else {
                    log::error!(
                        "Invalid GNSS Satellites In View: {:?}",
                        satellites_in_view_result
                    );
                    self.satellites_in_view = None;
                }
            }
            "antennaAltitude" => {
                self.antenna_altitude = Some(V1NumberValue::builder().json_value(value).build())
            }
            "horizontalDilution" => {
                self.horizontal_dilution = Some(V1NumberValue::builder().json_value(value).build())
            }
            "positionDilution" => {
                self.position_dilution = Some(V1NumberValue::builder().json_value(value).build())
            }
            "geoidalSeparation" => {
                self.geoidal_separation = Some(V1NumberValue::builder().json_value(value).build())
            }
            "differentialAge" => {
                self.differential_age = Some(V1NumberValue::builder().json_value(value).build())
            }
            &_ => {
                log::warn!("V1gnss: Unknown value to update: {:?}::{:?}", path, value);
            }
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(untagged)]
pub enum V1gnssType {
    Expanded(V1gnssExpandedType),
    Value(V1gnssTypeValue),
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct V1gnssExpandedType {
    value: Option<V1gnssTypeValue>,
    #[serde(flatten)]
    pub common_value_fields: Option<V1CommonValueFields>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub enum V1gnssTypeValue {
    #[default]
    Undefined,
    #[serde(rename = "GPS")]
    Gps,
    #[serde(rename = "GLONASS")]
    Glonass,
    #[serde(rename = "Combined GPS/GLONASS")]
    CombinedGpsGlonass,
    #[serde(rename = "Loran-C")]
    LoranC,
    Chayka,
    #[serde(rename = "Integrated navigation system")]
    IntegratedNavigationSystem,
    Surveyed,
    Galileo,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(untagged)]
pub enum V1gnssMethodQuality {
    Expanded(V1gnssExpandedMethodQuality),
    Value(V1gnssMethodQualityValue),
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct V1gnssExpandedMethodQuality {
    value: V1gnssMethodQualityValue,
    #[serde(flatten)]
    pub common_value_fields: Option<V1CommonValueFields>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub enum V1gnssMethodQualityValue {
    #[default]
    #[serde(rename = "no GPS")]
    NoGps,
    #[serde(rename = "GNSS Fix")]
    GNSSFix,
    #[serde(rename = "DGNSS Fix")]
    DGNSSFix,
    #[serde(rename = "Precise GNSS")]
    PreciseGNSS,
    #[serde(rename = "RTK fixed integer")]
    RTKFixedInteger,
    #[serde(rename = "RTK float")]
    RTKFloat,
    #[serde(rename = "Estimated (DR) mode")]
    EstimatedDRMode,
    #[serde(rename = "Manual input")]
    ManualInput,
    #[serde(rename = "Simulator mode")]
    SimulatorMode,
    #[serde(rename = "Error")]
    Error,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(untagged)]
pub enum V1gnssIntegrity {
    Expanded(V1gnssExpandedIntegrity),
    Value(V1gnssIntegrityValue),
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct V1gnssExpandedIntegrity {
    value: V1gnssIntegrityValue,
    #[serde(flatten)]
    pub common_value_fields: Option<V1CommonValueFields>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub enum V1gnssIntegrityValue {
    #[default]
    #[serde(rename = "no Integrity checking")]
    NoIntegrityChecking,
    Safe,
    Caution,
    Unsafe,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(untagged)]
pub enum V1gnssSatellitesInView {
    Expanded(V1gnssExpandedSatellitesInView),
    Value(V1gnssSatellitesInViewValue),
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct V1gnssExpandedSatellitesInView {
    value: V1gnssSatellitesInViewValue,
    #[serde(flatten)]
    pub common_value_fields: Option<V1CommonValueFields>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct V1gnssSatellitesInViewValue {
    count: i64,
    satellites: Option<Vec<V1gnssSatellite>>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct V1gnssSatellite {
    id: Option<i64>,
    elevation: Option<f64>,
    azimuth: Option<f64>,
    #[serde(rename = "SNR")]
    signal_to_noise_ratio: Option<i64>,
}

#[derive(Default)]
pub struct V1gnssBuilder {}

#[cfg(test)]
mod tests {
    use crate::navigation_gnss::{V1gnssExpandedSatellitesInView, V1gnssSatellite};

    #[test]
    fn satellite_value_object_json() {
        let j = r#"
        {
          "id": 17,
          "elevation": 0.3665,
          "azimuth": 5.5676,
          "SNR": 39
        }"#;
        let satellite: V1gnssSatellite = serde_json::from_str(j).unwrap();
    }
    #[test]
    fn satellites_in_view_value_object_json() {
        let j = r#"
        {
            "meta": {},
            "value": {
              "count": 11,
              "satellites": [
                {
                  "id": 32,
                  "elevation": 1.2043,
                  "azimuth": 3.8921,
                  "SNR": 32
                }
              ]
            },
            "$source": "n2k-sample-data.160",
            "timestamp": "2014-08-15T19:00:51.130Z",
            "pgn": 129540
        }"#;
        let satellites_in_view: V1gnssExpandedSatellitesInView = serde_json::from_str(j).unwrap();
    }
}
