use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::signalk::definitions::V1CommonValueFields;

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct V1Notification {
    pub value: Option<V1NotificationValue>,
    #[serde(flatten)]
    pub common_value_fields: Option<V1CommonValueFields>,
    #[serde(flatten)]
    pub childs: HashMap<String, V1Notification>,
}

impl V1Notification {
    pub fn builder() -> V1NotificationBuilder {
        V1NotificationBuilder::default()
    }
}

#[derive(Default)]
pub struct V1NotificationBuilder {
    value: Option<V1NotificationValue>,
    common_value_fields: Option<V1CommonValueFields>,
    childs: HashMap<String, V1Notification>,
}

impl V1NotificationBuilder {
    pub fn value(mut self, value: V1NotificationValue) -> V1NotificationBuilder {
        self.value = Some(value);
        self
    }
    pub fn common_value_fields(mut self, value: V1CommonValueFields) -> V1NotificationBuilder {
        self.common_value_fields = Some(value);
        self
    }
    pub fn add_child(mut self, key: String, value: V1Notification) -> V1NotificationBuilder {
        self.childs.insert(key, value);
        self
    }
    pub fn build(self) -> V1Notification {
        V1Notification {
            value: self.value,
            common_value_fields: self.common_value_fields,
            childs: self.childs,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct V1NotificationValue {
    pub method: Vec<String>,
    pub state: String,
    pub message: String,
    pub timestamp: Option<String>,
}

impl V1NotificationValue {
    pub fn builder() -> V1NotificationValueBuilder {
        V1NotificationValueBuilder::default()
    }
}

#[derive(Default)]
pub struct V1NotificationValueBuilder {
    method: Vec<String>,
    state: String,
    message: String,
    timestamp: Option<String>,
}

impl V1NotificationValueBuilder {
    pub fn method(mut self, value: String) -> V1NotificationValueBuilder {
        self.method.push(value);
        self
    }
    pub fn state(mut self, value: String) -> V1NotificationValueBuilder {
        self.state = value;
        self
    }
    pub fn message(mut self, value: String) -> V1NotificationValueBuilder {
        self.message = value;
        self
    }
    pub fn build(self) -> V1NotificationValue {
        V1NotificationValue {
            method: self.method,
            state: self.state,
            message: self.message,
            timestamp: self.timestamp,
        }
    }
}
