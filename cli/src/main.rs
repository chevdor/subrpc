mod opts;

use std::process;

// use clap::{crate_authors, crate_name, crate_version};
use clap::Parser;
use env_logger::Env;
use log::*;
use opts::*;
use subrpc_core::*;

/// Main entry point of the `subwasm` cli
fn main() -> color_eyre::Result<()> {
	env_logger::Builder::from_env(Env::default().default_filter_or("none")).init();
	let opts: Opts = Opts::parse();

	// println!("Running {} v{}", crate_name!(), crate_version!());
	// println!("{}", crate_authors!(", "));

	let local_data_file = &LocalData::get_default_file();
	debug!("Using local data from: {}", local_data_file.display());
	let mut db = LocalData::init(local_data_file, false)
		.expect("Should be able to load local data")
		.load()
		.expect("Should load properly");

	match opts.subcmd {
		SubCommand::Init(cmd_opts) => {
			debug!("Running Init command");
			debug!("cmd_opts: {:?}", cmd_opts);
			let _db = LocalData::init(local_data_file, true);
		}

		SubCommand::Registry(cmd_opts) => {
			debug!("Running Registry command");
			debug!("cmd_opts: {:?}", cmd_opts);
			match cmd_opts.registry_subcmd {
				RegistrySubCommand::List(reg_opts) => {
					debug!("registry/list");
					debug!("reg_opts: {:?}", reg_opts);
					db.print_registries();
				}

				RegistrySubCommand::Add(reg_opts) => {
					debug!("registry/add");
					debug!("reg_opts: {:?}", reg_opts);
					let reg_maybe = Registry::load_from_url(&reg_opts.url);

					match reg_maybe {
						Ok(mut reg) => {
							reg.url = Some(reg_opts.url);
							let reg_name = reg.name.clone();
							let res = db.add_registry(reg).save();
							match res {
								Ok(local_data) => {
									println!("OK, {reg_name} has been added to your local data.");
									local_data.print_registries();
								}
								Err(e) => println!(
									"Something went wrong while adding {reg_name} to your local data: {e:?}"
								),
							}
						}
						Err(e) => println!("Error adding your registry from {}: {e:?}", &reg_opts.url),
					}
				}
				RegistrySubCommand::Update(cmd_opts) => {
					debug!("Running Update command");
					debug!("cmd_opts: {:?}", cmd_opts);

					let db = db.refresh();
					db.print_summary();

					let res = db.save();
					match res {
						Ok(_db) => {
							println!("OK");
							process::exit(0);
						}
						Err(e) => {
							eprintln!("{e}");
							process::exit(1);
						}
					}
				}
				// RegistrySubCommand::Enable(reg_opts) => {
				//     debug!("registry/enable");
				//     debug!("reg_opts: {:?}", reg_opts);
				// }
				// RegistrySubCommand::Remove(_) => {
				//     debug!("registry/remove");

				// },
			}
		}

		// SubCommand::System(cmd_opts) => {
		// 	debug!("Running System command");
		// 	debug!("cmd_opts: {:?}", cmd_opts);
		// }
		SubCommand::Endpoints(cmd_opts) => {
			debug!("Running Endpoints command");
			debug!("cmd_opts: {:?}", cmd_opts);
			match cmd_opts.endpoints_subcmd {
				EndpointsSubCommand::Get(ep_opts) => {
					debug!("endpoints/get");
					debug!("ep_opts: {:?}", ep_opts);
					let endpoints = db.get_endpoints(Some(&ep_opts.chain));
					endpoints.iter().for_each(|e| {
						// println!("- {:<20}: {}", e.name, e.url);
						println!("{}", e.url);
					})
				}

				EndpointsSubCommand::List(ep_opts) => {
					debug!("endpoints/list");
					debug!("ep_opts: {:?}", ep_opts);
					let endpoints = db.get_endpoints(None);
					endpoints.iter().for_each(|e| {
						println!("{}", e.url);
					})
				}
				EndpointsSubCommand::Ping(ep_opts) => {
					debug!("endpoints/ping");
					debug!("ep_opts: {:?}", ep_opts);
					warn!("NOT FULLY IMPLEMENTED YET");
					db.registries.iter_mut().for_each(|(_name, reg)| reg.refresh_stats());
					warn!("NOT FULLY IMPLEMENTED YET");
				}
			}
		}

		_ => {
			println!("This command is not yet implemented.");
			println!("Do you fancy a PR ? Here is the repo: https://github.com/chevdor/subrpc");
			std::process::exit(1);
		} // SubCommand::Config(cmd_opts) => {
		  // 	debug!("Running Config command");
		  // 	debug!("cmd_opts: {:?}", cmd_opts);
		  // }
	}
	Ok(())
}
