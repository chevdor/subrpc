use clap::{crate_authors, crate_version, Parser, Subcommand};

/// You can find all available commands below.
#[derive(Debug, Subcommand)]
pub enum EndpointsSubCommand {
	#[clap(alias= "ls", version = crate_version!(), author = crate_authors!())]
	List(EndpointsListOpts),

	#[clap(version = crate_version!(), author = crate_authors!())]
	Get(EndpointsGetOpts),

	#[clap(version = crate_version!(), author = crate_authors!())]
	Ping(EndpointsPingOpts),

	#[clap(version = crate_version!(), author = crate_authors!())]
	Open(EndpointsOpenOpts),
}

/// Endpoints
#[derive(Debug, Parser)]
pub struct EndpointsOpts {
	#[clap(subcommand)]
	pub endpoints_subcmd: EndpointsSubCommand,
}

/// Show the list of all endpoints
#[derive(Debug, Parser)]
pub struct EndpointsListOpts {}

/// Ping endpoints
#[derive(Debug, Parser)]
pub struct EndpointsPingOpts {}

/// Get one or some endpoints
#[derive(Debug, Parser)]
pub struct EndpointsGetOpts {
	/// Name of the chain. Case insensitive.
	#[clap(index = 1)]
	pub chain: String,

	#[clap(long, short)]
	pub max: Option<usize>,
}

/// Pick an endpoint and open it using PolkadotJS
#[derive(Debug, Parser)]
pub struct EndpointsOpenOpts {
	/// Name of the chain. Case insensitive.
	#[clap(index = 1)]
	pub chain: String,
}
