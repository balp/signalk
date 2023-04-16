use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct V1Hello {
    pub name: Option<String>,
    pub version: String,
    pub timestamp: Option<String>,
    pub start_time: Option<String>,
    pub playback_rate: Option<f64>,
    #[serde(rename = "self")]
    pub self_: Option<String>,
    pub roles: Vec<String>,
}

impl V1Hello {
    pub fn builder() -> V1HelloBuilder {
        V1HelloBuilder::default()
    }
}

#[derive(Default)]
pub struct V1HelloBuilder {
    name: Option<String>,
    version: String,
    timestamp: Option<String>,
    start_time: Option<String>,
    playback_rate: Option<f64>,
    self_: Option<String>,
    roles: Vec<String>,
}

impl V1HelloBuilder {
    pub fn name(mut self, value: String) -> V1HelloBuilder {
        self.name = Some(value);
        self
    }
    pub fn version(mut self, value: String) -> V1HelloBuilder {
        self.version = value;
        self
    }
    pub fn timestamp(mut self, value: String) -> V1HelloBuilder {
        self.timestamp = Some(value);
        self
    }
    pub fn start_time(mut self, value: String) -> V1HelloBuilder {
        self.start_time = Some(value);
        self
    }
    pub fn playback_rate(mut self, value: f64) -> V1HelloBuilder {
        self.playback_rate = Some(value);
        self
    }
    pub fn self_(mut self, value: String) -> V1HelloBuilder {
        self.self_ = Some(value);
        self
    }
    pub fn role(mut self, value: String) -> V1HelloBuilder {
        self.roles.push(value);
        self
    }
    pub fn build(self) -> V1Hello {
        V1Hello {
            name: self.name,
            version: self.version,
            timestamp: self.timestamp,
            start_time: self.start_time,
            playback_rate: self.playback_rate,
            self_: self.self_,
            roles: self.roles,
        }
    }
}
