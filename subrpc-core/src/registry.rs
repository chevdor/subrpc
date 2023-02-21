use crate::{
	default_true, empty_string_array, endpoint::Endpoint, filter::Filter, ChainName, EndpointUrl, Label, RegistryUrl,
};
use anyhow::Result;
use chrono::{DateTime, Local};
use jsonrpsee::{core::client::ClientT, http_client::HttpClientBuilder, rpc_params, ws_client::WsClientBuilder};
use log::*;
use serde::{Deserialize, Serialize};
use std::{
	collections::{HashMap, HashSet},
	fmt::Display,
	fs::File,
	io::{Read, Write},
	path::PathBuf,
	time::Instant,
};
use tokio::runtime::Runtime;

#[derive(Eq, Debug, Deserialize, Serialize)]
pub struct Registry {
	/// Data won't be pulled from a disabled registry
	#[serde(default = "default_true")]
	pub enabled: bool,

	/// Name of the registry
	pub name: String,

	/// URL of the registry, there may be none for local/default registries
	pub url: Option<RegistryUrl>,

	/// Optional labels
	#[serde(default = "empty_string_array")]
	pub labels: Vec<Label>,

	/// DateTime of the last update of the data
	pub last_update: Option<DateTime<Local>>,

	/// Items of the registry
	pub rpc_endpoints: HashMap<ChainName, Vec<Endpoint>>,
}

impl PartialEq for Registry {
	fn eq(&self, other: &Self) -> bool {
		self.name == other.name
	}
}

impl Registry {
	pub fn new(name: &str, url: &str) -> Self {
		Self {
			name: name.to_string(),
			url: Some(url.to_string()),
			rpc_endpoints: HashMap::new(),
			enabled: true,
			last_update: None,
			labels: vec![],
		}
	}

	/// Fetch the information from located at the registry's url and update the registry items
	pub fn update(&mut self) -> Result<()> {
		if !self.enabled {
			warn!("Registry is disabled, skipping...");
			return Ok(());
		}

		if self.url.is_none() {
			warn!("Registry '{}' has no URL, skipping...", self.name);
			return Ok(());
		}

		// reg.items.iter().for_each(|(name, endpoints)| {
		//     debug!("   - {}", name);
		//     endpoints.iter().for_each(|e| {
		//         debug!("     - {} {}", e.name, e.url);
		//     });
		// });

		if let Some(registry_url) = &self.url {
			let reg = reqwest::blocking::get(registry_url)?.json::<Registry>()?;

			self.rpc_endpoints = reg.rpc_endpoints;
			debug!("Found {:?} items", self.rpc_endpoints.len());
		} else {
			log::warn!("No URL, skipping...");
		}

		// We attach the global registry labels to each endpoint
		self.attach_registry_labels();

		println!("self = {:#?}", self);

		Ok(())
	}

	/// Return the list of known chains
	pub fn get_chains(&self) -> HashSet<ChainName> {
		self.rpc_endpoints.keys().cloned().collect()
	}

	fn attach_registry_labels(&mut self) {
		self.rpc_endpoints
			.iter_mut()
			.for_each(|(_name, endpoints)| endpoints.iter_mut().for_each(|ep| ep.append_labels(self.labels.clone())));
	}

	/// Return a HashSet of Endpoints after appending the global registy labels and
	/// applying all the optionalfilters
	pub fn get_endpoints_filtered(&self, filters: Option<&Filter>) -> HashSet<Endpoint> {
		let mut endpoint_vec: HashSet<Endpoint> = HashSet::new();

		self.rpc_endpoints
			.iter()
			// .map(|(name, endpoints)| endpoints.iter_mut().map(|ep| ep.append_labels(self.labels)))
			// First we filter the chains out if the chain filter was defined
			.filter(|(chaine_name, _endpoints)| {
				if let Some(filter) = filters {
					let chain_filter = filter.chain.clone();

					if let Some(chain) = chain_filter {
						return chaine_name.as_str().to_ascii_lowercase() == chain.to_ascii_lowercase();
					} else {
						return true;
					}
				} else {
					true
				}
			})
			.for_each(|(_, e)| {
				let ee = e.clone();

				if let Some(filter) = filters {
					let alias = filter.alias.clone();

					let to_add = ee
						.iter()
						.filter(|ep| {
							if let Some(alias_filter) = alias.clone() {
								ep.aliases.contains(&alias_filter.to_ascii_lowercase())
							} else {
								true
							}
						})
						.cloned();
					endpoint_vec.extend(to_add);
				}
			});

		endpoint_vec
	}

