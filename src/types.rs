use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub drive: Vec<StorageLocation>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct StorageLocation {
    pub name: String,
    pub mountpoint: String,
    pub size: String,
}

#[derive(Debug)]
pub struct File {
    pub path: String,
    pub hash: String,
    pub size: u64,
}
