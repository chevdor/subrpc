use clap::{crate_authors, crate_version, Parser, Subcommand};

/// You can find all available commands below.
#[derive(Debug, Subcommand)]
pub enum SystemSubCommand {
	#[clap(alias= "ls", version = crate_version!(), author = crate_authors!())]
	Info(SystemInfoOpts),

	#[clap(version = crate_version!(), author = crate_authors!())]
	Init(SystemInitOpts),
}

/// Show general system information such as the location of relevant files
#[derive(Debug, Parser)]
pub struct SystemInfoOpts {}

/// Reset your local database.
///
/// This is done automatically as needed and you usually should not call this
/// command manually unless you know what you are doing.
#[derive(Debug, Parser)]
pub struct SystemInitOpts {}

/// System
#[derive(Debug, Parser)]
pub struct SystemOpts {
	#[clap(subcommand)]
	pub system_subcmd: SystemSubCommand,
}
