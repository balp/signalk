use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::definitions::{V1CommonValueFields, V1NumberValue, V2NumberValue};
use crate::{helper_functions, SignalKGetError};

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct V1Electrical {
    pub batteries: Option<HashMap<String, V1Battery>>,
    pub inverters: Option<HashMap<String, V1Inverter>>,
    pub chargers: Option<HashMap<String, V1Charger>>,
    pub alternators: Option<HashMap<String, V1Alternator>>,
    pub solar: Option<HashMap<String, V1Solar>>,
    pub ac: Option<HashMap<String, V1ACBus>>,
}

impl V1Electrical {
    pub fn builder() -> V1ElectricalBuilder {
        V1ElectricalBuilder::default()
    }

    pub fn update(&mut self, path: &mut Vec<&str>, value: &serde_json::value::Value) {
        match path[0] {
            "batteries" => {
                if self.batteries.is_none() {
                    self.batteries = Some(HashMap::new());
                }
                if let Some(ref mut batteries) = self.batteries {
                    let id = path[1].to_string();
                    if !batteries.contains_key(&id) {
                        batteries.insert(
                            id.clone(),
                            V1Battery::builder()
                                .identity(V1ElectricalIdentity::builder().name(id.clone()).build())
                                .build(),
                        );
                    }
                    let mut t = batteries.get_mut(&id);
                    if let Some(ref mut battery) = t {
                        path.remove(0); // Remove batteries
                        path.remove(0); // and the index of the bank
                        battery.update(path, value);
                    }
                }
            }

            &_ => {
                log::warn!(
                    "V1Electrical: Unknown value to update: {:?}::{:?}",
                    path,
                    value
                );
            }
        }
    }

    pub fn get_f64_for_path(&self, path: &mut Vec<&str>) -> Result<f64, SignalKGetError> {
        match path[0] {
            "batteries" => {
                log::debug!("batteries path: {:?}", path);
                if let Some(ref batteries) = self.batteries {
                    if let Some(battery) = batteries.get(path[1]) {
                        path.remove(0); // Remove both batteries
                        path.remove(0); // Remove name instance
                        battery.get_f64_for_path(path)
                    } else {
                        Err(SignalKGetError::NoSuchPath)
                    }
                } else {
                    Err(SignalKGetError::ValueNotSet)
                }
            }
            &_ => {
                log::info!("path: {:?}", path);
                Err(SignalKGetError::NoSuchPath)
            }
        }
    }
}

#[derive(Default)]
pub struct V1ElectricalBuilder {
    batteries: Option<HashMap<String, V1Battery>>,
    inverters: Option<HashMap<String, V1Inverter>>,
    chargers: Option<HashMap<String, V1Charger>>,
    alternators: Option<HashMap<String, V1Alternator>>,
    solar: Option<HashMap<String, V1Solar>>,
    ac: Option<HashMap<String, V1ACBus>>,
}

impl V1ElectricalBuilder {
    pub fn add_battery(mut self, key: String, value: V1Battery) -> V1ElectricalBuilder {
        if self.batteries.is_none() {
            self.batteries = Some(HashMap::new());
        }
        if let Some(ref mut x) = self.batteries {
            x.insert(key, value);
        }
        self
    }
    pub fn add_inverter(mut self, key: String, value: V1Inverter) -> V1ElectricalBuilder {
        if self.inverters.is_none() {
            self.inverters = Some(HashMap::new());
        }
        if let Some(ref mut x) = self.inverters {
            x.insert(key, value);
        }
        self
    }
    pub fn add_charger(mut self, key: String, value: V1Charger) -> V1ElectricalBuilder {
        if self.chargers.is_none() {
            self.chargers = Some(HashMap::new());
        }
        if let Some(ref mut x) = self.chargers {
            x.insert(key, value);
        }
        self
    }
    pub fn add_alternator(mut self, key: String, value: V1Alternator) -> V1ElectricalBuilder {
        if self.alternators.is_none() {
            self.alternators = Some(HashMap::new());
        }
        if let Some(ref mut x) = self.alternators {
            x.insert(key, value);
        }
        self
    }
    pub fn add_solar(mut self, key: String, value: V1Solar) -> V1ElectricalBuilder {
        if self.solar.is_none() {
            self.solar = Some(HashMap::new());
        }
        if let Some(ref mut x) = self.solar {
            x.insert(key, value);
        }
        self
    }
    pub fn add_ac(mut self, key: String, value: V1ACBus) -> V1ElectricalBuilder {
        if self.ac.is_none() {
            self.ac = Some(HashMap::new());
        }
        if let Some(ref mut x) = self.ac {
            x.insert(key, value);
        }
        self
    }

