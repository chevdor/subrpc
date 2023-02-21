use std::fmt::Display;

use crate::Label;

#[derive(Debug)]
pub enum EndpointType {
	Http,
	WebSocket,
	All,
}

impl Display for EndpointType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let s = match self {
			EndpointType::Http => "http",
			EndpointType::WebSocket => "ws",
			EndpointType::All => "http/ws",
		};
		write!(f, "{s}")
	}
}

/// This struct carries the optional data filter endpoints
#[derive(Debug)]
pub struct Filter {
	/// Allow searching for chains by name (case insensitive)
	pub chain: Option<String>,

	/// Allow searching for chains by aliases such as 'dot' for 'Polkadot'
	pub alias: Option<String>,

	/// List of labels that are required to be present
	pub includes: Option<Vec<Label>>,

	/// List of labels to exclude
	pub excludes: Option<Vec<Label>>,

	/// Whether you want ssl or not
	pub ssl: Option<bool>,

	/// Endpoint communication protocol
	pub endpoint_type: Option<EndpointType>,
}

impl Filter {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn with_chain(mut self, chain: &str) -> Self {
		self.chain = Some(chain.to_string());
		self
	}

	pub fn with_alias(mut self, alias: &str) -> Self {
		self.alias = Some(alias.to_string());
		self
	}

	pub fn with_includes(mut self, includes: Vec<Label>) -> Self {
		self.includes = Some(includes);
		self
	}

	pub fn with_excludes(mut self, excludes: Vec<Label>) -> Self {
		self.excludes = Some(excludes);
		self
	}

	/// Pass `true` for `httpS` or `wsS`
	pub fn with_ssl(mut self, ssl: bool) -> Self {
		self.ssl = Some(ssl);
		self
	}

	pub fn with_endpoint_type(mut self, endpoint_type: EndpointType) -> Self {
		self.endpoint_type = Some(endpoint_type);
		self
	}

	pub fn get_transport(&self) -> String {
		let ssl = match self.ssl {
			Some(s) if s => "s",
			Some(_) => "",
			None => "",
		};

		let t = match &self.endpoint_type {
			Some(et) => et.to_string(),
			None => "None".to_string(),
		};

		format!("{t}{ssl}")
	}
}

impl Default for Filter {
	fn default() -> Self {
		Self { chain: None, alias: None, includes: None, excludes: None, ssl: None, endpoint_type: Some(EndpointType::WebSocket) }
	}
}

impl Display for Filter {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"Chain: {c:?} - alias: {a:?} - with: {i:?} - without: {e:?} - {t}",
			c = self.chain,
			a = self.alias,
			i = self.includes,
			e = self.excludes,
			t = self.get_transport(),
		)
	}
}

#[cfg(test)]
mod test_filter {
	use super::*;

	#[test]
	fn test_builder() {
		let filter = Filter::new()
			.with_chain("Polkadot")
			.with_excludes(vec!["NO1".to_string(), "NO2".to_string()])
			.with_includes(vec!["YES1".to_string(), "YES2".to_string()])
			.with_ssl(true)
			.with_endpoint_type(EndpointType::Http);
		assert_eq!(Some("Polkadot".to_string()), filter.chain);
		println!("{:#?}", filter);
		println!("Filter: {}", filter);
	}
}
