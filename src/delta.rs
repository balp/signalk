use serde::{Deserialize, Serialize};

use crate::{V1DefSource, V1Meta};

/// Root structure for Delta Signal K data
#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct V1DeltaFormat {
    pub context: Option<String>,
    pub updates: Vec<V1UpdateType>,
}

impl V1DeltaFormat {
    pub fn builder() -> V1DeltaFormatBuilder {
        V1DeltaFormatBuilder::default()
    }
}

#[derive(Default)]
pub struct V1DeltaFormatBuilder {
    pub context: Option<String>,
    pub updates: Vec<V1UpdateType>,
}

impl V1DeltaFormatBuilder {
    pub fn context(mut self, vaule: String) -> V1DeltaFormatBuilder {
        self.context = Some(vaule);
        self
    }
    pub fn add_update(mut self, value: V1UpdateType) -> V1DeltaFormatBuilder {
        self.updates.push(value);
        self
    }
    pub fn build(self) -> V1DeltaFormat {
        V1DeltaFormat {
            context: self.context,
            updates: self.updates,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct V1UpdateType {
    pub values: Option<Vec<V1UpdateValue>>,
    pub meta: Option<Vec<V1UpdateMeta>>,
    #[serde(rename = "$source")]
    pub ref_source: Option<String>,
    pub source: Option<V1DefSource>,
    pub timestamp: Option<String>,
}

impl V1UpdateType {
    pub fn builder() -> V1UpdateTypeBuilder {
        V1UpdateTypeBuilder::default()
    }
}

#[derive(Default)]
pub struct V1UpdateTypeBuilder {
    values: Option<Vec<V1UpdateValue>>,
    meta: Option<Vec<V1UpdateMeta>>,
    ref_source: Option<String>,
    source: Option<V1DefSource>,
    timestamp: Option<String>,
}

impl V1UpdateTypeBuilder {
    pub fn add(mut self, value: V1UpdateValue) -> V1UpdateTypeBuilder {
        if self.values.is_none() {
            self.values = Some(Vec::new());
        }
        if let Some(ref mut x) = self.values {
            x.push(value);
        }
        self
    }
    pub fn meta(mut self, value: V1UpdateMeta) -> V1UpdateTypeBuilder {
        if self.meta.is_none() {
            self.meta = Some(Vec::new());
        }
        if let Some(ref mut x) = self.meta {
            x.push(value);
        }
        self
    }
    pub fn ref_source(mut self, value: String) -> V1UpdateTypeBuilder {
        self.ref_source = Some(value);
        self
    }
    pub fn source(mut self, value: V1DefSource) -> V1UpdateTypeBuilder {
        self.source = Some(value);
        self
    }
    pub fn timestamp(mut self, value: String) -> V1UpdateTypeBuilder {
        self.timestamp = Some(value);
        self
    }
    pub fn build(self) -> V1UpdateType {
        V1UpdateType {
            values: self.values,
            meta: self.meta,
            ref_source: self.ref_source,
            source: self.source,
            timestamp: self.timestamp,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct V1UpdateValue {
    pub path: String,
    pub value: serde_json::value::Value,
}

impl V1UpdateValue {
    pub fn new(path: String, value: serde_json::value::Value) -> Self {
        Self { path, value }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(untagged)]
pub enum V1UpdateValueType {
    String(String),
    Number(f64),
    Bool(bool),
    Object(serde_json::value::Value),
    #[default]
    None,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct V1UpdateMeta {
    pub path: String,
    pub value: V1Meta,
}

impl V1UpdateMeta {
    pub fn new(path: String, value: V1Meta) -> Self {
        Self { path, value }
    }
}
