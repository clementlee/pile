use anyhow::Result;
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io;
use std::path::Path;

pub fn hash_file(path: &Path) -> Result<String> {
    let mut hasher = Sha256::new();

    let mut file = File::open(path)?;
    io::copy(&mut file, &mut hasher)?;
    let hash = hasher.finalize();

    let hash_str = hex::encode(hash.as_slice());

    Ok(hash_str)
}
