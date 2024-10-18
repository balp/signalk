use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct V1Put {
    pub request_id: String,
    pub context: Option<String>,
    pub put: OptionalArray<V1PutValue>,
}

impl V1Put {
    pub fn builder() -> V1PutBuilder {
        V1PutBuilder::default()
    }
}

#[derive(Default)]
pub struct V1PutBuilder {
    request_id: String,
    context: Option<String>,
    put: OptionalArray<V1PutValue>,
}

impl V1PutBuilder {
    pub fn request_id(mut self, value: String) -> V1PutBuilder {
        self.request_id = value;
        self
    }
    pub fn context(mut self, value: String) -> V1PutBuilder {
        self.context = Some(value);
        self
    }
    pub fn put(mut self, value: V1PutValue) -> V1PutBuilder {
        self.put = OptionalArray::Value(value);
        self
    }
    pub fn build(self) -> V1Put {
        V1Put {
            request_id: self.request_id,
            context: self.context,
            put: self.put,
        }
    }
}



#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(untagged)]
pub enum OptionalArray<T: Default> {
    Value(T),
    Vector(Vec<T>),
}

impl<T: Default> Default for OptionalArray<T> {
    fn default() -> Self {
        Self::Value(T::default())
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct V1PutValue {
    pub path: String,
    pub value: Value,
}

impl V1PutValue {
    pub fn new(path: String, value: Value) -> Self {
        Self { path, value }
    }
}
