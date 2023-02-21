use crate::{empty_string_array, EndpointStats, EndpointUrl, Label, Alias};
use serde::{Deserialize, Serialize};

/// A Substrate RPC Endpoint
#[allow(clippy::derived_hash_with_manual_eq)]
#[derive(Debug, Hash, Deserialize, Serialize, Clone)]
pub struct Endpoint {
	/// Name of the endpoint
	pub name: String,

	/// Optional labels
	#[serde(default = "empty_string_array")]
	pub labels: Vec<Label>,

	/// Optional aliases
	#[serde(default = "empty_string_array")]
	pub aliases: Vec<Alias>,

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
	pub fn new(name: &str, url: &str, labels: Vec<Label>, aliases: Vec<Alias>) -> Self {
		Self {
			name: name.to_string(),
			url: EndpointUrl::try_from(url).unwrap(),
			labels,
			aliases,
			stats: EndpointStats::default(),
		}
	}

	/// Usually used to append the registry labels to an endpoint
	pub fn append_labels(&mut self, mut labels: Vec<Label>) {
		self.labels.append(&mut labels)
	}
}
