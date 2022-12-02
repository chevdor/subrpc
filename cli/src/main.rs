mod opts;

use clap::{crate_authors, crate_name, crate_version, Parser};
use env_logger::Env;
use log::*;
use opts::*;

/// Main entry point of the `subwasm` cli
fn main() -> color_eyre::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("none")).init();
    let opts: Opts = Opts::parse();

    println!("Running {} v{}", crate_name!(), crate_version!());
    println!("{}", crate_authors!(", "));

    match opts.subcmd {
        SubCommand::Init(_) => {
            debug!("Init")
        }

        SubCommand::Repo(_) => {
            debug!("Repo")
        }

        SubCommand::Update(_) => {
            debug!("Update")
        }

        SubCommand::System(_) => {
            debug!("System")
        }

        SubCommand::Endpoints(_) => {
            debug!("Endpoints")
        }

        SubCommand::Config(_) => {
            debug!("Config")
        }
    }
    Ok(())
}
