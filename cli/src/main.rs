mod opts;

use clap::{crate_authors, crate_name, crate_version, Parser};
use env_logger::Env;
use log::*;
use opts::*;
use subrpc_core::*;

/// Main entry point of the `subwasm` cli
fn main() -> color_eyre::Result<()> {
	env_logger::Builder::from_env(Env::default().default_filter_or("none")).init();
	let opts: Opts = Opts::parse();

	println!("Running {} v{}", crate_name!(), crate_version!());
	println!("{}", crate_authors!(", "));

	let local_data_file = &LocalData::get_default_file();
	debug!("Using local data from: {}", local_data_file.display());
	let db = LocalData::init(local_data_file, false)
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
                                    println!("OK, {} has been added to your local data.", reg_name);
                                    local_data.print_registries();
                                },
                                Err(e) => println!("Something went wrong while adding {} to your local data: {:?}", reg_name, e),
                            }
                        },
                        Err(e) => println!("Error adding your registry from {}: {:?}", &reg_opts.url, e),
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

		// Update the list data from the registries
		SubCommand::Update(cmd_opts) => {
			debug!("Running Update command");
			debug!("cmd_opts: {:?}", cmd_opts);
		}

		// SubCommand::System(cmd_opts) => {
		// 	debug!("Running System command");
		// 	debug!("cmd_opts: {:?}", cmd_opts);
		// }
		SubCommand::Endpoints(cmd_opts) => {
			debug!("Running Endpoints command");
			debug!("cmd_opts: {:?}", cmd_opts);
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
