mod config;
mod endpoint;
mod endpoint_stats;
mod endpoint_url;
mod registry;
mod local_data;

pub use config::*;
pub use endpoint_stats::*;
pub use endpoint_url::EndpointUrl;
pub use endpoint_url::*;
pub use registry::*;
pub use local_data::*;

pub type RegistryUrl = String; // FIXME
pub type ChainName = String;

fn empty_string_array() -> Vec<String> {
    vec![]
}

fn default_true() -> bool {
    true
}
