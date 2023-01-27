mod config;
mod endpoints;
mod registry;
mod system;

pub use config::*;
pub use endpoints::*;
pub use registry::*;
pub use system::*;

use self::{config::ConfigOpts, endpoints::EndpointsOpts, registry::RegistryOpts, system::SystemOpts};
use clap::{crate_authors, crate_version, ColorChoice, Parser, Subcommand};

/// `subrpc` allows managing a set of registry providing rpc nodes.
#[derive(Debug, Parser)]
#[clap(version = crate_version!(), author = crate_authors!(), color=ColorChoice::Always)]
pub struct Opts {
	#[clap(subcommand)]
	pub subcmd: SubCommand,

	#[clap(short, long, global = true)]
	pub json: bool,
}

/// You can find all available commands below.
#[derive(Debug, Subcommand)]
pub enum SubCommand {
	#[clap(alias="reg", version = crate_version!(), author = crate_authors!())]
	Registry(RegistryOpts),

	#[clap(alias="sys", version = crate_version!(), author = crate_authors!())]
	System(SystemOpts),

	#[clap(alias = "ep", version = crate_version!(), author = crate_authors!())]
	Endpoints(EndpointsOpts),

	#[clap(alias="conf", version = crate_version!(), author = crate_authors!())]
	Config(ConfigOpts),
}
