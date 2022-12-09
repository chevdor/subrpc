use clap::{crate_authors, crate_version, Parser, Subcommand};

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

/// Manage your registries
#[derive(Debug, Parser)]
pub struct RegistryOpts {
	#[clap(subcommand)]
	pub registry_subcmd: RegistrySubCommand,
}

/// Fetch the latest data from the registries and update the list of endpoints
#[derive(Debug, Parser)]
pub struct RegistryUpdateOpts {}

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
