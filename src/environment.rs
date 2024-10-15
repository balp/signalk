use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::definitions::{V1CommonValueFields, V1NumberValue};
use crate::helper_functions::get_f64_value;
use crate::sources::V1Source;
use crate::SignalKGetError;

trait F64Gettable {
    fn get_f64_for_path(&self, path: &mut Vec<&str>) -> Result<f64, SignalKGetError>;
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct V1Environment {
    pub outside: Option<V1EnvironmentOutside>,
    pub inside: Option<V1EnvironmentInside>,
    pub water: Option<V1EnvironmentWater>,
    pub depth: Option<V1EnvironmentDepth>,
    pub current: Option<V1EnvironmentCurrent>,
    pub tide: Option<V1EnvironmentTide>,
    pub heave: Option<V1NumberValue>,
    pub wind: Option<V1EnvironmentWind>,
    pub time: Option<V1EnvironmentTime>,
    pub mode: Option<V1EnvironmentMode>,
}

impl V1Environment {
    pub fn builder() -> V1EnvironmentBuilder {
        V1EnvironmentBuilder::default()
    }

    pub fn update(&mut self, path: &mut Vec<&str>, value: &serde_json::value::Value) {
        log::debug!("environment update: {:?} -> {:?}", path, value);
        match path[0] {
            "outside" => {
                if self.outside.is_none() {
                    self.outside = Some(V1EnvironmentOutside::default());
                }
                if let Some(ref mut outside) = self.outside {
                    path.remove(0);
                    outside.update(path, value);
                }
            }
            "inside" => {
                if self.inside.is_none() {
                    self.inside = Some(V1EnvironmentInside::default());
                }
                if let Some(ref mut inside) = self.inside {
                    path.remove(0);
                    inside.update(path, value);
                }
            }
            "water" => {
                if self.water.is_none() {
                    self.water = Some(V1EnvironmentWater::new(None, None));
                }
                if let Some(ref mut water) = self.water {
                    path.remove(0);
                    water.update(path, value);
                }
            }
            "depth" => {
                if self.depth.is_none() {
                    self.depth = Some(V1EnvironmentDepth::default());
                }
                if let Some(ref mut depth) = self.depth {
                    path.remove(0);
                    depth.update(path, value);
                }
            }
            "current" => {
                if self.current.is_none() {
                    self.current = Some(V1EnvironmentCurrent::default());
                }
                if let Some(ref mut current) = self.current {
                    path.remove(0);
                    current.update(path, value);
                }
            }
            "tide" => {
                if self.tide.is_none() {
                    self.tide = Some(V1EnvironmentTide::default());
                }
                if let Some(ref mut tide) = self.tide {
                    path.remove(0);
                    tide.update(path, value);
                }
            }
            "heave" => self.heave = Some(V1NumberValue::builder().json_value(value).build()),
            "wind" => {
                if self.wind.is_none() {
                    self.wind = Some(V1EnvironmentWind::default());
                }
                if let Some(ref mut wind) = self.wind {
                    path.remove(0);
                    wind.update(path, value);
                }
            }
            "time" => {
                if self.time.is_none() {
                    self.time = Some(V1EnvironmentTime::default());
                }
                if let Some(ref mut time) = self.time {
                    path.remove(0);
                    time.update(path, value);
                }
            }
            "mode" => {
                if self.mode.is_none() {
                    self.mode = Some(V1EnvironmentMode::default());
                }
                if let Some(ref mut mode) = self.mode {
                    path.remove(0);
                    mode.update(path, value);
                }
            }

            &_ => {
                log::warn!(
                    "V1Environment: Unknown value to update: {:?}::{:?}",
                    path,
                    value
                );
            }
        }
    }