	/// Ping all endpoints and refresh the stats
	pub fn refresh_stats(&mut self) {
		self.rpc_endpoints.iter_mut().for_each(|(_name, endpoints)| {
			endpoints.iter_mut().for_each(|endpoint| {
				let (success, latency) = Self::ping(endpoint).unwrap_or((false, None));
				let stats = &mut endpoint.stats;
				stats.add(success, latency)
			})
		})
	}

	/// Ping all endpoints and print the results to stdout.
	///
	/// Calling this function does NOT refresh the stats.
	pub fn ping_all(&mut self) {
		self.rpc_endpoints.iter_mut().for_each(|(_name, endpoints)| {
			endpoints.iter_mut().for_each(|endpoint| match Self::ping(endpoint) {
				Ok((success, latency)) => {
					if success {
						print!("✅ {:0.3}s", latency.unwrap_or(0f32));
					} else {
						print!("{: <8}", "❌");
					}
					println!(" - {:<20} {}", endpoint.name, endpoint.url);
				}
				Err(e) => {
					eprint!("{: <8}", "❌");
					eprintln!("{}: {e}", endpoint.url);
				}
			})
		})
	}

	pub fn ping(e: &Endpoint) -> Result<(bool, Option<f32>)> {
		debug!("pinging endpoint {} at {}", e.name, e.url);
		let rt = Runtime::new().unwrap();
		let start = Instant::now();

		let response: Result<String> = match &e.url {
			EndpointUrl::Https(url) | EndpointUrl::Http(url) => {
				debug!("Detected HTTP/S");
				let client = HttpClientBuilder::default().build(url)?;
				rt.block_on(client.request("system_chain", rpc_params![])).map_err(anyhow::Error::msg)
			}
			EndpointUrl::Wss(url) | EndpointUrl::Ws(url) => {
				debug!("Detected WS/S");
				let client = rt.block_on(WsClientBuilder::default().build(url))?;
				rt.block_on(client.request("system_chain", rpc_params![])).map_err(anyhow::Error::msg)
			}
		};
		debug!("response = {:?}", response);
		let duration = start.elapsed().as_millis() as f32 / 1000f32;
		let success = response.is_ok();
		rt.shutdown_background();
		Ok((success, Some(duration)))
	}

	pub fn save(&self, file: PathBuf) -> Result<()> {
		let json = serde_json::to_string_pretty(self)?;
		let mut fs = File::create(file)?;
		fs.write_all(json.as_bytes())?;
		Ok(())
	}

	pub fn load(file: PathBuf) -> Self {
		let mut fs = File::open(file).expect("File should be valid");
		let mut s = String::new();
		fs.read_to_string(&mut s).expect("Fail reading registry");
		serde_json::from_str(&s).expect("Format should be valid")
	}

	pub fn load_from_url(url: &str) -> Result<Self> {
		debug!("Adding registry from {url}");
		reqwest::blocking::get(url)?.json::<Registry>().map_err(anyhow::Error::msg)
	}

