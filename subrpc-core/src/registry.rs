use crate::{endpoint::Endpoint, EndpointUrl};
use anyhow::Result;
use jsonrpsee::{
    core::client::ClientT, http_client::HttpClientBuilder, rpc_params, ws_client::WsClientBuilder,
};
use log::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Display;
use std::time::Instant;
use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};
use tokio::runtime::Runtime;

type RegistryUrl = String; // FIXME
type ChainName = String;

#[derive(PartialEq, Debug, Deserialize, Serialize)]
pub struct Registry {
    /// Name of the registry
    pub name: String,

    /// URL of the registry, there may be none for local/default registries
    pub url: Option<RegistryUrl>,

    /// Items of the registry
    pub items: HashMap<ChainName, Vec<Endpoint>>,
}

impl Registry {
    pub fn new(name: String, url: String) -> Self {
        Self {
            name,
            url: Some(url),
            items: HashMap::new(),
        }
    }

    /// Fetch the information from located at the registry's url and update the registry items
    pub fn update(self) {
        if let Some(_url) = self.url {
            log::warn!("NOT IMPLEMENTED");
        } else {
            log::warn!("No URL, skipping...");
        }
    }

    /// Ping all endpoints and refresh the stats
    pub fn refresh_stats(&mut self) {
        self.items.iter_mut().for_each(|(_name, endpoints)| {
            endpoints.iter_mut().for_each(|endpoint| {
                let (success, latency) = Self::ping(endpoint).unwrap_or((false, None));
                let stats = &mut endpoint.stats;
                stats.add(success, latency)
            })
        })
    }

    /// Ping all endpoints and print the results to stdout
    pub fn ping_all(&mut self) {
        self.items.iter_mut().for_each(|(_name, endpoints)| {
            endpoints.iter_mut().for_each(|endpoint| {
                let (success, latency) = Self::ping(endpoint).unwrap_or((false, None));
                if success {
                    print!("✅ {:0.3}s", latency.unwrap_or(0f32));
                } else {
                    print!("{: <8}", "❌");
                }
                println!(" - {:<20} {}", endpoint.name, endpoint.url);
            })
        })
    }

    pub fn ping(e: &Endpoint) -> Result<(bool, Option<f32>)> {
        debug!("    pinging endpoint {} at {}", e.name, e.url);
        let rt = Runtime::new().unwrap();
        let start = Instant::now();

        let response: Result<String> = match &e.url {
            EndpointUrl::Http(url) | EndpointUrl::Https(url) => {
                let client = HttpClientBuilder::default().build(url)?;
                rt.block_on(client.request("system_chain", rpc_params![]))
                    .map_err(anyhow::Error::msg)
            }
            EndpointUrl::Ws(url) | EndpointUrl::Wss(url) => {
                let client = rt.block_on(WsClientBuilder::default().build(url))?;

                rt.block_on(client.request("system_chain", rpc_params![]))
                    .map_err(anyhow::Error::msg)
            }
        };
        let duration = start.elapsed().as_millis() as f32 / 1000f32;
        let success = response.is_ok();

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
}

impl Default for Registry {
    fn default() -> Self {
        let items = HashMap::from([
            (
                "Polkadot".to_string(),
                vec![
                    Endpoint::new("Parity", "wss://rpc.polkadot.io:443"),
                    Endpoint::new(
                        "OnFinality",
                        "wss://polkadot.api.onfinality.io:443/public-ws",
                    ),
                ],
            ),
            (
                "Kusama".to_string(),
                vec![
                    Endpoint::new("Parity", "wss://kusama-rpc.polkadot.io:443"),
                    Endpoint::new("Parity Bad", "wss://bad-rpc.polkadot.io:443"),
                ],
            ),
        ]);

        Self {
            name: "SubRPC Default".to_string(),
            url: None,
            items,
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

        self.items.iter().for_each(|(name, endpoints)| {
            let _ = f.write_fmt(format_args!("  - {}\n", name));
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
    use std::{env, path::Path};

    use super::*;

    #[test]
    fn test_default() {
        let reg1 = Registry::default();
        let json = ::serde_json::to_string_pretty(&reg1).unwrap();
        println!("json= {}", json);
    }

    #[test]
    fn test_refresh_stats() {
        env_logger::init();

        let mut reg1 = Registry::default();
        reg1.refresh_stats();
        println!("{}", &reg1);
        reg1.refresh_stats();
        println!("{}", &reg1);
    }

    #[test]
    fn test_ping_all() {
        env_logger::init();

        let mut reg1 = Registry::default();
        reg1.ping_all();
    }

    #[test]
    fn test_ping() {
        let reg1 = Registry::default();
        reg1.items.iter().for_each(|(_chain, endpoints)| {
            endpoints.iter().for_each(|e| {
                let (success, duration) = Registry::ping(e).unwrap();
                println!("{} => {:?} {:?}", e.name, success, duration);
                assert!(success);
            });
        });
        // let endpoint = Endpoint::new("tmp", "wss://rpc.polkadot.io:443");
    }

    #[test]
    fn test_save() {
        let reg1 = Registry::default();
        let tmpdir = env::temp_dir();
        let target_file = Path::new(&tmpdir).join("subrpc.json");
        println!("Saving to {:?}", target_file);
        assert!(reg1.save(target_file).is_ok());
    }

    #[test]
    fn test_save_load() {
        let reg1 = Registry::default();
        let tmpdir = env::temp_dir();
        let target_file = Path::new(&tmpdir).join("subrpc.json");
        assert!(reg1.save(target_file.clone()).is_ok());
        let reg2 = Registry::load(target_file.clone());
        assert_eq!(reg2, reg1);
    }
}
