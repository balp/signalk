use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct V1CommonValueFields {
    pub timestamp: String,
    #[serde(rename = "$source")]
    pub source: String,
    #[serde(rename = "_attr")]
    pub attr: Option<V1Attr>,
    pub meta: Option<V1Meta>,
    pub pgn: Option<i64>,
    pub sentence: Option<String>,
}

impl V1CommonValueFields {
    pub fn builder() -> V1CommonValueFieldsBuilder {
        V1CommonValueFieldsBuilder::default()
    }
}

#[derive(Default)]
pub struct V1CommonValueFieldsBuilder {
    timestamp: String,
    source: String,
    attr: Option<V1Attr>,
    meta: Option<V1Meta>,
    pgn: Option<i64>,
    sentence: Option<String>,
}

impl V1CommonValueFieldsBuilder {
    pub fn timestamp(mut self, value: String) -> V1CommonValueFieldsBuilder {
        self.timestamp = value;
        self
    }
    pub fn source(mut self, value: String) -> V1CommonValueFieldsBuilder {
        self.source = value;
        self
    }
    pub fn attr(mut self, value: V1Attr) -> V1CommonValueFieldsBuilder {
        self.attr = Some(value);
        self
    }
    pub fn meta(mut self, value: V1Meta) -> V1CommonValueFieldsBuilder {
        self.meta = Some(value);
        self
    }
    pub fn pgn(mut self, value: i64) -> V1CommonValueFieldsBuilder {
        self.pgn = Some(value);
        self
    }
    pub fn sentence(mut self, value: String) -> V1CommonValueFieldsBuilder {
        self.sentence = Some(value);
        self
    }
    pub fn build(self) -> V1CommonValueFields {
        V1CommonValueFields {
            timestamp: self.timestamp,
            source: self.source,
            attr: self.attr,
            meta: self.meta,
            pgn: self.pgn,
            sentence: self.sentence,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct V1NumberValue {
    pub value: Option<f64>,
    pub timestamp: String,
    #[serde(rename = "$source")]
    pub source: String,
    pub pgn: Option<f64>,
    pub sentence: Option<String>,
    // TODO: Add support for meta values
}

impl V1NumberValue {
    pub fn builder() -> V1NumberValueBuilder {
        V1NumberValueBuilder::default()
    }
}

#[derive(Default)]
pub struct V1NumberValueBuilder {
    pub value: Option<f64>,
    pub timestamp: String,
    pub source: String,
    pub pgn: Option<f64>,
    pub sentence: Option<String>,
}

impl V1NumberValueBuilder {
    pub fn json_value(mut self, value: &serde_json::Value) -> V1NumberValueBuilder {
        if let Some(f64_value) = value.as_f64() {
            self.value = Some(f64_value);
        } else {
            self.value = None;
        }
        self
    }
    pub fn value(mut self, value: f64) -> V1NumberValueBuilder {
        self.value = Some(value);
        self
    }
    pub fn timestamp(mut self, timestamp: String) -> V1NumberValueBuilder {
        self.timestamp = timestamp;
        self
    }
    pub fn source(mut self, source: String) -> V1NumberValueBuilder {
        self.source = source;
        self
    }
    pub fn pgn(mut self, pgn: f64) -> V1NumberValueBuilder {
        self.pgn = Some(pgn);
        self
    }
    pub fn sentence(mut self, sentence: String) -> V1NumberValueBuilder {
        self.sentence = Some(sentence);
        self
    }
    pub fn build(self) -> V1NumberValue {
        V1NumberValue {
            value: self.value,
            timestamp: self.timestamp,
            source: self.source,
            pgn: self.pgn,
            sentence: self.sentence,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct V1Meta {
    pub description: String,
    pub display_name: Option<String>,
    pub long_name: Option<String>,
    pub short_name: Option<String>,
    #[serde(rename = "enum")]
    pub enum_: Option<Vec<String>>,
    pub properties: Option<V1MetaProperties>,
    pub gauge_type: Option<String>,
    pub display_scale: Option<V1MetaDisplayScale>,
    pub units: Option<String>,
    pub timeout: Option<f64>,
    pub alert_method: Option<Vec<String>>,
    pub warn_method: Option<Vec<String>>,
    pub alarm_method: Option<Vec<String>>,
    pub emergency_method: Option<Vec<String>>,
    pub zones: Option<Vec<V1MetaZone>>,
}

impl V1Meta {
    pub fn builder() -> V1MetaBuilder {
        V1MetaBuilder::default()
    }
}

#[derive(Default)]
pub struct V1MetaBuilder {
    description: String,
    display_name: Option<String>,
    long_name: Option<String>,
    short_name: Option<String>,
    enum_: Option<Vec<String>>,
    properties: Option<V1MetaProperties>,
    gauge_type: Option<String>,
    display_scale: Option<V1MetaDisplayScale>,
    units: Option<String>,
    timeout: Option<f64>,
    alert_method: Option<Vec<String>>,
    warn_method: Option<Vec<String>>,
    alarm_method: Option<Vec<String>>,
    emergency_method: Option<Vec<String>>,
    zones: Option<Vec<V1MetaZone>>,
}

impl V1MetaBuilder {
    pub fn description(mut self, value: String) -> V1MetaBuilder {
        self.description = value;
        self
    }
    pub fn display_name(mut self, value: String) -> V1MetaBuilder {
        self.display_name = Some(value);
        self
    }
    pub fn long_name(mut self, value: String) -> V1MetaBuilder {
        self.long_name = Some(value);
        self
    }
    pub fn short_name(mut self, value: String) -> V1MetaBuilder {
        self.short_name = Some(value);
        self
    }
    pub fn enum_(mut self, value: String) -> V1MetaBuilder {
        if self.enum_.is_none() {
            self.enum_ = Some(Vec::new());
        }
        if let Some(ref mut x) = self.enum_ {
            x.push(value);
        }
        self
    }
    pub fn properties(mut self, value: V1MetaProperties) -> V1MetaBuilder {
        self.properties = Some(value);
        self
    }
    pub fn gauge_type(mut self, value: String) -> V1MetaBuilder {
        self.gauge_type = Some(value);
        self
    }
    pub fn display_scale(mut self, value: V1MetaDisplayScale) -> V1MetaBuilder {
        self.display_scale = Some(value);
        self
    }
    pub fn units(mut self, value: String) -> V1MetaBuilder {
        self.units = Some(value);
        self
    }
    pub fn timeout(mut self, value: f64) -> V1MetaBuilder {
        self.timeout = Some(value);
        self
    }
    pub fn alert_method(mut self, value: String) -> V1MetaBuilder {
        if self.alert_method.is_none() {
            self.alert_method = Some(Vec::new());
        }
        if let Some(ref mut x) = self.alert_method {
            x.push(value);
        }
        self
    }
    pub fn warn_method(mut self, value: String) -> V1MetaBuilder {
        if self.warn_method.is_none() {
            self.warn_method = Some(Vec::new());
        }
        if let Some(ref mut x) = self.warn_method {
            x.push(value);
        }
        self
    }
    pub fn alarm_method(mut self, value: String) -> V1MetaBuilder {
        if self.alarm_method.is_none() {
            self.alarm_method = Some(Vec::new());
        }
        if let Some(ref mut x) = self.alarm_method {
            x.push(value);
        }
        self
    }
    pub fn emergency_method(mut self, value: String) -> V1MetaBuilder {
        if self.emergency_method.is_none() {
            self.emergency_method = Some(Vec::new());
        }
        if let Some(ref mut x) = self.emergency_method {
            x.push(value);
        }
        self
    }
    pub fn zones(mut self, value: V1MetaZone) -> V1MetaBuilder {
        if self.zones.is_none() {
            self.zones = Some(Vec::new());
        }
        if let Some(ref mut x) = self.zones {
            x.push(value);
        }
        self
    }
    pub fn build(self) -> V1Meta {
        V1Meta {
            description: self.description,
            display_name: self.display_name,
            long_name: self.long_name,
            short_name: self.short_name,
            enum_: self.enum_,
            properties: self.properties,
            gauge_type: self.gauge_type,
            display_scale: self.display_scale,
            units: self.units,
            timeout: self.timeout,
            alert_method: self.alert_method,
            warn_method: self.warn_method,
            alarm_method: self.alarm_method,
            emergency_method: self.emergency_method,
            zones: self.zones,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct V1MetaProperties {
    pub properties: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub units: Option<String>,
    pub example: Option<Value>,
}

impl V1MetaProperties {
    pub fn builder() -> V1MetaPropertiesBuilder {
        V1MetaPropertiesBuilder::default()
    }
}

#[derive(Default)]
pub struct V1MetaPropertiesBuilder {
    properties: Option<String>,
    title: Option<String>,
    description: Option<String>,
    units: Option<String>,
    example: Option<Value>,
}

impl V1MetaPropertiesBuilder {
    pub fn properties(mut self, value: String) -> V1MetaPropertiesBuilder {
        self.properties = Some(value);
        self
    }
    pub fn title(mut self, value: String) -> V1MetaPropertiesBuilder {
        self.title = Some(value);
        self
    }
    pub fn description(mut self, value: String) -> V1MetaPropertiesBuilder {
        self.description = Some(value);
        self
    }
    pub fn units(mut self, value: String) -> V1MetaPropertiesBuilder {
        self.units = Some(value);
        self
    }
    pub fn example(mut self, value: Value) -> V1MetaPropertiesBuilder {
        self.example = Some(value);
        self
    }

    pub fn build(self) -> V1MetaProperties {
        V1MetaProperties {
            properties: self.properties,
            title: self.title,
            description: self.description,
            units: self.units,
            example: self.example,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct V1MetaDisplayScale {
    pub lower: f64,
    pub upper: f64,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub power: Option<f64>,
}

impl V1MetaDisplayScale {
    pub fn builder() -> V1MetaDisplayScaleBuilder {
        V1MetaDisplayScaleBuilder::default()
    }
}

#[derive(Default)]
pub struct V1MetaDisplayScaleBuilder {
    lower: f64,
    upper: f64,
    type_: Option<String>,
    power: Option<f64>,
}

impl V1MetaDisplayScaleBuilder {
    pub fn lower(mut self, value: f64) -> V1MetaDisplayScaleBuilder {
        self.lower = value;
        self
    }
    pub fn upper(mut self, value: f64) -> V1MetaDisplayScaleBuilder {
        self.upper = value;
        self
    }
    pub fn type_(mut self, value: String) -> V1MetaDisplayScaleBuilder {
        self.type_ = Some(value);
        self
    }
    pub fn power(mut self, value: f64) -> V1MetaDisplayScaleBuilder {
        self.power = Some(value);
        self
    }

    pub fn build(self) -> V1MetaDisplayScale {
        V1MetaDisplayScale {
            lower: self.lower,
            upper: self.upper,
            type_: self.type_,
            power: self.power,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct V1MetaZone {
    pub lower: Option<f64>,
    pub upper: Option<f64>,
    pub state: String,
    pub message: Option<String>,
}

impl V1MetaZone {
    pub fn builder() -> V1MetaZoneBuilder {
        V1MetaZoneBuilder::default()
    }
}

#[derive(Default)]
pub struct V1MetaZoneBuilder {
    lower: Option<f64>,
    upper: Option<f64>,
    state: String,
    message: Option<String>,
}

impl V1MetaZoneBuilder {
    pub fn lower(mut self, value: f64) -> V1MetaZoneBuilder {
        self.lower = Some(value);
        self
    }
    pub fn upper(mut self, value: f64) -> V1MetaZoneBuilder {
        self.upper = Some(value);
        self
    }
    pub fn state(mut self, value: String) -> V1MetaZoneBuilder {
        self.state = value;
        self
    }
    pub fn message(mut self, value: String) -> V1MetaZoneBuilder {
        self.message = Some(value);
        self
    }

    pub fn build(self) -> V1MetaZone {
        V1MetaZone {
            lower: self.lower,
            upper: self.upper,
            state: self.state,
            message: self.message,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct V1Attr {
    #[serde(rename = "_mode")]
    pub mode: Option<i64>,
    #[serde(rename = "_owner")]
    pub owner: Option<String>,
    #[serde(rename = "_group")]
    pub group: Option<String>,
}

impl Default for V1Attr {
    fn default() -> Self {
        V1Attr {
            mode: Some(644),
            owner: Some("self".into()),
            group: Some("self".into()),
        }
    }
}

impl V1Attr {
    pub fn new(mode: i64, owner: String, group: String) -> V1Attr {
        V1Attr {
            mode: Some(mode),
            owner: Some(owner),
            group: Some(group),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct V1DefSource {
    pub label: String,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub src: Option<String>,
    pub can_name: Option<String>,
    pub pgn: Option<i32>,
    pub instance: Option<String>,
    pub sentence: Option<String>,
    pub talker: Option<String>,
    pub ais_type: Option<i32>,
}

impl V1DefSource {
    pub fn builder() -> V1DefSourceBuilder {
        V1DefSourceBuilder::default()
    }
}

#[derive(Default)]
pub struct V1DefSourceBuilder {
    label: String,
    type_: Option<String>,
    src: Option<String>,
    can_name: Option<String>,
    pgn: Option<i32>,
    instance: Option<String>,
    sentence: Option<String>,
    talker: Option<String>,
    ais_type: Option<i32>,
}

impl V1DefSourceBuilder {
    pub fn label(mut self, value: String) -> V1DefSourceBuilder {
        self.label = value;
        self
    }
    pub fn type_(mut self, value: String) -> V1DefSourceBuilder {
        self.type_ = Some(value);
        self
    }
    pub fn src(mut self, value: String) -> V1DefSourceBuilder {
        self.src = Some(value);
        self
    }
    pub fn can_name(mut self, value: String) -> V1DefSourceBuilder {
        self.can_name = Some(value);
        self
    }
    pub fn pgn(mut self, value: i32) -> V1DefSourceBuilder {
        self.pgn = Some(value);
        self
    }
    pub fn instance(mut self, value: String) -> V1DefSourceBuilder {
        self.instance = Some(value);
        self
    }
    pub fn sentence(mut self, value: String) -> V1DefSourceBuilder {
        self.sentence = Some(value);
        self
    }
    pub fn talker(mut self, value: String) -> V1DefSourceBuilder {
        self.talker = Some(value);
        self
    }
    pub fn ais_type(mut self, value: i32) -> V1DefSourceBuilder {
        self.ais_type = Some(value);
        self
    }
    pub fn build(self) -> V1DefSource {
        V1DefSource {
            label: self.label,
            type_: self.type_,
            src: self.src,
            can_name: self.can_name,
            pgn: self.pgn,
            instance: self.instance,
            sentence: self.sentence,
            talker: self.talker,
            ais_type: self.ais_type,
        }
    }
}