	pub fn default_bad() -> Self {
		let rpc_endpoints = HashMap::from([
			(
				"Polkadot".to_string(),
				vec![
					Endpoint::new("Parity", "wss://rpc.polkadot.io:443", vec!["Parity".to_string()], vec![]),
					Endpoint::new(
						"OnFinality",
						"wss://polkadot.api.onfinality.io:443/public-ws",
						vec!["OnFinality".to_string()],
						vec![],
					),
				],
			),
			(
				"Kusama".to_string(),
				vec![
					Endpoint::new("Parity", "wss://kusama-rpc.polkadot.io:443", vec!["Parity".to_string()], vec![]),
					Endpoint::new(
						"Parity Bad",
						"wss://bad-rpc.polkadot.io:443",
						vec!["Parity".to_string(), "Bad".to_string()],
						vec![],
					),
				],
			),
		]);

		Self { rpc_endpoints, ..Default::default() }
	}

	#[cfg(test)]
	pub fn add_endpoint(&mut self, chain: &str, endpoint: Endpoint) {
		let endpoints_vec = self.rpc_endpoints.get_mut(chain);
		let mut endpoint = endpoint.clone();
		endpoint.append_labels(self.labels.clone());

		if let Some(ep_vec) = endpoints_vec {
			ep_vec.push(endpoint);
		} else {
			self.rpc_endpoints.insert(chain.into(), vec![endpoint]);
		}
	}
}

impl Default for Registry {
	fn default() -> Self {
		let rpc_endpoints = HashMap::from([
			(
				"Polkadot".to_string(),
				vec![
					Endpoint::new("Parity", "wss://rpc.polkadot.io:443", vec!["Parity".to_string()], vec![]),
					Endpoint::new(
						"OnFinality",
						"wss://polkadot.api.onfinality.io:443/public-ws",
						vec!["OnFinality".to_string()],
						vec![],
					),
				],
			),
			(
				"Kusama".to_string(),
				vec![Endpoint::new("Parity", "wss://kusama-rpc.polkadot.io:443", vec!["Parity".to_string()], vec![])],
			),
		]);

		Self {
			name: "SubRPC Default".to_string(),
			url: None,
			rpc_endpoints,
			enabled: true,
			last_update: None,
			labels: vec![],
		}
	}
}

impl Display for Registry {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let _ = f.write_fmt(format_args!(
			"Registry: {} (url: {})\n",
			&self.name,
			&self.url.clone().unwrap_or("n/a".to_string())
		));

		self.rpc_endpoints.iter().for_each(|(name, endpoints)| {
			let _ = f.write_fmt(format_args!("  - {name}\n"));
			endpoints.iter().for_each(|e| {
				let _ = f.write_fmt(format_args!(
					"    - {}: {:?}\n",
					e.name,
					// e.url,
					e.stats
				));
			})
		});
		Ok(())
	}
}

#[cfg(test)]
mod test_super {
	use super::*;
	use std::{env, path::Path};

	#[test]
	fn test_default() {
		let reg1 = Registry::default();
		let json = ::serde_json::to_string_pretty(&reg1).unwrap();
		println!("json= {json}");
	}

	#[test]
	fn test_refresh_stats() {
		let mut reg1 = Registry::default();
		reg1.refresh_stats();
		println!("{}", &reg1);
		reg1.refresh_stats();
		println!("{}", &reg1);
	}

	#[test]
	fn test_ping_all() {
		let mut reg1 = Registry::default();
		reg1.ping_all();
	}

	#[test]
	fn test_ping_each() {
		let reg1 = Registry::default();
		reg1.rpc_endpoints.iter().for_each(|(_chain, endpoints)| {
			endpoints.iter().for_each(|e| {
				println!("Checking {}: {:?}", e.name, e.url);
				let (success, duration) = Registry::ping(e).unwrap();
				println!("{} => {:?} {:?}", e.name, success, duration);
				assert!(success);
			});
		});
	}

	#[test]
	fn test_save() {
		let reg1 = Registry::default();
		let tmpdir = env::temp_dir();
		let target_file = Path::new(&tmpdir).join("subrpc.json");
		println!("Saving to {target_file:?}");
		assert!(reg1.save(target_file).is_ok());
	}

