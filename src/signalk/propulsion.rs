use serde::{Deserialize, Serialize};

use crate::signalk::definitions::V1NumberValue;

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct V1Propulsion {
    pub label: String,
    pub state: Option<V1PropulsionState>,
    pub revolutions: Option<V1NumberValue>,
    pub temperature: Option<V1NumberValue>,
    pub oil_temperature: Option<V1NumberValue>,
    pub oil_pressure: Option<V1NumberValue>,
    pub alternator_voltage: Option<V1NumberValue>,
    pub run_time: Option<V1NumberValue>,
    pub coolant_temperature: Option<V1NumberValue>,
    pub coolant_pressure: Option<V1NumberValue>,
    pub boost_pressure: Option<V1NumberValue>,
    pub intake_manifold_temperature: Option<V1NumberValue>,
    pub engine_load: Option<V1NumberValue>,
    pub engine_torque: Option<V1NumberValue>,
    // pub transmission: Option<V1Transmission>,
}

impl V1Propulsion {
    pub fn builder() -> V1PropulsionBuilder {
        V1PropulsionBuilder::default()
    }
}

#[derive(Default)]
pub struct V1PropulsionBuilder {
    label: String,
    state: Option<V1PropulsionState>,
    revolutions: Option<V1NumberValue>,
    temperature: Option<V1NumberValue>,
    oil_temperature: Option<V1NumberValue>,
    oil_pressure: Option<V1NumberValue>,
    alternator_voltage: Option<V1NumberValue>,
    run_time: Option<V1NumberValue>,
    coolant_temperature: Option<V1NumberValue>,
    coolant_pressure: Option<V1NumberValue>,
    boost_pressure: Option<V1NumberValue>,
    intake_manifold_temperature: Option<V1NumberValue>,
    engine_load: Option<V1NumberValue>,
    engine_torque: Option<V1NumberValue>,
}

impl V1PropulsionBuilder {
    pub fn label(mut self, label: String) -> V1PropulsionBuilder {
        self.label = label;
        self
    }
    pub fn state(mut self, state: V1PropulsionState) -> V1PropulsionBuilder {
        self.state = Some(state);
        self
    }
    pub fn revolutions(mut self, revolutions: V1NumberValue) -> V1PropulsionBuilder {
        self.revolutions = Some(revolutions);
        self
    }
    pub fn temperature(mut self, temperature: V1NumberValue) -> V1PropulsionBuilder {
        self.temperature = Some(temperature);
        self
    }
    pub fn oil_temperature(mut self, oil_temperature: V1NumberValue) -> V1PropulsionBuilder {
        self.oil_temperature = Some(oil_temperature);
        self
    }
    pub fn oil_pressure(mut self, oil_pressure: V1NumberValue) -> V1PropulsionBuilder {
        self.oil_pressure = Some(oil_pressure);
        self
    }
    pub fn alternator_voltage(mut self, alternator_voltage: V1NumberValue) -> V1PropulsionBuilder {
        self.alternator_voltage = Some(alternator_voltage);
        self
    }
    pub fn run_time(mut self, run_time: V1NumberValue) -> V1PropulsionBuilder {
        self.run_time = Some(run_time);
        self
    }
    pub fn coolant_temperature(
        mut self,
        coolant_temperature: V1NumberValue,
    ) -> V1PropulsionBuilder {
        self.coolant_temperature = Some(coolant_temperature);
        self
    }
    pub fn coolant_pressure(mut self, coolant_pressure: V1NumberValue) -> V1PropulsionBuilder {
        self.coolant_pressure = Some(coolant_pressure);
        self
    }
    pub fn boost_pressure(mut self, boost_pressure: V1NumberValue) -> V1PropulsionBuilder {
        self.boost_pressure = Some(boost_pressure);
        self
    }
    pub fn intake_manifold_temperature(
        mut self,
        intake_manifold_temperature: V1NumberValue,
    ) -> V1PropulsionBuilder {
        self.intake_manifold_temperature = Some(intake_manifold_temperature);
        self
    }
    pub fn engine_load(mut self, engine_load: V1NumberValue) -> V1PropulsionBuilder {
        self.engine_load = Some(engine_load);
        self
    }
    pub fn engine_torque(mut self, engine_torque: V1NumberValue) -> V1PropulsionBuilder {
        self.engine_torque = Some(engine_torque);
        self
    }
    pub fn build(self) -> V1Propulsion {
        V1Propulsion {
            label: self.label,
            state: self.state,
            revolutions: self.revolutions,
            temperature: self.temperature,
            oil_temperature: self.oil_temperature,
            oil_pressure: self.oil_pressure,
            alternator_voltage: self.alternator_voltage,
            run_time: self.run_time,
            coolant_temperature: self.coolant_temperature,
            coolant_pressure: self.coolant_pressure,
            boost_pressure: self.boost_pressure,
            intake_manifold_temperature: self.intake_manifold_temperature,
            engine_load: self.engine_load,
            engine_torque: self.engine_torque,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
#[serde(rename_all = "lowercase")]
pub enum V1PropulsionState {
    Stopped,
    Started,
    #[default]
    Unusable,
}
