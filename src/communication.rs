use crate::{helper_functions, SignalKGetError};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct V1Communication {
    pub callsign_vhf: Option<String>,
    pub callsign_hf: Option<String>,
    pub phone_number: Option<String>,
    pub email_hf: Option<String>,
    pub email: Option<String>,
    pub sat_phone_number: Option<String>,
    pub skipper_name: Option<String>,
    pub crew_names: Option<Vec<String>>,
}

impl V1Communication {
    pub fn builder() -> V1CommunicationBuilder {
        V1CommunicationBuilder::default()
    }

    pub fn update(&mut self, path: &mut Vec<&str>, value: &serde_json::value::Value) {
        if path.is_empty() {
            log::warn!("Complex value object {:?}", value);
            if let serde_json::Value::Object(ref map) = value {
                for (k, v) in map.iter() {
                    log::debug!(" key: {:?} value: {:?}", k, v);
                    let mut path = vec![k.as_str()];
                    self.update(&mut path, v);
                }
            }
        } else {
            match path[0] {
                "callsignVhf" => {
                    self.callsign_vhf = helper_functions::json_as_optional_string(value)
                }
                "callsignHf" => self.callsign_hf = helper_functions::json_as_optional_string(value),
                "phoneNumber" => {
                    self.phone_number = helper_functions::json_as_optional_string(value)
                }
                "emailHf" => self.email_hf = helper_functions::json_as_optional_string(value),
                "email" => self.email = helper_functions::json_as_optional_string(value),
                "satPhoneNumber" => {
                    self.sat_phone_number = helper_functions::json_as_optional_string(value)
                }
                "skipperName" => {
                    self.skipper_name = helper_functions::json_as_optional_string(value)
                }
                &_ => log::warn!("Unknown value to update: {:?}::{:?}", path, value),
            }
        }
    }

    pub fn get_f64_for_path(&self, _path: &mut [&str]) -> Result<f64, SignalKGetError> {
        Err(SignalKGetError::WrongDataType) // None of the types are f64
    }
}

#[derive(Default)]
pub struct V1CommunicationBuilder {
    callsign_vhf: Option<String>,
    callsign_hf: Option<String>,
    phone_number: Option<String>,
    email_hf: Option<String>,
    email: Option<String>,
    sat_phone_number: Option<String>,
    skipper_name: Option<String>,
    crew_names: Option<Vec<String>>,
}

impl V1CommunicationBuilder {
    pub fn add_callsign_vhf(mut self, value: String) -> V1CommunicationBuilder {
        self.callsign_vhf = Some(value);
        self
    }
    pub fn add_callsign_hf(mut self, value: String) -> V1CommunicationBuilder {
        self.callsign_hf = Some(value);
        self
    }
    pub fn add_phone_number(mut self, value: String) -> V1CommunicationBuilder {
        self.phone_number = Some(value);
        self
    }
    pub fn add_email_hf(mut self, value: String) -> V1CommunicationBuilder {
        self.email_hf = Some(value);
        self
    }
    pub fn add_email(mut self, value: String) -> V1CommunicationBuilder {
        self.email = Some(value);
        self
    }
    pub fn add_sat_phone_number(mut self, value: String) -> V1CommunicationBuilder {
        self.sat_phone_number = Some(value);
        self
    }
    pub fn add_skipper_name(mut self, value: String) -> V1CommunicationBuilder {
        self.skipper_name = Some(value);
        self
    }
    pub fn add_crew_name(mut self, value: String) -> V1CommunicationBuilder {
        if self.crew_names.is_none() {
            self.crew_names = Some(Vec::new());
        }
        if let Some(ref mut crew_names) = self.crew_names {
            crew_names.push(value);
        }
        self
    }

    pub fn build(self) -> V1Communication {
        V1Communication {
            callsign_vhf: self.callsign_vhf,
            callsign_hf: self.callsign_hf,
            phone_number: self.phone_number,
            email_hf: self.email_hf,
            email: self.email,
            sat_phone_number: self.sat_phone_number,
            skipper_name: self.skipper_name,
            crew_names: self.crew_names,
        }
    }
}
