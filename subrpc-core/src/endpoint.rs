use crate::{empty_string_array, EndpointStats, EndpointUrl};
use serde::{Deserialize, Serialize};

#[allow(clippy::derived_hash_with_manual_eq)]
#[derive(Debug, Hash, Deserialize, Serialize, Clone)]
pub struct Endpoint {
	/// Name of the endpoint
	pub name: String,

	/// Optional labels
	#[serde(default = "empty_string_array")]
	pub labels: Vec<String>,

	/// Endpoint URL
	pub url: EndpointUrl,

	#[serde(default)]
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
			url: EndpointUrl::try_from(url).unwrap(),
			labels,
			stats: EndpointStats::default(),
		}
	}
}