    pub fn build(self) -> V1Electrical {
        V1Electrical {
            batteries: self.batteries,
            inverters: self.inverters,
            chargers: self.chargers,
            alternators: self.alternators,
            solar: self.solar,
            ac: self.ac,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct V1ElectricalIdentity {
    pub name: Option<String>,
    pub location: Option<String>,
    pub date_installed: Option<String>,
    pub manufacturer: Option<V1ElectricalManufacturer>,
}

impl V1ElectricalIdentity {
    pub fn builder() -> V1ElectricalIdentityBuilder {
        V1ElectricalIdentityBuilder::default()
    }
}

#[derive(Default)]
pub struct V1ElectricalIdentityBuilder {
    name: Option<String>,
    location: Option<String>,
    date_installed: Option<String>,
    manufacturer: Option<V1ElectricalManufacturer>,
}

impl V1ElectricalIdentityBuilder {
    pub fn name(mut self, value: String) -> V1ElectricalIdentityBuilder {
        self.name = Some(value);
        self
    }
    pub fn location(mut self, value: String) -> V1ElectricalIdentityBuilder {
        self.location = Some(value);
        self
    }
    pub fn date_installed(mut self, value: String) -> V1ElectricalIdentityBuilder {
        self.date_installed = Some(value);
        self
    }
    pub fn manufacturer(mut self, value: V1ElectricalManufacturer) -> V1ElectricalIdentityBuilder {
        self.manufacturer = Some(value);
        self
    }
    pub fn build(self) -> V1ElectricalIdentity {
        V1ElectricalIdentity {
            name: self.name,
            location: self.location,
            date_installed: self.date_installed,
            manufacturer: self.manufacturer,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct V1ElectricalManufacturer {
    pub name: Option<String>,
    pub model: Option<String>,
    #[serde(rename = "URL")]
    pub url: Option<String>,
}

impl V1ElectricalManufacturer {
    pub fn builder() -> V1ElectricalManufacturerBuilder {
        V1ElectricalManufacturerBuilder::default()
    }
}

#[derive(Default)]
pub struct V1ElectricalManufacturerBuilder {
    name: Option<String>,
    model: Option<String>,
    url: Option<String>,
}

impl V1ElectricalManufacturerBuilder {
    pub fn name(mut self, value: String) -> V1ElectricalManufacturerBuilder {
        self.name = Some(value);
        self
    }
    pub fn model(mut self, value: String) -> V1ElectricalManufacturerBuilder {
        self.model = Some(value);
        self
    }
    pub fn url(mut self, value: String) -> V1ElectricalManufacturerBuilder {
        self.url = Some(value);
        self
    }
    pub fn build(self) -> V1ElectricalManufacturer {
        V1ElectricalManufacturer {
            name: self.name,
            model: self.model,
            url: self.url,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct V1ElectricalDCQualities {
    pub associated_bus: Option<String>,
    pub voltage: Option<V1ElectricalDCVoltageValue>,
    pub current: Option<V1ElectricalDCCurrentValue>,
    pub temperature: Option<V1ElectricalDCTemperatureValue>,
}

impl V1ElectricalDCQualities {
    pub fn builder() -> V1ElectricalDCQualitiesBuilder {
        V1ElectricalDCQualitiesBuilder::default()
    }
}

#[derive(Default)]
pub struct V1ElectricalDCQualitiesBuilder {
    associated_bus: Option<String>,
    voltage: Option<V1ElectricalDCVoltageValue>,
    current: Option<V1ElectricalDCCurrentValue>,
    temperature: Option<V1ElectricalDCTemperatureValue>,
}

impl V1ElectricalDCQualitiesBuilder {
    pub fn associated_bus(mut self, value: String) -> V1ElectricalDCQualitiesBuilder {
        self.associated_bus = Some(value);
        self
    }
    pub fn voltage(mut self, value: V1ElectricalDCVoltageValue) -> V1ElectricalDCQualitiesBuilder {
        self.voltage = Some(value);
        self
    }
    pub fn current(mut self, value: V1ElectricalDCCurrentValue) -> V1ElectricalDCQualitiesBuilder {
        self.current = Some(value);
        self
    }
    pub fn temperature(
        mut self,
        value: V1ElectricalDCTemperatureValue,
    ) -> V1ElectricalDCQualitiesBuilder {
        self.temperature = Some(value);
        self
    }
    pub fn build(self) -> V1ElectricalDCQualities {
        V1ElectricalDCQualities {
            associated_bus: self.associated_bus,
            voltage: self.voltage,
            current: self.current,
            temperature: self.temperature,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct V1ElectricalDCVoltageValue {
    #[serde(flatten)]
    pub value: Option<V1NumberValue>,
    pub ripple: Option<V1NumberValue>,
    pub meta: Option<V1ElectricalDCVoltageMeta>,
}

impl V1ElectricalDCVoltageValue {
    pub fn builder() -> V1ElectricalDCVoltageValueBuilder {
        V1ElectricalDCVoltageValueBuilder::default()
    }
}

#[derive(Default)]
pub struct V1ElectricalDCVoltageValueBuilder {
    value: Option<V1NumberValue>,
    ripple: Option<V1NumberValue>,
    meta: Option<V1ElectricalDCVoltageMeta>,
}

impl V1ElectricalDCVoltageValueBuilder {
    pub fn value(mut self, value: V1NumberValue) -> V1ElectricalDCVoltageValueBuilder {
        self.value = Some(value);
        self
    }
    pub fn ripple(mut self, value: V1NumberValue) -> V1ElectricalDCVoltageValueBuilder {
        self.ripple = Some(value);
        self
    }
    pub fn meta(mut self, value: V1ElectricalDCVoltageMeta) -> V1ElectricalDCVoltageValueBuilder {
        self.meta = Some(value);
        self
    }
    pub fn build(self) -> V1ElectricalDCVoltageValue {
        V1ElectricalDCVoltageValue {
            value: self.value,
            ripple: self.ripple,
            meta: self.meta,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct V1ElectricalDCVoltageMeta {
    pub nominal: Option<f64>,
    pub warn_upper: Option<f64>,
    pub warn_lower: Option<f64>,
    pub fault_upper: Option<f64>,
    pub fault_lower: Option<f64>,
}

impl V1ElectricalDCVoltageMeta {
    pub fn builder() -> V1ElectricalDCVoltageMetaBuilder {
        V1ElectricalDCVoltageMetaBuilder::default()
    }
}

#[derive(Default)]
pub struct V1ElectricalDCVoltageMetaBuilder {
    nominal: Option<f64>,
    warn_upper: Option<f64>,
    warn_lower: Option<f64>,
    fault_upper: Option<f64>,
    fault_lower: Option<f64>,
}

impl V1ElectricalDCVoltageMetaBuilder {
    pub fn nominal(mut self, value: f64) -> V1ElectricalDCVoltageMetaBuilder {
        self.nominal = Some(value);
        self
    }
    pub fn warn_upper(mut self, value: f64) -> V1ElectricalDCVoltageMetaBuilder {
        self.warn_upper = Some(value);
        self
    }
    pub fn warn_lower(mut self, value: f64) -> V1ElectricalDCVoltageMetaBuilder {
        self.warn_lower = Some(value);
        self
    }
    pub fn fault_upper(mut self, value: f64) -> V1ElectricalDCVoltageMetaBuilder {
        self.fault_upper = Some(value);
        self
    }
    pub fn fault_lower(mut self, value: f64) -> V1ElectricalDCVoltageMetaBuilder {
        self.fault_lower = Some(value);
        self
    }
    pub fn build(self) -> V1ElectricalDCVoltageMeta {
        V1ElectricalDCVoltageMeta {
            nominal: self.nominal,
            warn_upper: self.warn_upper,
            warn_lower: self.warn_lower,
            fault_upper: self.fault_upper,
            fault_lower: self.fault_lower,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct V1ElectricalDCCurrentValue {
    #[serde(flatten)]
    pub value: Option<V1NumberValue>,
    pub meta: Option<V1ElectricalDCCurrentMeta>,
}

impl V1ElectricalDCCurrentValue {
    pub fn builder() -> V1ElectricalDCCurrentValueBuilder {
        V1ElectricalDCCurrentValueBuilder::default()
    }
}

#[derive(Default)]
pub struct V1ElectricalDCCurrentValueBuilder {
    value: Option<V1NumberValue>,
    meta: Option<V1ElectricalDCCurrentMeta>,
}

impl V1ElectricalDCCurrentValueBuilder {
    pub fn value(mut self, value: V1NumberValue) -> V1ElectricalDCCurrentValueBuilder {
        self.value = Some(value);
        self
    }
    pub fn meta(mut self, value: V1ElectricalDCCurrentMeta) -> V1ElectricalDCCurrentValueBuilder {
        self.meta = Some(value);
        self
    }
    pub fn build(self) -> V1ElectricalDCCurrentValue {
        V1ElectricalDCCurrentValue {
            value: self.value,
            meta: self.meta,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct V1ElectricalDCCurrentMeta {
    #[serde(flatten)]
    pub warn_upper: Option<f64>,
    pub warn_lower: Option<f64>,
    pub fault_upper: Option<f64>,
    pub fault_lower: Option<f64>,
}

impl V1ElectricalDCCurrentMeta {
    pub fn new(
        warn_upper: Option<f64>,
        warn_lower: Option<f64>,
        fault_upper: Option<f64>,
        fault_lower: Option<f64>,
    ) -> Self {
        Self {
            warn_upper,
            warn_lower,
            fault_upper,
            fault_lower,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct V1ElectricalDCTemperatureValue {
    #[serde(flatten)]
    pub value: Option<V1NumberValue>,
    pub warn_upper: Option<f64>,
    pub warn_lower: Option<f64>,
    pub fault_upper: Option<f64>,
    pub fault_lower: Option<f64>,
}

impl V1ElectricalDCTemperatureValue {
    pub fn new(
        value: Option<V1NumberValue>,
        warn_upper: Option<f64>,
        warn_lower: Option<f64>,
        fault_upper: Option<f64>,
        fault_lower: Option<f64>,
    ) -> Self {
        Self {
            value,
            warn_upper,
            warn_lower,
            fault_upper,
            fault_lower,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct V1ElectricalACQualities {
    pub associated_bus: Option<String>,
    pub line_neutral_voltage: Option<V1NumberValue>,
    pub line_line_voltage: Option<V1NumberValue>,
    pub current: Option<V1NumberValue>,
    pub frequency: Option<V1NumberValue>,
    pub reactive_power: Option<V1NumberValue>,
    pub power_factor: Option<V1NumberValue>,
    pub power_factor_lagging: Option<String>,
    pub real_power: Option<V1NumberValue>,
    pub apparent_power: Option<V1NumberValue>,
}

impl V1ElectricalACQualities {
    pub fn builder() -> V1ElectricalACQualitiesBuilder {
        V1ElectricalACQualitiesBuilder::default()
    }
}

#[derive(Default)]
pub struct V1ElectricalACQualitiesBuilder {
    associated_bus: Option<String>,
    line_neutral_voltage: Option<V1NumberValue>,
    line_line_voltage: Option<V1NumberValue>,
    current: Option<V1NumberValue>,
    frequency: Option<V1NumberValue>,
    reactive_power: Option<V1NumberValue>,
    power_factor: Option<V1NumberValue>,
    power_factor_lagging: Option<String>,
    real_power: Option<V1NumberValue>,
    apparent_power: Option<V1NumberValue>,
}

impl V1ElectricalACQualitiesBuilder {
    pub fn associated_bus(mut self, value: String) -> V1ElectricalACQualitiesBuilder {
        self.associated_bus = Some(value);
        self
    }
    pub fn line_neutral_voltage(mut self, value: V1NumberValue) -> V1ElectricalACQualitiesBuilder {
        self.line_neutral_voltage = Some(value);
        self
    }
    pub fn line_line_voltage(mut self, value: V1NumberValue) -> V1ElectricalACQualitiesBuilder {
        self.line_line_voltage = Some(value);
        self
    }
    pub fn current(mut self, value: V1NumberValue) -> V1ElectricalACQualitiesBuilder {
        self.current = Some(value);
        self
    }
    pub fn frequency(mut self, value: V1NumberValue) -> V1ElectricalACQualitiesBuilder {
        self.frequency = Some(value);
        self
    }
    pub fn reactive_power(mut self, value: V1NumberValue) -> V1ElectricalACQualitiesBuilder {
        self.reactive_power = Some(value);
        self
    }
    pub fn power_factor(mut self, value: V1NumberValue) -> V1ElectricalACQualitiesBuilder {
        self.power_factor = Some(value);
        self
    }
    pub fn power_factor_lagging(mut self, value: String) -> V1ElectricalACQualitiesBuilder {
        self.power_factor_lagging = Some(value);
        self
    }
    pub fn real_power(mut self, value: V1NumberValue) -> V1ElectricalACQualitiesBuilder {
        self.real_power = Some(value);
        self
    }
    pub fn apparent_power(mut self, value: V1NumberValue) -> V1ElectricalACQualitiesBuilder {
        self.apparent_power = Some(value);
        self
    }
    pub fn build(self) -> V1ElectricalACQualities {
        V1ElectricalACQualities {
            associated_bus: self.associated_bus,
            line_neutral_voltage: self.line_neutral_voltage,
            line_line_voltage: self.line_line_voltage,
            current: self.current,
            frequency: self.frequency,
            reactive_power: self.reactive_power,
            power_factor: self.power_factor,
            power_factor_lagging: self.power_factor_lagging,
            real_power: self.real_power,
            apparent_power: self.apparent_power,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct V1BatteryTemperature {
    #[serde(flatten)]
    pub value: Option<V1NumberValue>,

    pub limit_discharge_lower: Option<f64>,
    pub limit_discharge_upper: Option<f64>,
    pub limit_recharge_lower: Option<f64>,
    pub limit_recharge_upper: Option<f64>,
}

impl V1BatteryTemperature {
    pub fn new(
        value: Option<V1NumberValue>,
        limit_discharge_lower: Option<f64>,
        limit_discharge_upper: Option<f64>,
        limit_recharge_lower: Option<f64>,
        limit_recharge_upper: Option<f64>,
    ) -> Self {
        Self {
            value,
            limit_discharge_lower,
            limit_discharge_upper,
            limit_recharge_lower,
            limit_recharge_upper,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct V1BatteryCapacity {
    pub nominal: Option<V2NumberValue>,
    pub actual: Option<V2NumberValue>,
    pub remaining: Option<V2NumberValue>,
    pub discharge_limit: Option<V2NumberValue>,
    pub state_of_charge: Option<V2NumberValue>,
    pub state_of_health: Option<V2NumberValue>,
    pub discharge_since_full: Option<V2NumberValue>,
    pub time_remaining: Option<V2NumberValue>,
}

impl V1BatteryCapacity {
    pub fn builder() -> V1BatteryCapacityBuilder {
        V1BatteryCapacityBuilder::default()
    }
    pub fn update(&mut self, path: &mut Vec<&str>, value: &serde_json::value::Value) {
        match path[0] {
            "nominal" => self.nominal = V2NumberValue::from_value(value),
            "actual" => self.actual = V2NumberValue::from_value(value),
            "remaining" => self.remaining = V2NumberValue::from_value(value),
            "dischargeLimit" => self.discharge_limit = V2NumberValue::from_value(value),
            "stateOfCharge" => self.state_of_charge = V2NumberValue::from_value(value),
            "stateOfHealth" => self.state_of_health = V2NumberValue::from_value(value),
            "dischargeSinceFull" => self.discharge_since_full = V2NumberValue::from_value(value),
            "timeRemaining" => self.time_remaining = V2NumberValue::from_value(value),
            &_ => {
                log::warn!(
                    "V1BatteryCapacity: Unknown value to update: {:?}::{:?}",
                    path,
                    value
                );
            }
        }
    }
}

#[derive(Default)]
pub struct V1BatteryCapacityBuilder {
    nominal: Option<V2NumberValue>,
    actual: Option<V2NumberValue>,
    remaining: Option<V2NumberValue>,
    discharge_limit: Option<V2NumberValue>,
    state_of_charge: Option<V2NumberValue>,
    state_of_health: Option<V2NumberValue>,
    discharge_since_full: Option<V2NumberValue>,
    time_remaining: Option<V2NumberValue>,
}

impl V1BatteryCapacityBuilder {
    pub fn nominal(mut self, value: i64) -> V1BatteryCapacityBuilder {
        self.nominal = Some(V2NumberValue::Int(value));
        self
    }
    pub fn actual(mut self, value: i64) -> V1BatteryCapacityBuilder {
        self.actual = Some(V2NumberValue::Int(value));
        self
    }
    pub fn remaining(mut self, value: i64) -> V1BatteryCapacityBuilder {
        self.remaining = Some(V2NumberValue::Int(value));
        self
    }
    pub fn discharge_limit(mut self, value: i64) -> V1BatteryCapacityBuilder {
        self.discharge_limit = Some(V2NumberValue::Int(value));
        self
    }
    pub fn state_of_charge(mut self, value: i64) -> V1BatteryCapacityBuilder {
        self.state_of_charge = Some(V2NumberValue::Int(value));
        self
    }
    pub fn state_of_health(mut self, value: i64) -> V1BatteryCapacityBuilder {
        self.state_of_health = Some(V2NumberValue::Int(value));
        self
    }
    pub fn discharge_since_full(mut self, value: i64) -> V1BatteryCapacityBuilder {
        self.discharge_since_full = Some(V2NumberValue::Int(value));
        self
    }
    pub fn time_remaining(mut self, value: i64) -> V1BatteryCapacityBuilder {
        self.time_remaining = Some(V2NumberValue::Int(value));
        self
    }
    pub fn build(self) -> V1BatteryCapacity {
        V1BatteryCapacity {
            nominal: self.nominal,
            actual: self.actual,
            remaining: self.remaining,
            discharge_limit: self.discharge_limit,
            state_of_charge: self.state_of_charge,
            state_of_health: self.state_of_health,
            discharge_since_full: self.discharge_since_full,
            time_remaining: self.time_remaining,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct V1Battery {
    #[serde(flatten)]
    pub identity: Option<V1ElectricalIdentity>,
    #[serde(flatten)]
    pub dc_qualities: Option<V1ElectricalDCQualities>,
    pub chemistry: Option<String>,
    pub temperature: Option<V1BatteryTemperature>,
    pub capacity: Option<V1BatteryCapacity>,
    pub lifetime_discharge: Option<V1NumberValue>,
    pub lifetime_recharge: Option<V1NumberValue>,
}

impl V1Battery {
    pub fn builder() -> V1BatteryBuilder {
        V1BatteryBuilder::default()
    }
    pub fn update(&mut self, path: &mut Vec<&str>, value: &serde_json::value::Value) {
        match path[0] {
            // V1ElectricalIdentity
            // name
            // location
            // dateInstalled
            // manufacturer
            // V1ElectricalDCQualities
            // associated_bus
            "voltage" => {
                if self.dc_qualities.is_none() {
                    self.dc_qualities = Some(V1ElectricalDCQualities::default());
                }
                if let Some(ref mut dc) = self.dc_qualities {
                    let val = V1NumberValue::builder().json_value(value).build();
                    dc.voltage = Some(V1ElectricalDCVoltageValue::builder().value(val).build());
                }
            }
            "current" => {
                if self.dc_qualities.is_none() {
                    self.dc_qualities = Some(V1ElectricalDCQualities::default());
                }
                if let Some(ref mut dc) = self.dc_qualities {
                    let val = V1NumberValue::builder().json_value(value).build();
                    dc.current = Some(V1ElectricalDCCurrentValue::builder().value(val).build());
                }
            }
            "temperature" => {
                if self.temperature.is_none() {
                    self.temperature = Some(V1BatteryTemperature::default());
                }
                if let Some(ref mut temperature) = self.temperature {
                    let val = V1NumberValue::builder().json_value(value).build();
                    temperature.value = Some(val);
                }
            }
            "capacity" => {
                if self.capacity.is_none() {
                    self.capacity = Some(V1BatteryCapacity::default());
                }
                if let Some(ref mut capacity) = self.capacity {
                    path.remove(0);
                    capacity.update(path, value);
                }
            }
            &_ => {
                log::warn!(
                    "V1Battery: {:?}--Unknown value to update: {:?}::{:?}",
                    self,
                    path,
                    value
                );
            }
        }
    }

    pub fn get_f64_for_path(&self, path: &mut Vec<&str>) -> Result<f64, SignalKGetError> {
        log::info!("V1Batterypath: {:?}", path);
        match path[0] {
            "voltage" => {
                if let Some(ref dc) = self.dc_qualities {
                    if let Some(ref voltage) = dc.voltage {
                        helper_functions::get_f64_value(&voltage.value)
                    } else {
                        Err(SignalKGetError::ValueNotSet)
                    }
                } else {
                    Err(SignalKGetError::ValueNotSet)
                }
            }
            &_ => Err(SignalKGetError::TBD),
        }
    }
}

#[derive(Default)]
pub struct V1BatteryBuilder {
    identity: Option<V1ElectricalIdentity>,
    dc_qualities: Option<V1ElectricalDCQualities>,
    chemistry: Option<String>,
    temperature: Option<V1BatteryTemperature>,
    capacity: Option<V1BatteryCapacity>,
    lifetime_discharge: Option<V1NumberValue>,
    lifetime_recharge: Option<V1NumberValue>,
}

impl V1BatteryBuilder {
    pub fn identity(mut self, value: V1ElectricalIdentity) -> V1BatteryBuilder {
        self.identity = Some(value);
        self
    }
    pub fn dc_qualities(mut self, value: V1ElectricalDCQualities) -> V1BatteryBuilder {
        self.dc_qualities = Some(value);
        self
    }
    pub fn chemistry(mut self, value: String) -> V1BatteryBuilder {
        self.chemistry = Some(value);
        self
    }
    pub fn temperature(mut self, value: V1BatteryTemperature) -> V1BatteryBuilder {
        self.temperature = Some(value);
        self
    }
    pub fn capacity(mut self, value: V1BatteryCapacity) -> V1BatteryBuilder {
        self.capacity = Some(value);
        self
    }
    pub fn lifetime_discharge(mut self, value: f64) -> V1BatteryBuilder {
        self.lifetime_discharge = Some(V1NumberValue::builder().value(value).build());
        self
    }
    pub fn lifetime_recharge(mut self, value: f64) -> V1BatteryBuilder {
        self.lifetime_recharge = Some(V1NumberValue::builder().value(value).build());
        self
    }
    pub fn build(self) -> V1Battery {
        V1Battery {
            identity: self.identity,
            dc_qualities: self.dc_qualities,
            chemistry: self.chemistry,
            temperature: self.temperature,
            capacity: self.capacity,
            lifetime_discharge: self.lifetime_discharge,
            lifetime_recharge: self.lifetime_recharge,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct V1ElectricalInverterMode {
    #[serde(flatten)]
    pub common: Option<V1CommonValueFields>,
    pub value: Option<String>,
}

impl V1ElectricalInverterMode {
    pub fn new(common: Option<V1CommonValueFields>, value: Option<String>) -> Self {
        Self { common, value }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct V1ElectricalChargerQualities {
    #[serde(flatten)]
    pub common: Option<V1CommonValueFields>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct V1Inverter {
    #[serde(flatten)]
    pub identity: Option<V1ElectricalIdentity>,
    pub dc: Option<V1ElectricalDCQualities>,
    pub ac: Option<V1ElectricalACQualities>,
    pub inverter_mode: Option<V1ElectricalInverterMode>,
}

impl V1Inverter {
    pub fn new(
        identity: Option<V1ElectricalIdentity>,
        dc: Option<V1ElectricalDCQualities>,
        ac: Option<V1ElectricalACQualities>,
        inverter_mode: Option<V1ElectricalInverterMode>,
    ) -> Self {
        Self {
            identity,
            dc,
            ac,
            inverter_mode,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct V1Charger {
    #[serde(flatten)]
    pub identity: Option<V1ElectricalIdentity>,
    #[serde(flatten)]
    pub dc: Option<V1ElectricalDCQualities>,
    #[serde(flatten)]
    pub charger: Option<V1ElectricalChargerQualities>,
}

impl V1Charger {
    pub fn new(
        identity: Option<V1ElectricalIdentity>,
        dc: Option<V1ElectricalDCQualities>,
        charger: Option<V1ElectricalChargerQualities>,
    ) -> Self {
        Self {
            identity,
            dc,
            charger,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct V1Alternator {
    #[serde(flatten)]
    pub identity: Option<V1ElectricalIdentity>,
    #[serde(flatten)]
    pub dc: Option<V1ElectricalDCQualities>,
    #[serde(flatten)]
    pub charger: Option<V1ElectricalChargerQualities>,
    pub revolutions: Option<f64>,
    pub pulley_ratio: Option<f64>,
    pub field_drive: Option<f64>,
    pub regulator_temperature: Option<f64>,
}

impl V1Alternator {
    pub fn builder() -> V1AlternatorBuilder {
        V1AlternatorBuilder::default()
    }
}

#[derive(Default)]
pub struct V1AlternatorBuilder {
    identity: Option<V1ElectricalIdentity>,
    dc: Option<V1ElectricalDCQualities>,
    charger: Option<V1ElectricalChargerQualities>,
    revolutions: Option<f64>,
    pulley_ratio: Option<f64>,
    field_drive: Option<f64>,
    regulator_temperature: Option<f64>,
}

impl V1AlternatorBuilder {
    pub fn identity(mut self, value: V1ElectricalIdentity) -> V1AlternatorBuilder {
        self.identity = Some(value);
        self
    }
    pub fn dc(mut self, value: V1ElectricalDCQualities) -> V1AlternatorBuilder {
        self.dc = Some(value);
        self
    }
    pub fn charger(mut self, value: V1ElectricalChargerQualities) -> V1AlternatorBuilder {
        self.charger = Some(value);
        self
    }
    pub fn revolutions(mut self, value: f64) -> V1AlternatorBuilder {
        self.revolutions = Some(value);
        self
    }
    pub fn pulley_ratio(mut self, value: f64) -> V1AlternatorBuilder {
        self.pulley_ratio = Some(value);
        self
    }
    pub fn field_drive(mut self, value: f64) -> V1AlternatorBuilder {
        self.field_drive = Some(value);
        self
    }
    pub fn regulator_temperature(mut self, value: f64) -> V1AlternatorBuilder {
        self.regulator_temperature = Some(value);
        self
    }
    pub fn build(self) -> V1Alternator {
        V1Alternator {
            identity: self.identity,
            dc: self.dc,
            charger: self.charger,
            revolutions: self.revolutions,
            pulley_ratio: self.pulley_ratio,
            field_drive: self.field_drive,
            regulator_temperature: self.regulator_temperature,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct V1SolarLoad {
    #[serde(flatten)]
    pub common: Option<V1CommonValueFields>,
    pub value: Option<String>,
}

impl V1SolarLoad {
    pub fn new(common: Option<V1CommonValueFields>, value: Option<String>) -> Self {
        Self { common, value }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct V1Solar {
    #[serde(flatten)]
    pub identity: Option<V1ElectricalIdentity>,
    #[serde(flatten)]
    pub dc: Option<V1ElectricalDCQualities>,
    #[serde(flatten)]
    pub charger: Option<V1ElectricalChargerQualities>,
    pub value: Option<String>,
    pub panel_voltage: Option<V1NumberValue>,
    pub panel_current: Option<V1NumberValue>,
    pub panel_temperature: Option<V1NumberValue>,
    pub load: Option<V1SolarLoad>,
    pub load_current: Option<V1NumberValue>,
}

impl V1Solar {
    pub fn builder() -> V1SolarBuilder {
        V1SolarBuilder::default()
    }
}

#[derive(Default)]
pub struct V1SolarBuilder {
    identity: Option<V1ElectricalIdentity>,
    dc: Option<V1ElectricalDCQualities>,
    charger: Option<V1ElectricalChargerQualities>,
    value: Option<String>,
    panel_voltage: Option<V1NumberValue>,
    panel_current: Option<V1NumberValue>,
    panel_temperature: Option<V1NumberValue>,
    load: Option<V1SolarLoad>,
    load_current: Option<V1NumberValue>,
}

impl V1SolarBuilder {
    pub fn identity(mut self, value: V1ElectricalIdentity) -> V1SolarBuilder {
        self.identity = Some(value);
        self
    }
    pub fn dc(mut self, value: V1ElectricalDCQualities) -> V1SolarBuilder {
        self.dc = Some(value);
        self
    }
    pub fn charger(mut self, value: V1ElectricalChargerQualities) -> V1SolarBuilder {
        self.charger = Some(value);
        self
    }
    pub fn value(mut self, value: String) -> V1SolarBuilder {
        self.value = Some(value);
        self
    }
    pub fn panel_voltage(mut self, value: f64) -> V1SolarBuilder {
        self.panel_voltage = Some(V1NumberValue::builder().value(value).build());
        self
    }
    pub fn panel_current(mut self, value: f64) -> V1SolarBuilder {
        self.panel_current = Some(V1NumberValue::builder().value(value).build());
        self
    }
    pub fn panel_temperature(mut self, value: f64) -> V1SolarBuilder {
        self.panel_temperature = Some(V1NumberValue::builder().value(value).build());
        self
    }
    pub fn load(mut self, value: V1SolarLoad) -> V1SolarBuilder {
        self.load = Some(value);
        self
    }
    pub fn load_current(mut self, value: f64) -> V1SolarBuilder {
        self.load_current = Some(V1NumberValue::builder().value(value).build());
        self
    }
    pub fn build(self) -> V1Solar {
        V1Solar {
            identity: self.identity,
            dc: self.dc,
            charger: self.charger,
            value: self.value,
            panel_voltage: self.panel_voltage,
            panel_current: self.panel_current,
            panel_temperature: self.panel_temperature,
            load: self.load,
            load_current: self.load_current,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct V1ACBus {
    #[serde(flatten)]
    pub identity: Option<V1ElectricalIdentity>,
    pub phase: HashMap<String, V1ElectricalACQualities>,
}

impl V1ACBus {
    pub fn builder() -> V1ACBusBuilder {
        V1ACBusBuilder::default()
    }
}

#[derive(Default)]
pub struct V1ACBusBuilder {
    identity: Option<V1ElectricalIdentity>,
    phase: HashMap<String, V1ElectricalACQualities>,
}

impl V1ACBusBuilder {
    pub fn identity(mut self, value: V1ElectricalIdentity) -> V1ACBusBuilder {
        self.identity = Some(value);
        self
    }
    pub fn add_phase(mut self, key: String, value: V1ElectricalACQualities) -> V1ACBusBuilder {
        self.phase.insert(key, value);
        self
    }
    pub fn build(self) -> V1ACBus {
        V1ACBus {
            identity: self.identity,
            phase: self.phase,
        }
    }
}
