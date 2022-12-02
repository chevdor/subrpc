use clap::{crate_authors, crate_version, ColorChoice, Parser, Subcommand};
// use std::path::PathBuf;

/// `subrpc` allows managing a set of registry providing rpc nodes.
#[derive(Parser)]
#[clap(version = crate_version!(), author = crate_authors!(), color=ColorChoice::Always)]
pub struct Opts {
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

/// You can find all available commands below.
#[derive(Subcommand)]
pub enum SubCommand {
    #[clap(version = crate_version!(), author = crate_authors!())]
    Init(InitOpts),

    #[clap(version = crate_version!(), author = crate_authors!())]
    Repo(RepoOpts),

    #[clap(version = crate_version!(), author = crate_authors!())]
    Update(UpdateOpts),

    #[clap(version = crate_version!(), author = crate_authors!())]
    System(SystemOpts),

    #[clap(version = crate_version!(), author = crate_authors!())]
    Endpoints(EndpointsOpts),

    #[clap(version = crate_version!(), author = crate_authors!())]
    Config(ConfigOpts),
}

/// Init subrpc, you usually won't need this.
#[derive(Parser)]
pub struct InitOpts {}

/// Init subrpc, you usually won't need this.
#[derive(Parser)]
pub struct RepoOpts {
    #[clap(subcommand)]
    pub repo_subcmd: RepoSubCommand,
}

/// You can find all available commands below.
#[derive(Subcommand)]
pub enum RepoSubCommand {
    #[clap(alias= "ls", version = crate_version!(), author = crate_authors!())]
    List(RepoListOpts),

    #[clap(version = crate_version!(), author = crate_authors!())]
    Add(RepoAddOpts),

    #[clap(alias="rm", version = crate_version!(), author = crate_authors!())]
    Remove(RepoRemoveOpts),
}

/// You can find all available commands below.
#[derive(Subcommand)]
pub enum ConfigSubCommand {
    #[clap(alias= "ls", version = crate_version!(), author = crate_authors!())]
    List(ConfigListOpts),

    #[clap(version = crate_version!(), author = crate_authors!())]
    Edit(ConfigEditOpts),
}

/// Update
#[derive(Parser)]
pub struct UpdateOpts {}

/// Endpoints
#[derive(Parser)]
pub struct EndpointsOpts {}

/// Config
#[derive(Parser)]
pub struct ConfigOpts {
    #[clap(subcommand)]
    pub repo_subcmd: ConfigSubCommand,
}

/// You can find all available commands below.
#[derive(Subcommand)]
pub enum SystemSubCommand {
    #[clap(alias= "ls", version = crate_version!(), author = crate_authors!())]
    Info(SystemInfoOpts),
}

/// Config
#[derive(Parser)]
pub struct SystemOpts {
    #[clap(subcommand)]
    pub system_subcmd: SystemSubCommand,
}

/// List repos
#[derive(Parser)]
pub struct RepoListOpts {}

/// Add repo
#[derive(Parser)]
pub struct RepoAddOpts {}

/// Remove repo
#[derive(Parser)]
pub struct RepoRemoveOpts {}

/// Remove repo
#[derive(Parser)]
pub struct ConfigListOpts {}

/// Remove repo
#[derive(Parser)]
pub struct ConfigEditOpts {}

/// Remove repo
#[derive(Parser)]
pub struct SystemInfoOpts {}
