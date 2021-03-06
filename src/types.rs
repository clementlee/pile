use std::path::Path;

use serde::{Deserialize, Serialize};

/// The config
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    #[serde(rename = "drive")]
    pub drives: Vec<Drive>,
}

/// A Drive is a location where files can be stored.
#[derive(Serialize, Deserialize, Debug)]
pub struct Drive {
    pub name: String,
    pub mountpoint: String,
    pub size: String,
}

impl Drive {
    pub fn is_mounted(&self) -> bool {
        Path::new(&self.mountpoint).exists()
    }
}

/// A Pile is a dataset containing files.
#[derive(Debug)]
pub struct Pile {
    pub name: String,
}

/// A File is, well, a file. Pile keeps track of its hash and other details for safety.
#[derive(Debug)]
pub struct File {
    pub path: String,
    pub hash: String,
    pub size: u64,
}
