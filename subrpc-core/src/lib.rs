mod config;
mod endpoint;
mod endpoint_stats;
mod endpoint_url;
mod local_data;
mod registry;

pub use config::*;
pub use endpoint::*;
pub use endpoint_stats::*;
pub use endpoint_url::EndpointUrl;
pub use local_data::*;
pub use registry::*;

pub type RegistryUrl = String; // FIXME
pub type ChainName = String;

fn empty_string_array() -> Vec<String> {
	vec![]
}

fn default_true() -> bool {
	true
}
