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

    #[clap(version = crate_version!(), author = crate_authors!())]
    Registry(RegistryOpts),

    #[clap(version = crate_version!(), author = crate_authors!())]
    Update(UpdateOpts),

    #[clap(version = crate_version!(), author = crate_authors!())]
    System(SystemOpts),

    #[clap(version = crate_version!(), author = crate_authors!())]
    Endpoints(EndpointsOpts),

    #[clap(version = crate_version!(), author = crate_authors!())]
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
    Add(RegistryAddOpts),

    // #[clap(version = crate_version!(), author = crate_authors!())]
    // Enable(RegistryEnableOpts),

    // #[clap(alias="rm", version = crate_version!(), author = crate_authors!())]
    // Remove(RegistryRemoveOpts),
}

/// You can find all available commands below.
#[derive(Debug, Subcommand)]
pub enum ConfigSubCommand {
    #[clap(alias= "ls", version = crate_version!(), author = crate_authors!())]
    List(ConfigListOpts),

    #[clap(version = crate_version!(), author = crate_authors!())]
    Edit(ConfigEditOpts),
}

/// Fetch the latest data from the registries and update the list of endpoints
#[derive(Debug, Parser)]
pub struct UpdateOpts {}

/// Endpoints
#[derive(Debug, Parser)]
pub struct EndpointsOpts {}

/// Config
#[derive(Debug, Parser)]
pub struct ConfigOpts {
    #[clap(subcommand)]
    pub Registry_subcmd: ConfigSubCommand,
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

/// Add a new registry. It will be enabled by default.
#[derive(Debug, Parser)]
pub struct RegistryAddOpts {
    #[clap(index = 1)]
    pub url: String,
}

/// Remove Registry
#[derive(Debug, Parser)]
pub struct RegistryRemoveOpts {}

/// Enable or disable a registry
#[derive(Debug, Parser)]
pub struct RegistryEnableOpts {
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