	#[test]
	fn test_save_load() {
		let reg1 = Registry::default();
		let tmpdir = env::temp_dir();
		let target_file = Path::new(&tmpdir).join("subrpc.json");
		assert!(reg1.save(target_file.clone()).is_ok());
		let reg2 = Registry::load(target_file);
		assert_eq!(reg2, reg1);
	}

	#[test]
	fn test_load_from_url() {
		let test_url = "https://paritytech.github.io/polkadot_network_directory/registry.json";
		let reg = Registry::load_from_url(test_url).unwrap();
		println!("{reg:#?}");
		assert_eq!("Polkadot Network Directory", reg.name);
	}
}

#[cfg(test)]
mod test_local {
	use super::*;

	macro_rules! vec_of_strings {
		($($x:expr),*) => (vec![$($x.to_string()),*]);
	}

	/// Generate some reference test data
	/// - reg (gloab1, glob2)
	///   - chain 1
	/// 	- ep11 = one1 (foo, bar)
	/// 	- ep12 = one2 (foo, baz)
	///   - chain 2
	/// 	- ep21 = two1 (foo, bar)
	/// 	- ep22 = two2 (foo, baz)
	fn test_data() -> Registry {
		let mut reg = Registry::new("foo", "http://foo.bar");
		reg.labels.append(&mut vec_of_strings!["glob1", "glob2"]);

		reg.add_endpoint(
			"chain1",
			Endpoint::new("ep11", "http://ep11.url", vec_of_strings!["foo", "bar"], vec_of_strings!["one1"]),
		);

		reg.add_endpoint(
			"chain1",
			Endpoint::new("ep21", "http://ep21.url", vec_of_strings!["foo", "baz"], vec_of_strings!["one2"]),
		);

		reg.add_endpoint(
			"chain2",
			Endpoint::new("ep21", "http://ep21.url", vec_of_strings!["foo", "bar"], vec_of_strings!["two1"]),
		);

		reg.add_endpoint(
			"chain2",
			Endpoint::new("ep22", "http://ep22.url", vec_of_strings!["foo", "baz"], vec_of_strings!["two2"]),
		);

		println!("{reg:#?}");
		reg
	}

	#[test]
	fn test_get_chains() {
		let reg = test_data();

		let chains = reg.get_chains();
		println!("chains = {:?}", chains);
		assert_eq!(2, chains.len());
	}

	#[test]
	fn test_filter_chain_only() {
		let reg = test_data();

		// let filter = Filter::new()
		// 	.with_chain("Chain1")
		// 	.with_excludes(vec!["NO1".to_string(), "NO2".to_string()])
		// 	.with_includes(vec!["YES1".to_string(), "YES2".to_string()])
		// 	.with_ssl(true)
		// 	.with_endpoint_type(EndpointType::Http)
		// 	;

		let hits = reg.get_endpoints_filtered(Some(&Filter::new().with_chain("Chain1")));
		assert_eq!(2, hits.len())
	}

	#[test]
	fn test_filter_alias() {
		let reg = test_data();

		let hits = reg.get_endpoints_filtered(Some(&Filter::new().with_alias("one1")));
		// println!("hits = {:#?}", hits);
		assert_eq!(1, hits.len())
	}

	#[test]
	fn test_filter_glob1() {
		let reg = test_data();

		assert_eq!(
			2,
			reg.get_endpoints_filtered(Some(
				&Filter::new().with_chain("Chain1").with_includes(vec_of_strings!["glob1"])
			))
			.len()
		);

		assert_eq!(4, reg.get_endpoints_filtered(Some(&Filter::new().with_includes(vec_of_strings!["glob1"]))).len());
	}

	#[test]
	fn test_filter_exclude_2() {
		let reg = test_data();

		assert_eq!(2, reg.get_endpoints_filtered(Some(&Filter::new().with_excludes(vec_of_strings!["bar"]))).len());
		assert_eq!(0, reg.get_endpoints_filtered(Some(&Filter::new().with_excludes(vec_of_strings!["foo"]))).len());
	}
}
