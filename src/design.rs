use crate::definitions::V1StringValue;
use crate::{V1CommonValueFields, V1NumberValue};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct V1Design {
    pub displacement: Option<V1NumberValue>,
    pub ais_ship_type: Option<V1DesignAisShipType>,
    pub draft: Option<V1DesignDraft>,
    pub length: Option<V1DesignLength>,
    pub keel: Option<V1DesignKeel>,
    pub beam: Option<V1NumberValue>,
    pub air_height: Option<V1NumberValue>,
    pub rigging: Option<V1DesignRigging>,
}

impl V1Design {
    pub fn update(&mut self, path: &mut Vec<&str>, value: &serde_json::value::Value) {
        if path.is_empty() {
            return;
        }
        match path[0] {
            "displacement" => self.displacement = V1NumberValue::from_value(value),
            "aisShipType" => self.ais_ship_type = V1DesignAisShipType::from_value(value),
            "draft" => self.draft = V1DesignDraft::from_value(value),
            "length" => self.length = V1DesignLength::from_value(value),
            "keel" => self.keel = V1DesignKeel::from_value(value),
            "beam" => self.beam = V1NumberValue::from_value(value),
            "airHeight" => self.air_height = V1NumberValue::from_value(value),
            "rigging" => self.rigging = V1DesignRigging::from_value(value),
            &_ => {
                log::warn!("V1Design: Unknown update pattern: {:?}::{:?}", path, value);
            }
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct V1DesignAisShipType {
    pub value: Option<V1DesignAisShipTypeValue>,
    #[serde(flatten)]
    pub common_value_fields: Option<V1CommonValueFields>,
}

impl V1DesignAisShipType {
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
pub struct V1DesignAisShipTypeValue {
    pub id: i64,
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct V1DesignDraft {
    pub value: Option<V1DesignDraftValue>,
    #[serde(flatten)]
    pub common_value_fields: Option<V1CommonValueFields>,
}

impl V1DesignDraft {
    pub fn from_value(value: &serde_json::value::Value) -> Option<Self> {
        if value.is_null() {
            None
        } else {
            let draft_result: Result<V1DesignDraft, serde_json::Error> =
                serde_json::from_value(value.clone());
            if let Ok(draft_value) = draft_result {
                Some(draft_value)
            } else {
                None
            }
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct V1DesignDraftValue {
    pub minimum: Option<f64>,
    pub maximum: Option<f64>,
    pub current: Option<f64>,
    pub canoe: Option<f64>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct V1DesignLength {
    pub value: Option<V1DesignLengthValue>,
    #[serde(flatten)]
    pub common_value_fields: Option<V1CommonValueFields>,
}

impl V1DesignLength {
    pub fn from_value(value: &serde_json::value::Value) -> Option<Self> {
        if value.is_null() {
            None
        } else {
            let length_result: Result<Self, serde_json::Error> =
                serde_json::from_value(value.clone());
            if let Ok(length_value) = length_result {
                Some(length_value)
            } else {
                None
            }
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct V1DesignLengthValue {
    pub overall: Option<f64>,
    pub hull: Option<f64>,
    pub waterline: Option<f64>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct V1DesignKeel {
    #[serde(rename = "type")]
    pub type_: Option<V1DesignKeelType>,
    pub angle: Option<V1NumberValue>,
    pub lift: Option<V1NumberValue>,
    #[serde(flatten)]
    pub common_value_fields: Option<V1CommonValueFields>,
}
impl V1DesignKeel {
    pub fn from_value(value: &serde_json::value::Value) -> Option<Self> {
        if value.is_null() {
            None
        } else {
            let keel_result: Result<Self, serde_json::Error> =
                serde_json::from_value(value.clone());
            if let Ok(keel_value) = keel_result {
                Some(keel_value)
            } else {
                None
            }
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
#[serde(rename_all = "lowercase")]
pub enum V1DesignKeelType {
    #[default]
    Long,
    Fin,
    Flare,
    Bulb,
    Wing,
    Centerboard,
    Kanting,
    Lifting,
    Daggerboard,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct V1DesignRigging {
    #[serde(rename = "type")]
    pub type_: Option<V1StringValue>,
    pub masts: Option<V1NumberValue>,
    #[serde(flatten)]
    pub common_value_fields: Option<V1CommonValueFields>,
}

impl V1DesignRigging {
    pub fn from_value(value: &serde_json::value::Value) -> Option<Self> {
        if value.is_null() {
            None
        } else {
            let keel_result: Result<Self, serde_json::Error> =
                serde_json::from_value(value.clone());
            if let Ok(keel_value) = keel_result {
                Some(keel_value)
            } else {
                None
            }
        }
    }
}
