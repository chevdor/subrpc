mod opts;

use std::process;

use clap::Parser;
use clap::{crate_authors, crate_name, crate_version};
use env_logger::Env;
use log::*;
use opts::*;
use subrpc_core::*;

/// Main entry point of the `subwasm` cli
fn main() -> color_eyre::Result<()> {
	env_logger::Builder::from_env(Env::default().default_filter_or("none")).init();
	let opts: Opts = Opts::parse();

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

				RegistrySubCommand::Show(reg_opts) => {
					debug!("registry/show");
					debug!("reg_opts: {:?}", reg_opts);
					db.print_summary();
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

		SubCommand::System(cmd_opts) => {
			debug!("Running System command");
			debug!("cmd_opts: {:?}", cmd_opts);
			match cmd_opts.system_subcmd {
				SystemSubCommand::Info(sys_opts) => {
					debug!("sys_opts: {:?}", sys_opts);

					println!("Running {} v{}", crate_name!(), crate_version!());
					println!("{}", crate_authors!(", "));

					println!("local data file: {}", local_data_file.display());
					db.print_summary();
				}
			}
		}

		SubCommand::Endpoints(cmd_opts) => {
			debug!("Running Endpoints command");
			debug!("cmd_opts: {:?}", cmd_opts);
			match cmd_opts.endpoints_subcmd {
				EndpointsSubCommand::Get(ep_opts) => {
					debug!("endpoints/get");
					debug!("ep_opts: {:?}", ep_opts);
					let endpoints = db.get_endpoints(Some(&ep_opts.chain));
					// let sorted_vec: Vec<_> = endpoints.iter().collect::<Vec<_>>().sort();
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

					db.registries.iter_mut().for_each(|(_name, reg)| {
						println!("Pinging endpoints from '{}'", reg.name);
						reg.refresh_stats();
					});

					match db.save() {
						Ok(_) => println!("OK"),
						Err(e) => eprintln!("{e}"),
					}
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
