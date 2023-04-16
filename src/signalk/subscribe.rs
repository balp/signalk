use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct V1Subscribe {
    pub context: String,
    #[serde(rename = "websocket.connectionkey")]
    pub websocket_connectionkey: Option<String>,
    #[serde(rename = "reply-to")]
    pub reply_to: Option<String>,
    pub subscribe: Vec<V1Subscription>,
}

impl V1Subscribe {
    pub fn builder() -> V1SubscribeBuilder {
        V1SubscribeBuilder::default()
    }
}

#[derive(Default)]
pub struct V1SubscribeBuilder {
    context: String,
    websocket_connectionkey: Option<String>,
    reply_to: Option<String>,
    subscribe: Vec<V1Subscription>,
}

impl V1SubscribeBuilder {
    pub fn context(mut self, value: String) -> V1SubscribeBuilder {
        self.context = value;
        self
    }
    pub fn websocket_connectionkey(mut self, value: String) -> V1SubscribeBuilder {
        self.websocket_connectionkey = Some(value);
        self
    }
    pub fn reply_to(mut self, value: String) -> V1SubscribeBuilder {
        self.reply_to = Some(value);
        self
    }
    pub fn subscribe(mut self, value: V1Subscription) -> V1SubscribeBuilder {
        self.subscribe.push(value);
        self
    }

    pub fn build(self) -> V1Subscribe {
        V1Subscribe {
            context: self.context,
            websocket_connectionkey: self.websocket_connectionkey,
            reply_to: self.reply_to,
            subscribe: self.subscribe,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct V1Subscription {
    pub path: Option<String>,
    pub period: Option<i64>,
    pub format: Option<String>,
    pub policy: Option<String>,
    pub min_period: Option<i64>,
}

impl V1Subscription {
    pub fn builder() -> V1SubscriptionBuilder {
        V1SubscriptionBuilder::default()
    }
}

#[derive(Default)]
pub struct V1SubscriptionBuilder {
    path: Option<String>,
    period: Option<i64>,
    format: Option<String>,
    policy: Option<String>,
    min_period: Option<i64>,
}

impl V1SubscriptionBuilder {
    pub fn path(mut self, value: String) -> V1SubscriptionBuilder {
        self.path = Some(value);
        self
    }
    pub fn period(mut self, value: i64) -> V1SubscriptionBuilder {
        self.period = Some(value);
        self
    }
    pub fn format(mut self, value: String) -> V1SubscriptionBuilder {
        self.format = Some(value);
        self
    }
    pub fn policy(mut self, value: String) -> V1SubscriptionBuilder {
        self.policy = Some(value);
        self
    }
    pub fn min_period(mut self, value: i64) -> V1SubscriptionBuilder {
        self.min_period = Some(value);
        self
    }
    pub fn build(self) -> V1Subscription {
        V1Subscription {
            path: self.path,
            period: self.period,
            format: self.format,
            policy: self.policy,
            min_period: self.min_period,
        }
    }
}