    pub fn get_f64_for_path(&self, path: &mut Vec<&str>) -> Result<f64, SignalKGetError> {
        match path[0] {
            "outside" => {
                if let Some(ref value) = self.outside {
                    path.remove(0);
                    value.get_f64_for_path(path)
                } else {
                    Err(SignalKGetError::NoSuchPath)
                }
            }
            "inside" => {
                if let Some(ref value) = self.inside {
                    path.remove(0);
                    value.get_f64_for_path(path)
                } else {
                    Err(SignalKGetError::NoSuchPath)
                }
            }
            "water" => {
                if let Some(ref value) = self.water {
                    path.remove(0);
                    value.get_f64_for_path(path)
                } else {
                    Err(SignalKGetError::NoSuchPath)
                }
            }
            "depth" => {
                if let Some(ref value) = self.depth {
                    path.remove(0);
                    value.get_f64_for_path(path)
                } else {
                    Err(SignalKGetError::NoSuchPath)
                }
            }
            "current" => {
                if let Some(ref value) = self.current {
                    path.remove(0);
                    value.get_f64_for_path(path)
                } else {
                    Err(SignalKGetError::NoSuchPath)
                }
            }
            "tide" => {
                if let Some(ref value) = self.tide {
                    path.remove(0);
                    value.get_f64_for_path(path)
                } else {
                    Err(SignalKGetError::NoSuchPath)
                }
            }
            "heave" => get_f64_value(&self.heave),
            "wind" => {
                if let Some(ref value) = self.wind {
                    path.remove(0);
                    value.get_f64_for_path(path)
                } else {
                    Err(SignalKGetError::NoSuchPath)
                }
            }
            "time" => {
                if let Some(ref value) = self.time {
                    path.remove(0);
                    value.get_f64_for_path(path)
                } else {
                    Err(SignalKGetError::NoSuchPath)
                }
            }
            "mode" => {
                if let Some(ref value) = self.mode {
                    path.remove(0);
                    value.get_f64_for_path(path)
                } else {
                    Err(SignalKGetError::NoSuchPath)
                }
            }
            &_ => Err(SignalKGetError::NoSuchPath),
        }
    }
}

#[derive(Default)]
pub struct V1EnvironmentBuilder {
    outside: Option<V1EnvironmentOutside>,
    inside: Option<V1EnvironmentInside>,
    water: Option<V1EnvironmentWater>,
    depth: Option<V1EnvironmentDepth>,
    current: Option<V1EnvironmentCurrent>,
    tide: Option<V1EnvironmentTide>,
    heave: Option<V1NumberValue>,
    wind: Option<V1EnvironmentWind>,
    time: Option<V1EnvironmentTime>,
    mode: Option<V1EnvironmentMode>,
}

impl V1EnvironmentBuilder {
    pub fn outside(mut self, label: V1EnvironmentOutside) -> V1EnvironmentBuilder {
        self.outside = Some(label);
        self
    }
    pub fn inside(mut self, label: V1EnvironmentInside) -> V1EnvironmentBuilder {
        self.inside = Some(label);
        self
    }
    pub fn water(mut self, label: V1EnvironmentWater) -> V1EnvironmentBuilder {
        self.water = Some(label);
        self
    }
    pub fn depth(mut self, label: V1EnvironmentDepth) -> V1EnvironmentBuilder {
        self.depth = Some(label);
        self
    }
    pub fn current(mut self, label: V1EnvironmentCurrent) -> V1EnvironmentBuilder {
        self.current = Some(label);
        self
    }
    pub fn tide(mut self, label: V1EnvironmentTide) -> V1EnvironmentBuilder {
        self.tide = Some(label);
        self
    }
    pub fn heave(mut self, label: V1NumberValue) -> V1EnvironmentBuilder {
        self.heave = Some(label);
        self
    }
    pub fn wind(mut self, label: V1EnvironmentWind) -> V1EnvironmentBuilder {
        self.wind = Some(label);
        self
    }
    pub fn time(mut self, label: V1EnvironmentTime) -> V1EnvironmentBuilder {
        self.time = Some(label);
        self
    }
    pub fn mode(mut self, label: V1EnvironmentMode) -> V1EnvironmentBuilder {
        self.mode = Some(label);
        self
    }
    pub fn build(self) -> V1Environment {
        V1Environment {
            outside: self.outside,
            inside: self.inside,
            water: self.water,
            depth: self.depth,
            current: self.current,
            tide: self.tide,
            heave: self.heave,
            wind: self.wind,
            time: self.time,
            mode: self.mode,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct V1EnvironmentOutside {
    pub temperature: Option<V1NumberValue>,
    pub dew_point_temperature: Option<V1NumberValue>,
    pub apparent_wind_chill_temperature: Option<V1NumberValue>,
    pub theoretical_wind_chill_temperature: Option<V1NumberValue>,
    pub heat_index_temperature: Option<V1NumberValue>,
    pub pressure: Option<V1NumberValue>,
    pub humidity: Option<V1NumberValue>,
    pub relative_humidity: Option<V1NumberValue>,
    pub air_density: Option<V1NumberValue>,
    pub illuminance: Option<V1NumberValue>,
}

impl F64Gettable for V1EnvironmentOutside {
    fn get_f64_for_path(&self, path: &mut Vec<&str>) -> Result<f64, SignalKGetError> {
        match path[0] {
            "temperature" => get_f64_value(&self.temperature),
            "dew_point_temperature" => get_f64_value(&self.dew_point_temperature),
            "apparent_wind_chill_temperature" => {
                get_f64_value(&self.apparent_wind_chill_temperature)
            }
            "theoretical_wind_chill_temperature" => {
                get_f64_value(&self.theoretical_wind_chill_temperature)
            }
            "heat_index_temperature" => get_f64_value(&self.heat_index_temperature),
            "pressure" => get_f64_value(&self.pressure),
            "humidity" => get_f64_value(&self.humidity),
            "relative_humidity" => get_f64_value(&self.relative_humidity),
            "air_density" => get_f64_value(&self.pressure),
            "illuminance" => get_f64_value(&self.illuminance),
            &_ => Err(SignalKGetError::NoSuchPath),
        }
    }
}

impl V1EnvironmentOutside {
    pub fn builder() -> V1EnvironmentOutsideBuilder {
        V1EnvironmentOutsideBuilder::default()
    }
    pub fn update(&mut self, path: &mut Vec<&str>, value: &serde_json::value::Value) {
        log::debug!("V1EnvironmentOutside update: {:?} -> {:?}", path, value);
        match path[0] {
            &_ => {
                log::warn!(
                    "V1EnvironmentOutside: Unknown value to update: {:?}::{:?}",
                    path,
                    value
                );
            }
        };
    }
}

#[derive(Default)]
pub struct V1EnvironmentOutsideBuilder {
    temperature: Option<V1NumberValue>,
    dew_point_temperature: Option<V1NumberValue>,
    apparent_wind_chill_temperature: Option<V1NumberValue>,
    theoretical_wind_chill_temperature: Option<V1NumberValue>,
    heat_index_temperature: Option<V1NumberValue>,
    pressure: Option<V1NumberValue>,
    humidity: Option<V1NumberValue>,
    relative_humidity: Option<V1NumberValue>,
    air_density: Option<V1NumberValue>,
    illuminance: Option<V1NumberValue>,
}

impl V1EnvironmentOutsideBuilder {
    pub fn temperature(mut self, value: V1NumberValue) -> V1EnvironmentOutsideBuilder {
        self.temperature = Some(value);
        self
    }
    pub fn dew_point_temperature(mut self, value: V1NumberValue) -> V1EnvironmentOutsideBuilder {
        self.dew_point_temperature = Some(value);
        self
    }
    pub fn apparent_wind_chill_temperature(
        mut self,
        value: V1NumberValue,
    ) -> V1EnvironmentOutsideBuilder {
        self.apparent_wind_chill_temperature = Some(value);
        self
    }
    pub fn theoretical_wind_chill_temperature(
        mut self,
        value: V1NumberValue,
    ) -> V1EnvironmentOutsideBuilder {
        self.theoretical_wind_chill_temperature = Some(value);
        self
    }
    pub fn heat_index_temperature(mut self, value: V1NumberValue) -> V1EnvironmentOutsideBuilder {
        self.heat_index_temperature = Some(value);
        self
    }
    pub fn pressure(mut self, value: V1NumberValue) -> V1EnvironmentOutsideBuilder {
        self.pressure = Some(value);
        self
    }
    pub fn humidity(mut self, value: V1NumberValue) -> V1EnvironmentOutsideBuilder {
        self.humidity = Some(value);
        self
    }
    pub fn relative_humidity(mut self, value: V1NumberValue) -> V1EnvironmentOutsideBuilder {
        self.relative_humidity = Some(value);
        self
    }
    pub fn air_density(mut self, value: V1NumberValue) -> V1EnvironmentOutsideBuilder {
        self.air_density = Some(value);
        self
    }
    pub fn illuminance(mut self, value: V1NumberValue) -> V1EnvironmentOutsideBuilder {
        self.illuminance = Some(value);
        self
    }
    pub fn build(self) -> V1EnvironmentOutside {
        V1EnvironmentOutside {
            temperature: self.temperature,
            dew_point_temperature: self.dew_point_temperature,
            apparent_wind_chill_temperature: self.apparent_wind_chill_temperature,
            theoretical_wind_chill_temperature: self.theoretical_wind_chill_temperature,
            heat_index_temperature: self.heat_index_temperature,
            pressure: self.pressure,
            humidity: self.humidity,
            relative_humidity: self.relative_humidity,
            air_density: self.air_density,
            illuminance: self.illuminance,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct V1EnvironmentInside {
    #[serde(flatten)]
    pub zones: HashMap<String, V1EnvironmentZone>,
}

impl V1EnvironmentInside {
    pub fn builder() -> V1EnvironmentInsideBuilder {
        V1EnvironmentInsideBuilder::default()
    }
    pub fn update(&mut self, path: &mut Vec<&str>, value: &serde_json::value::Value) {
        log::debug!("V1EnvironmentOutside update: {:?} -> {:?}", path, value);
        log::warn!(
            "V1EnvironmentInside: Unknown value to update: {:?}::{:?}",
            path,
            value
        );
    }
}

impl F64Gettable for V1EnvironmentInside {
    fn get_f64_for_path(&self, path: &mut Vec<&str>) -> Result<f64, SignalKGetError> {
        match path[0] {
            &_ => Err(SignalKGetError::NoSuchPath),
        }
    }
}

#[derive(Default)]
pub struct V1EnvironmentInsideBuilder {
    pub zones: HashMap<String, V1EnvironmentZone>,
}

impl V1EnvironmentInsideBuilder {
    pub fn add_zone(mut self, key: String, value: V1EnvironmentZone) -> V1EnvironmentInsideBuilder {
        self.zones.insert(key, value);
        self
    }
    pub fn build(self) -> V1EnvironmentInside {
        V1EnvironmentInside { zones: self.zones }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct V1EnvironmentZone {
    pub temperature: Option<V1NumberValue>,
    pub heat_index_temperature: Option<V1NumberValue>,
    pub pressure: Option<V1NumberValue>,
    pub relative_humidity: Option<V1NumberValue>,
    pub dew_point: Option<V1NumberValue>,
    pub dew_point_temperature: Option<V1NumberValue>,
    pub air_density: Option<V1NumberValue>,
    pub illuminance: Option<V1NumberValue>,
}

impl V1EnvironmentZone {
    pub fn builder() -> V1EnvironmentZoneBuilder {
        V1EnvironmentZoneBuilder::default()
    }
    pub fn update(&mut self, path: &mut Vec<&str>, value: &serde_json::value::Value) {
        log::debug!("V1EnvironmentZone update: {:?} -> {:?}", path, value);
        log::warn!(
            "V1EnvironmentZone: Unknown value to update: {:?}::{:?}",
            path,
            value
        );
    }
}
impl F64Gettable for V1EnvironmentZone {
    fn get_f64_for_path(&self, path: &mut Vec<&str>) -> Result<f64, SignalKGetError> {
        match path[0] {
            "temperature" => get_f64_value(&self.temperature),
            "heatIndexTemperature" => get_f64_value(&self.heat_index_temperature),
            "pressure" => get_f64_value(&self.pressure),
            "relativeHumidity" => get_f64_value(&self.relative_humidity),
            "dewPoint" => get_f64_value(&self.dew_point),
            "dewPointTemperature" => get_f64_value(&self.dew_point_temperature),
            "airDensity" => get_f64_value(&self.air_density),
            "illuminance" => get_f64_value(&self.illuminance),
            &_ => Err(SignalKGetError::NoSuchPath),
        }
    }
}

#[derive(Default)]
pub struct V1EnvironmentZoneBuilder {
    temperature: Option<V1NumberValue>,
    heat_index_temperature: Option<V1NumberValue>,
    pressure: Option<V1NumberValue>,
    relative_humidity: Option<V1NumberValue>,
    dew_point: Option<V1NumberValue>,
    dew_point_temperature: Option<V1NumberValue>,
    air_density: Option<V1NumberValue>,
    illuminance: Option<V1NumberValue>,
}

impl V1EnvironmentZoneBuilder {
    pub fn temperature(mut self, value: V1NumberValue) -> V1EnvironmentZoneBuilder {
        self.temperature = Some(value);
        self
    }
    pub fn heat_index_temperature(mut self, value: V1NumberValue) -> V1EnvironmentZoneBuilder {
        self.heat_index_temperature = Some(value);
        self
    }
    pub fn pressure(mut self, value: V1NumberValue) -> V1EnvironmentZoneBuilder {
        self.pressure = Some(value);
        self
    }
    pub fn relative_humidity(mut self, value: V1NumberValue) -> V1EnvironmentZoneBuilder {
        self.relative_humidity = Some(value);
        self
    }
    pub fn dew_point(mut self, value: V1NumberValue) -> V1EnvironmentZoneBuilder {
        self.dew_point = Some(value);
        self
    }
    pub fn dew_point_temperature(mut self, value: V1NumberValue) -> V1EnvironmentZoneBuilder {
        self.dew_point_temperature = Some(value);
        self
    }
    pub fn air_density(mut self, value: V1NumberValue) -> V1EnvironmentZoneBuilder {
        self.air_density = Some(value);
        self
    }
    pub fn illuminance(mut self, value: V1NumberValue) -> V1EnvironmentZoneBuilder {
        self.illuminance = Some(value);
        self
    }
    pub fn build(self) -> V1EnvironmentZone {
        V1EnvironmentZone {
            temperature: self.temperature,
            heat_index_temperature: self.heat_index_temperature,
            pressure: self.pressure,
            relative_humidity: self.relative_humidity,
            dew_point: self.dew_point,
            dew_point_temperature: self.dew_point_temperature,
            air_density: self.air_density,
            illuminance: self.illuminance,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct V1EnvironmentWater {
    pub temperature: Option<V1NumberValue>,
    pub salinity: Option<V1NumberValue>,
}

impl V1EnvironmentWater {
    pub fn new(temperature: Option<V1NumberValue>, salinity: Option<V1NumberValue>) -> Self {
        Self {
            temperature,
            salinity,
        }
    }
    pub fn update(&mut self, path: &mut Vec<&str>, value: &serde_json::value::Value) {
        log::debug!("V1EnvironmentWater update: {:?} -> {:?}", path, value);
        match path[0] {
            "temperature" => {
                if let Some(val) = value.as_f64() {
                    self.temperature = Some(V1NumberValue::builder().value(val).build())
                }
            }
            "salinity" => {
                if let Some(val) = value.as_f64() {
                    self.salinity = Some(V1NumberValue::builder().value(val).build())
                }
            }
            &_ => {
                log::warn!(
                    "V1EnvironmentWater: Unknown value to update: {:?}::{:?}",
                    path,
                    value
                );
            }
        };
    }
}

impl F64Gettable for V1EnvironmentWater {
    fn get_f64_for_path(&self, path: &mut Vec<&str>) -> Result<f64, SignalKGetError> {
        match path[0] {
            "temperature" => get_f64_value(&self.temperature),
            "salinity" => get_f64_value(&self.salinity),
            &_ => Err(SignalKGetError::NoSuchPath),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct V1EnvironmentDepth {
    pub below_keel: Option<V1NumberValue>,
    pub below_transducer: Option<V1NumberValue>,
    pub below_surface: Option<V1NumberValue>,
    pub transducer_to_keel: Option<V1NumberValue>,
    pub surface_to_transducer: Option<V1NumberValue>,
}

impl V1EnvironmentDepth {
    pub fn builder() -> V1EnvironmentDepthBuilder {
        V1EnvironmentDepthBuilder::default()
    }
    pub fn update(&mut self, path: &mut Vec<&str>, value: &serde_json::value::Value) {
        log::debug!("V1EnvironmentDepth update: {:?} -> {:?}", path, value);
        match path[0] {
            "belowKeel" => {
                if let Some(val) = value.as_f64() {
                    self.below_keel = Some(V1NumberValue::builder().value(val).build())
                }
            }
            "belowTransducer" => {
                if let Some(val) = value.as_f64() {
                    self.below_transducer = Some(V1NumberValue::builder().value(val).build())
                }
            }
            "belowSurface" => {
                if let Some(val) = value.as_f64() {
                    self.below_surface = Some(V1NumberValue::builder().value(val).build())
                }
            }
            "transducerToKeel" => {
                if let Some(val) = value.as_f64() {
                    self.transducer_to_keel = Some(V1NumberValue::builder().value(val).build())
                }
            }
            "surfaceToTransducer" => {
                if let Some(val) = value.as_f64() {
                    self.surface_to_transducer = Some(V1NumberValue::builder().value(val).build())
                }
            }
            &_ => {
                log::warn!(
                    "V1EnvironmentDepth: Unknown value to update: {:?}::{:?}",
                    path,
                    value
                );
            }
        };
    }
}

impl F64Gettable for V1EnvironmentDepth {
    fn get_f64_for_path(&self, path: &mut Vec<&str>) -> Result<f64, SignalKGetError> {
        match path[0] {
            "belowKeel" => get_f64_value(&self.below_keel),
            "belowTransducer" => get_f64_value(&self.below_transducer),
            "belowSurface" => get_f64_value(&self.below_surface),
            "transducerToKeel" => get_f64_value(&self.transducer_to_keel),
            "surfaceToTransducer" => get_f64_value(&self.surface_to_transducer),
            &_ => Err(SignalKGetError::NoSuchPath),
        }
    }
}

#[derive(Default)]
pub struct V1EnvironmentDepthBuilder {
    below_keel: Option<V1NumberValue>,
    below_transducer: Option<V1NumberValue>,
    below_surface: Option<V1NumberValue>,
    transducer_to_keel: Option<V1NumberValue>,
    surface_to_transducer: Option<V1NumberValue>,
}

impl V1EnvironmentDepthBuilder {
    pub fn below_keel(mut self, value: V1NumberValue) -> V1EnvironmentDepthBuilder {
        self.below_keel = Some(value);
        self
    }
    pub fn below_transducer(mut self, value: V1NumberValue) -> V1EnvironmentDepthBuilder {
        self.below_transducer = Some(value);
        self
    }
    pub fn below_surface(mut self, value: V1NumberValue) -> V1EnvironmentDepthBuilder {
        self.below_surface = Some(value);
        self
    }
    pub fn transducer_to_keel(mut self, value: V1NumberValue) -> V1EnvironmentDepthBuilder {
        self.transducer_to_keel = Some(value);
        self
    }
    pub fn surface_to_transducer(mut self, value: V1NumberValue) -> V1EnvironmentDepthBuilder {
        self.surface_to_transducer = Some(value);
        self
    }
    pub fn build(self) -> V1EnvironmentDepth {
        V1EnvironmentDepth {
            below_keel: self.below_keel,
            below_transducer: self.below_transducer,
            below_surface: self.below_surface,
            transducer_to_keel: self.transducer_to_keel,
            surface_to_transducer: self.surface_to_transducer,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct V1EnvironmentCurrent {
    #[serde(flatten)]
    pub common: Option<V1CommonValueFields>,
    pub value: Option<V1EnvironmentCurrentValue>,
    pub values: Option<HashMap<String, V1EnvironmentCurrentType>>,
}

impl V1EnvironmentCurrent {
    pub fn update(&mut self, path: &mut Vec<&str>, value: &serde_json::value::Value) {
        log::debug!("V1EnvironmentCurrent update: {:?} -> {:?}", path, value);
        let val: Result<V1EnvironmentCurrentValue, serde_json::Error> =
            serde_json::from_value(value.clone());
        if let Ok(cur) = val {
            log::debug!("V1EnvironmentCurrent value: {:?}", cur);
            self.value = Some(cur);
        } else {
            log::warn!(
                "V1EnvironmentCurrent: Unknown value to update: {:?}::{:?}",
                path,
                value
            );
        }
    }
}

impl F64Gettable for V1EnvironmentCurrent {
    fn get_f64_for_path(&self, _path: &mut Vec<&str>) -> Result<f64, SignalKGetError> {
        Err(SignalKGetError::TBD)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct V1EnvironmentCurrentType {
    pub timestamp: Option<String>,
    pub pgn: Option<f64>,
    pub sentence: Option<String>,
    pub value: Option<V1EnvironmentCurrentValue>,
}

impl V1EnvironmentCurrentType {
    pub fn update(&mut self, path: &mut Vec<&str>, value: &serde_json::value::Value) {
        log::debug!("V1EnvironmentCurrentType update: {:?} -> {:?}", path, value);
        log::warn!(
            "V1EnvironmentCurrentType: Unknown value to update: {:?}::{:?}",
            path,
            value
        );
    }
}

impl F64Gettable for V1EnvironmentCurrentType {
    fn get_f64_for_path(&self, _path: &mut Vec<&str>) -> Result<f64, SignalKGetError> {
        Err(SignalKGetError::TBD)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct V1EnvironmentCurrentValue {
    pub drift: Option<f64>,
    pub set_true: Option<f64>,
    pub set_magnetic: Option<f64>,
}

impl V1EnvironmentCurrentValue {
    pub fn update(&mut self, path: &mut Vec<&str>, value: &serde_json::value::Value) {
        log::debug!(
            "V1EnvironmentCurrentValue update: {:?} -> {:?}",
            path,
            value
        );
        log::warn!(
            "V1EnvironmentCurrentValue: Unknown value to update: {:?}::{:?}",
            path,
            value
        );
    }
}

impl F64Gettable for V1EnvironmentCurrentValue {
    fn get_f64_for_path(&self, _path: &mut Vec<&str>) -> Result<f64, SignalKGetError> {
        Err(SignalKGetError::TBD)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct V1EnvironmentTide {
    pub height_high: Option<V1NumberValue>,
    pub height_now: Option<V1NumberValue>,
    pub height_low: Option<V1NumberValue>,
    pub time_low: Option<String>,
    pub time_high: Option<String>,
}

impl V1EnvironmentTide {
    pub fn update(&mut self, path: &mut Vec<&str>, value: &serde_json::value::Value) {
        log::debug!("V1EnvironmentTide update: {:?} -> {:?}", path, value);
        log::warn!(
            "V1EnvironmentTide: Unknown value to update: {:?}::{:?}",
            path,
            value
        );
    }
}

impl F64Gettable for V1EnvironmentTide {
    fn get_f64_for_path(&self, _path: &mut Vec<&str>) -> Result<f64, SignalKGetError> {
        Err(SignalKGetError::TBD)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct V1EnvironmentWind {
    pub angle_apparent: Option<V1NumberValue>,
    pub angle_true_ground: Option<V1NumberValue>,
    pub angle_true_water: Option<V1NumberValue>,
    pub direction_change_alarm: Option<V1NumberValue>,
    pub direction_true: Option<V1NumberValue>,
    pub direction_magnetic: Option<V1NumberValue>,
    pub speed_true: Option<V1NumberValue>,
    pub speed_over_ground: Option<V1NumberValue>,
    pub speed_apparent: Option<V1NumberValue>,
}

impl V1EnvironmentWind {
    pub fn update(&mut self, path: &mut Vec<&str>, value: &serde_json::value::Value) {
        log::debug!("V1EnvironmentWind update: {:?} -> {:?}", path, value);
        match path[0] {
            "angleApparent" => {
                if let Some(val) = value.as_f64() {
                    self.angle_apparent = Some(V1NumberValue::builder().value(val).build())
                }
            }
            "angleTrueGround" => {
                if let Some(val) = value.as_f64() {
                    self.angle_true_ground = Some(V1NumberValue::builder().value(val).build())
                }
            }
            "angleTrueWater" => {
                if let Some(val) = value.as_f64() {
                    self.angle_true_water = Some(V1NumberValue::builder().value(val).build())
                }
            }
            "directionChangeAlarm" => {
                if let Some(val) = value.as_f64() {
                    self.direction_change_alarm = Some(V1NumberValue::builder().value(val).build())
                }
            }
            "directionTrue" => {
                if let Some(val) = value.as_f64() {
                    self.direction_true = Some(V1NumberValue::builder().value(val).build())
                }
            }
            "directionMagnetic" => {
                if let Some(val) = value.as_f64() {
                    self.direction_magnetic = Some(V1NumberValue::builder().value(val).build())
                }
            }
            "speedTrue" => {
                if let Some(val) = value.as_f64() {
                    self.speed_true = Some(V1NumberValue::builder().value(val).build())
                }
            }
            "speedOverGround" => {
                if let Some(val) = value.as_f64() {
                    self.speed_over_ground = Some(V1NumberValue::builder().value(val).build())
                }
            }
            "speedApparent" => {
                if let Some(val) = value.as_f64() {
                    self.speed_apparent = Some(V1NumberValue::builder().value(val).build())
                }
            }
            &_ => {
                log::warn!(
                    "V1EnvironmentWind: Unknown value to update: {:?}::{:?}",
                    path,
                    value
                );
            }
        }
    }
}

impl F64Gettable for V1EnvironmentWind {
    fn get_f64_for_path(&self, _path: &mut Vec<&str>) -> Result<f64, SignalKGetError> {
        Err(SignalKGetError::TBD)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct V1EnvironmentTime {
    pub millis: Option<i64>,
    pub timezone_offset: Option<i64>,
    pub timezone_region: Option<String>,
    pub timestamp: Option<String>,
    pub source: Option<V1Source>,
}

impl V1EnvironmentTime {
    pub fn builder() -> V1EnvironmentTimeBuilder {
        V1EnvironmentTimeBuilder::default()
    }
    pub fn update(&mut self, path: &mut Vec<&str>, value: &serde_json::value::Value) {
        log::debug!("V1EnvironmentTime update: {:?} -> {:?}", path, value);
        log::warn!(
            "V1EnvironmentTime: Unknown value to update: {:?}::{:?}",
            path,
            value
        );
    }
}

impl F64Gettable for V1EnvironmentTime {
    fn get_f64_for_path(&self, _path: &mut Vec<&str>) -> Result<f64, SignalKGetError> {
        Err(SignalKGetError::TBD)
    }
}

#[derive(Default)]
pub struct V1EnvironmentTimeBuilder {
    millis: Option<i64>,
    timezone_offset: Option<i64>,
    timezone_region: Option<String>,
    timestamp: Option<String>,
    source: Option<V1Source>,
}

impl V1EnvironmentTimeBuilder {
    pub fn millis(mut self, value: i64) -> V1EnvironmentTimeBuilder {
        self.millis = Some(value);
        self
    }
    pub fn timezone_offset(mut self, value: i64) -> V1EnvironmentTimeBuilder {
        self.timezone_offset = Some(value);
        self
    }
    pub fn timezone_region(mut self, value: String) -> V1EnvironmentTimeBuilder {
        self.timezone_region = Some(value);
        self
    }
    pub fn timestamp(mut self, value: String) -> V1EnvironmentTimeBuilder {
        self.timestamp = Some(value);
        self
    }
    pub fn source(mut self, value: V1Source) -> V1EnvironmentTimeBuilder {
        self.source = Some(value);
        self
    }
    pub fn build(self) -> V1EnvironmentTime {
        V1EnvironmentTime {
            millis: self.millis,
            timezone_offset: self.timezone_offset,
            timezone_region: self.timezone_region,
            timestamp: self.timestamp,
            source: self.source,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct V1EnvironmentMode {
    pub value: Option<String>,
    pub timestamp: Option<String>,
    pub source: Option<V1Source>,
}

impl V1EnvironmentMode {
    pub fn update(&mut self, path: &mut Vec<&str>, value: &serde_json::value::Value) {
        log::debug!("V1EnvironmentMode update: {:?} -> {:?}", path, value);
        log::warn!(
            "V1EnvironmentMode: Unknown value to update: {:?}::{:?}",
            path,
            value
        );
    }
}

impl F64Gettable for V1EnvironmentMode {
    fn get_f64_for_path(&self, _path: &mut Vec<&str>) -> Result<f64, SignalKGetError> {
        Err(SignalKGetError::WrongDataType)
    }
}
