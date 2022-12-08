use clap::{crate_authors, crate_version, ColorChoice, Parser, Subcommand};
// use std::path::PathBuf;

/// `subrpc` allows managing a set of registry providing rpc nodes.
#[derive(Debug, Parser)]
#[clap(version = crate_version!(), author = crate_authors!(), color=ColorChoice::Always)]
pub struct Opts {
	#[clap(subcommand)]
	pub subcmd: SubCommand,
}

/// You can find all available commands below.
#[derive(Debug, Subcommand)]
pub enum SubCommand {
	#[clap(version = crate_version!(), author = crate_authors!())]
	Init(InitOpts),

	#[clap(alias="reg", version = crate_version!(), author = crate_authors!())]
	Registry(RegistryOpts),

	#[clap(alias="sys", version = crate_version!(), author = crate_authors!())]
	System(SystemOpts),

	#[clap(alias = "ep", version = crate_version!(), author = crate_authors!())]
	Endpoints(EndpointsOpts),

	#[clap(alias="conf", version = crate_version!(), author = crate_authors!())]
	Config(ConfigOpts),
}

/// Force `init` the `subrpc` local data. This is done automatically as needed and
/// you usually should not call this command manually unless you know what you are doing.
#[derive(Debug, Parser)]
pub struct InitOpts {}

/// Manage your registries
#[derive(Debug, Parser)]
pub struct RegistryOpts {
	#[clap(subcommand)]
	pub registry_subcmd: RegistrySubCommand,
}

/// You can find all available commands below.
#[derive(Debug, Subcommand)]
pub enum RegistrySubCommand {
	#[clap(alias= "ls", version = crate_version!(), author = crate_authors!())]
	List(RegistryListOpts),

	#[clap(version = crate_version!(), author = crate_authors!())]
	Show(RegistryShowOpts),

	#[clap(version = crate_version!(), author = crate_authors!())]
	Add(RegistryAddOpts),
	// #[clap(version = crate_version!(), author = crate_authors!())]
	// Enable(RegistryEnableOpts),

	// #[clap(alias="rm", version = crate_version!(), author = crate_authors!())]
	// Remove(RegistryRemoveOpts),
	#[clap(alias="up", version = crate_version!(), author = crate_authors!())]
	Update(RegistryUpdateOpts),
}

/// You can find all available commands below.
#[derive(Debug, Subcommand)]
pub enum ConfigSubCommand {
	#[clap(alias= "ls", version = crate_version!(), author = crate_authors!())]
	List(ConfigListOpts),

	#[clap(version = crate_version!(), author = crate_authors!())]
	Edit(ConfigEditOpts),
}

/// You can find all available commands below.
#[derive(Debug, Subcommand)]
pub enum EndpointsSubCommand {
	#[clap(alias= "ls", version = crate_version!(), author = crate_authors!())]
	List(EndpointsListOpts),

	#[clap(version = crate_version!(), author = crate_authors!())]
	Get(EndpointsGetOpts),

	#[clap(version = crate_version!(), author = crate_authors!())]
	Ping(EndpointsPingOpts),
}

/// Fetch the latest data from the registries and update the list of endpoints
#[derive(Debug, Parser)]
pub struct RegistryUpdateOpts {}

/// Endpoints
#[derive(Debug, Parser)]
pub struct EndpointsOpts {
	#[clap(subcommand)]
	pub endpoints_subcmd: EndpointsSubCommand,
}

/// Config
#[derive(Debug, Parser)]
pub struct ConfigOpts {
	#[clap(subcommand)]
	pub registry_subcmd: ConfigSubCommand,
}

/// You can find all available commands below.
#[derive(Debug, Subcommand)]
pub enum SystemSubCommand {
	#[clap(alias= "ls", version = crate_version!(), author = crate_authors!())]
	Info(SystemInfoOpts),
}

/// System
#[derive(Debug, Parser)]
pub struct SystemOpts {
	#[clap(subcommand)]
	pub system_subcmd: SystemSubCommand,
}

/// List currently known registries
#[derive(Debug, Parser)]
pub struct RegistryListOpts {}

/// Show the list of registries and some of the content
#[derive(Debug, Parser)]
pub struct RegistryShowOpts {}

/// Add a new registry. It will be enabled by default.
#[derive(Debug, Parser)]
pub struct RegistryAddOpts {
	/// Url of the registry. This should be pointing to a json file.
	#[clap(index = 1)]
	pub url: String,
}

/// Remove Registry
#[derive(Debug, Parser)]
pub struct RegistryRemoveOpts {}

/// Enable or disable a registry
#[derive(Debug, Parser)]
pub struct RegistryEnableOpts {
	/// true/false
	#[clap(index = 1)]
	pub state: bool,
}

/// Config list
#[derive(Debug, Parser)]
pub struct ConfigListOpts {}

/// Config edit
#[derive(Debug, Parser)]
pub struct ConfigEditOpts {}

/// System info
#[derive(Debug, Parser)]
pub struct SystemInfoOpts {}

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
}
