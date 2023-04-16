use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct V1Unsubscribe {
    pub context: String,
    #[serde(rename = "websocket.connectionkey")]
    pub websocket_connectionkey: Option<String>,
    #[serde(rename = "reply-to")]
    pub reply_to: Option<String>,
    pub unsubscribe: Vec<V1Unsubscription>,
}

impl V1Unsubscribe {
    pub fn builder() -> V1UnsubscribeBuilder {
        V1UnsubscribeBuilder::default()
    }
}

#[derive(Default)]
pub struct V1UnsubscribeBuilder {
    context: String,
    websocket_connectionkey: Option<String>,
    reply_to: Option<String>,
    unsubscribe: Vec<V1Unsubscription>,
}

impl V1UnsubscribeBuilder {
    pub fn context(mut self, value: String) -> V1UnsubscribeBuilder {
        self.context = value;
        self
    }
    pub fn websocket_connectionkey(mut self, value: String) -> V1UnsubscribeBuilder {
        self.websocket_connectionkey = Some(value);
        self
    }
    pub fn reply_to(mut self, value: String) -> V1UnsubscribeBuilder {
        self.reply_to = Some(value);
        self
    }
    pub fn unsubscribe(mut self, value: V1Unsubscription) -> V1UnsubscribeBuilder {
        self.unsubscribe.push(value);
        self
    }

    pub fn build(self) -> V1Unsubscribe {
        V1Unsubscribe {
            context: self.context,
            websocket_connectionkey: self.websocket_connectionkey,
            reply_to: self.reply_to,
            unsubscribe: self.unsubscribe,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct V1Unsubscription {
    pub path: Option<String>,
    pub period: Option<i64>,
    pub format: Option<String>,
    pub policy: Option<String>,
    pub min_period: Option<i64>,
}

impl V1Unsubscription {
    pub fn builder() -> V1UnsubscriptionBuilder {
        V1UnsubscriptionBuilder::default()
    }
}

#[derive(Default)]
pub struct V1UnsubscriptionBuilder {
    path: Option<String>,
    period: Option<i64>,
    format: Option<String>,
    policy: Option<String>,
    min_period: Option<i64>,
}

impl V1UnsubscriptionBuilder {
    pub fn path(mut self, value: String) -> V1UnsubscriptionBuilder {
        self.path = Some(value);
        self
    }
    pub fn period(mut self, value: i64) -> V1UnsubscriptionBuilder {
        self.period = Some(value);
        self
    }
    pub fn format(mut self, value: String) -> V1UnsubscriptionBuilder {
        self.format = Some(value);
        self
    }
    pub fn policy(mut self, value: String) -> V1UnsubscriptionBuilder {
        self.policy = Some(value);
        self
    }
    pub fn min_period(mut self, value: i64) -> V1UnsubscriptionBuilder {
        self.min_period = Some(value);
        self
    }
    pub fn build(self) -> V1Unsubscription {
        V1Unsubscription {
            path: self.path,
            period: self.period,
            format: self.format,
            policy: self.policy,
            min_period: self.min_period,
        }
    }
}
