use crate::{EndpointStats, EndpointUrl};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Deserialize, Serialize)]
pub struct Endpoint {
    /// Name of the endpoint
    pub name: String,

    /// Optional labels
    pub labels: Vec<String>,

    /// Endpoint URL
    pub url: EndpointUrl,

    pub stats: EndpointStats,
}

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
