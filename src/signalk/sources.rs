use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::signalk::definitions::V1Attr;

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct V1Sources {
    #[serde(rename = "_attr")]
    pub type_: Option<V1Attr>,
    #[serde(flatten)]
    pub fields: HashMap<String, V1Source>,
}

impl V1Sources {
    pub fn builder() -> V1SourcesBuilder {
        V1SourcesBuilder::default()
    }
}

#[derive(Default)]
pub struct V1SourcesBuilder {
    type_: Option<V1Attr>,
    pub fields: HashMap<String, V1Source>,
}

impl V1SourcesBuilder {
    pub fn t(mut self, type_: V1Attr) -> V1SourcesBuilder {
        self.type_ = Some(type_);
        self
    }
    pub fn add_field(mut self, key: String, value: V1Source) -> V1SourcesBuilder {
        self.fields.insert(key, value);
        self
    }
    pub fn build(self) -> V1Sources {
        V1Sources {
            type_: self.type_,
            fields: self.fields,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct V1Source {
    pub label: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(flatten)]
    pub properties: HashMap<String, V1SourceProperty>,
}

impl V1Source {
    pub fn builder() -> V1SourceBuilder {
        V1SourceBuilder::default()
    }
}

#[derive(Default)]
pub struct V1SourceBuilder {
    label: Option<String>,
    type_: Option<String>,
    properties: HashMap<String, V1SourceProperty>,
}

impl V1SourceBuilder {
    pub fn label(mut self, label: String) -> V1SourceBuilder {
        self.label = Some(label);
        self
    }
    pub fn type_(mut self, type_: String) -> V1SourceBuilder {
        self.type_ = Some(type_);
        self
    }
    pub fn add_property(mut self, key: String, property: V1SourceProperty) -> V1SourceBuilder {
        self.properties.insert(key, property);
        self
    }
    pub fn build(self) -> V1Source {
        V1Source {
            label: self.label,
            type_: self.type_,
            properties: self.properties,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct V1SourceProperty {
    // pub ais: V1SourceAIS,
    // pub n2k: V1SourceN2K,
    pub talker: Option<String>,
    pub sentences: Option<HashMap<String, String>>,
    // TODO: Not optional?
    #[serde(flatten)]
    pub extras: HashMap<String, String>,
}

impl V1SourceProperty {
    pub fn builder() -> V1SourcePropertyBuilder {
        V1SourcePropertyBuilder::default()
    }
}

#[derive(Default)]
pub struct V1SourcePropertyBuilder {
    talker: Option<String>,
    sentences: Option<HashMap<String, String>>,
    extras: HashMap<String, String>,
}

impl V1SourcePropertyBuilder {
    pub fn talker(mut self, value: String) -> V1SourcePropertyBuilder {
        self.talker = Some(value);
        self
    }
    pub fn add_sentence(mut self, key: String, sentence: String) -> V1SourcePropertyBuilder {
        if self.sentences.is_none() {
            self.sentences = Some(HashMap::new());
        }
        if let Some(ref mut x) = self.sentences {
            x.insert(key, sentence);
        }
        self
    }
    pub fn add_extra(mut self, key: String, value: String) -> V1SourcePropertyBuilder {
        self.extras.insert(key, value);
        self
    }
    pub fn build(self) -> V1SourceProperty {
        V1SourceProperty {
            talker: self.talker,
            sentences: self.sentences,
            extras: self.extras,
        }
    }
}
