use anyhow::{bail, Result};
use chrono::{DateTime, Local};
use log::*;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{self, File},
    io::{Read, Write},
    ops::Deref,
    path::{Path, PathBuf},
};

use crate::{endpoint::Endpoint, ChainName, Registry};

/// This is the local data collected from the various regitries
#[derive(PartialEq, Debug, Deserialize, Serialize)]
pub struct LocalData {
    /// File where the local data are stored
    pub file: PathBuf,

    /// List of the registries where the RPC endpoints are pulled from
    pub registries: HashMap<String, Registry>,

    /// DateTime of the last update of the data
    pub last_update: Option<DateTime<Local>>,

    /// HashMap providing a list of [Endpoint]s for a given chain
    pub items: HashMap<ChainName, Vec<Endpoint>>,
}

impl LocalData {
    pub fn get_default_file() -> PathBuf {
        let home = dirs::home_dir().expect("Failed fetching home dir");
        let dir = Path::new(&home).join(".subrpc");
        let _ = fs::create_dir_all(&dir);
        dir.join("data.json")
    }

    /// Initialize a DB based on a given file.
    /// After initializing a DB, you should ensure it contains
    /// at least one registry and call the [refresh] function.
    pub fn init(file: PathBuf, force: bool) -> Result<Self> {
        if file.exists() && !force {
            bail!("File already exsits: {}", file.display());
        }

        let data = Self {
            file,
            ..Default::default()
        };

        Ok(data)
    }

    /// Load [LocalData] and deserialize data from [file].
    pub fn load(self) -> Self {
        debug!("Loading data from {}", self.file.display());
        let mut fs = File::open(self.file).expect("File should be valid");
        let mut s = String::new();
        fs.read_to_string(&mut s).expect("Failed reading data");
        serde_json::from_str(&s).expect("Format should be valid")
    }

    /// Loops through each registry, each network/chain, each endpoint
    /// and update the endpoints lists.
    pub fn refresh(mut self) -> Self {
        debug!("Refreshing registries");

        self.registries
            .iter_mut()
            .for_each(|(_registry_name, reg)| {
                debug!(" - {} - enabled: {:?}", &reg.name, &reg.enabled);
                // println!("reg = {:?}", &reg);
                let _status = reg.update();
            });

        self.last_update = Some(Local::now());
        self
    }

    /// Add a new registry. Registries are identitfied by their names, make sure the name is unique.
    pub fn add_registry(mut self, registry: Registry) -> Self {
        self.registries.insert(registry.name.clone(), registry);
        self
    }

    /// Save the current state to file
    pub fn save(self) -> Result<Self> {
        debug!("Saving data to {}", self.file.display());
        let json = serde_json::to_string_pretty(&self)?;
        let mut fs = File::create(&self.file)?;
        fs.write_all(json.as_bytes())?;
        Ok(self)
    }

    pub fn get_endpoints(&self, chain: &str) -> Option<Vec<&Endpoint>> {
        // let r = self
        //     .registries
        //     .iter()
        //     .map(|(_, reg)| {
        //         if !reg.enabled {
        //             return None;
        //         }

        //         let res = reg
        //             .items
        //             .iter()
        //             .filter(|(c, _)| c.as_str() == chain)
        //             .map(|(_, &endpoints)| {
        //                 let r: Vec<&Endpoint> = endpoints.iter().map(|e| e).collect();
        //                 r
        //             })
        //             .reduce(|accum, item| accum.append(&mut item));

        //         res
        //     })
        //     .collect();
        // r
        None
    }
}

impl Default for LocalData {
    fn default() -> Self {
        Self {
            file: Self::get_default_file(),
            registries: HashMap::new(),
            items: HashMap::new(),
            last_update: None,
        }
    }
}

#[cfg(test)]
mod test_local_data {
    use super::*;

    #[test]
    fn test_builder() {
        env_logger::init();

        let data = LocalData::init(LocalData::get_default_file(), true)
            .expect("Forced init should work")
            .save()
            .expect("Saving data should work")
            .load()
            .add_registry(Registry::new("SubRPC", "http://some-registry/data.json"))
            .add_registry(Registry::new("SubRPC Gist", "https://gist.githubusercontent.com/chevdor/a8b381911c28f6de02dde62ed1a17dec/raw/64479d971ce984e6d61010b94a2f81a4c5896d9d/data.json"))
            .refresh()
            .save().expect("Saving works");
        println!("{:#?}", data);
    }

    #[test]
    fn test_merge() {
        env_logger::init();

        let data = LocalData::init(LocalData::get_default_file(), true)
            .expect("Forced init should work")
            .add_registry(Registry::new("SubRPC Gist 1", "https://gist.githubusercontent.com/chevdor/a8b381911c28f6de02dde62ed1a17dec/raw/64479d971ce984e6d61010b94a2f81a4c5896d9d/data.json"))
            .add_registry(Registry::new("SubRPC Gist 2", "https://gist.githubusercontent.com/chevdor/a8b381911c28f6de02dde62ed1a17dec/raw/41fd7aba8ffc02e1adde4590eb145f583a0c7689/data2.json"))
            .refresh()
            .save().expect("Saving works");
        assert_eq!(2, data.registries.len());
        println!("{:#?}", data);
    }
}
