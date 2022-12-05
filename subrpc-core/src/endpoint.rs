use crate::{EndpointStats, EndpointUrl};
use serde::{Deserialize, Serialize};

#[derive(Debug, Hash, Deserialize, Serialize)]
pub struct Endpoint {
    /// Name of the endpoint
    pub name: String,

    /// Optional labels
    pub labels: Vec<String>,

    /// Endpoint URL
    pub url: EndpointUrl,

    pub stats: EndpointStats,
}

impl PartialEq for Endpoint {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.labels == other.labels && self.url == other.url
    }
}

impl Eq for Endpoint {}

impl Endpoint {
    pub fn new(name: &str, url: &str) -> Self {
        Self {
            name: name.to_string(),
            stats: EndpointStats::default(),
            labels: vec![],
            url: EndpointUrl::Wss(url.to_string()), // FIXME: wrong
        }
    }
}
