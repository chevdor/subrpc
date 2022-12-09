use clap::{crate_authors, crate_version, Parser, Subcommand};

/// You can find all available commands below.
#[derive(Debug, Subcommand)]
pub enum ConfigSubCommand {
	#[clap(alias= "ls", version = crate_version!(), author = crate_authors!())]
	List(ConfigListOpts),

	#[clap(version = crate_version!(), author = crate_authors!())]
	Edit(ConfigEditOpts),
}

/// Config
#[derive(Debug, Parser)]
pub struct ConfigOpts {
	#[clap(subcommand)]
	pub registry_subcmd: ConfigSubCommand,
}

/// Config list
#[derive(Debug, Parser)]
pub struct ConfigListOpts {}

/// Config edit
#[derive(Debug, Parser)]
pub struct ConfigEditOpts {}
