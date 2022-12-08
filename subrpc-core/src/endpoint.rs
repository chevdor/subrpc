use crate::{empty_string_array, EndpointStats, EndpointUrl};
use serde::{Deserialize, Serialize};

#[allow(clippy::derive_hash_xor_eq)]
#[derive(Debug, Hash, Deserialize, Serialize, Clone)]
pub struct Endpoint {
	/// Name of the endpoint
	pub name: String,

	/// Optional labels
	#[serde(default = "empty_string_array")]
	pub labels: Vec<String>,

	/// Endpoint URL
	pub url: EndpointUrl,

	#[serde(skip_deserializing)]
	pub stats: EndpointStats,
}

impl PartialEq for Endpoint {
	fn eq(&self, other: &Self) -> bool {
		self.name == other.name && self.labels == other.labels && self.url == other.url
	}
}

impl Eq for Endpoint {}

impl Endpoint {
	pub fn new(name: &str, url: &str, labels: Vec<String>) -> Self {
		Self {
			name: name.to_string(),
			url: EndpointUrl::Wss(url.to_string()), // FIXME: wrong
			labels,
			stats: EndpointStats::default(),
		}
	}
}
