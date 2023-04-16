use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct V1Discovery {
    pub endpoints: HashMap<String, V1DiscoveryEndpoint>,
    pub server: V1DiscoveryServer,
}

impl V1Discovery {
    pub fn builder() -> V1DiscoveryBuilder {
        V1DiscoveryBuilder::default()
    }
}

#[derive(Default)]
pub struct V1DiscoveryBuilder {
    endpoints: HashMap<String, V1DiscoveryEndpoint>,
    server: V1DiscoveryServer,
}

impl V1DiscoveryBuilder {
    pub fn server(mut self, value: V1DiscoveryServer) -> V1DiscoveryBuilder {
        self.server = value;
        self
    }
    pub fn endpoint(mut self, key: String, value: V1DiscoveryEndpoint) -> V1DiscoveryBuilder {
        self.endpoints.insert(key, value);
        self
    }
    pub fn build(self) -> V1Discovery {
        V1Discovery {
            endpoints: self.endpoints,
            server: self.server,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct V1DiscoveryEndpoint {
    pub version: String, // TODO: Version type?
    #[serde(rename = "signalk-http")]
    pub signalk_http: Option<String>, // TODO: URL type
    #[serde(rename = "signalk-ws")]
    pub signalk_ws: Option<String>, // TODO: URL type
    #[serde(rename = "signalk-tcp")]
    pub signalk_tcp: Option<String>, // TODO: URL type
}

impl V1DiscoveryEndpoint {
    pub fn builder() -> V1DiscoveryEndpointBuilder {
        V1DiscoveryEndpointBuilder::default()
    }
}

#[derive(Default)]
pub struct V1DiscoveryEndpointBuilder {
    version: String,
    signalk_http: Option<String>,
    signalk_ws: Option<String>,
    signalk_tcp: Option<String>,
}

impl V1DiscoveryEndpointBuilder {
    pub fn version(mut self, value: String) -> V1DiscoveryEndpointBuilder {
        self.version = value;
        self
    }
    pub fn signalk_http(mut self, value: String) -> V1DiscoveryEndpointBuilder {
        self.signalk_http = Some(value);
        self
    }
    pub fn signalk_ws(mut self, value: String) -> V1DiscoveryEndpointBuilder {
        self.signalk_ws = Some(value);
        self
    }
    pub fn signalk_tcp(mut self, value: String) -> V1DiscoveryEndpointBuilder {
        self.signalk_tcp = Some(value);
        self
    }
    pub fn build(self) -> V1DiscoveryEndpoint {
        V1DiscoveryEndpoint {
            version: self.version,
            signalk_http: self.signalk_http,
            signalk_ws: self.signalk_ws,
            signalk_tcp: self.signalk_tcp,
        }
    }
}

/// Information about this server
#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct V1DiscoveryServer {
    /// The id of this server (signalk-server-node, iKommunicate, etc.)
    pub id: String,

    /// The version of this server (not limited to signalk versioning rules).
    pub version: String,
}

impl V1DiscoveryServer {
    pub fn new(id: String, version: String) -> Self {
        Self { id, version }
    }
}
