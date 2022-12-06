use std::fmt::Display;

use anyhow::bail;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Hash, Eq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum EndpointUrl {
	Http(String),
	Https(String),
	Ws(String),
	Wss(String),
}

impl Display for EndpointUrl {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			EndpointUrl::Http(s) | EndpointUrl::Https(s) | EndpointUrl::Ws(s) | EndpointUrl::Wss(s) => f.write_str(s),
		}
	}
}

impl TryFrom<&str> for EndpointUrl {
	type Error = anyhow::Error;

	fn try_from(s: &str) -> Result<Self, Self::Error> {
		if s.starts_with("wss://") {
			return Ok(EndpointUrl::Wss(s.to_string()));
		}

		if s.starts_with("ws://") {
			return Ok(EndpointUrl::Ws(s.to_string()));
		}

		if s.starts_with("https://") {
			return Ok(EndpointUrl::Https(s.to_string()));
		}

		if s.starts_with("http://") {
			return Ok(EndpointUrl::Http(s.to_string()));
		}

		// Err(format!("Invalid endpoint: {}", s))
		bail!("Invalid endpoint: {}", s)
	}
}

// impl Deserialize<'de> for EndpointUrl {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de> {
//         todo!()
//     }
// }

#[cfg(test)]
mod test_from {
	use super::*;

	#[test]
	fn test_from_str() {
		assert_eq!(EndpointUrl::Wss("wss://foobar".to_string()), EndpointUrl::try_from("wss://foobar").unwrap());
	}
}
