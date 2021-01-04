use core::result::Result;
use indicatif::ProgressIterator;
use rusqlite::{params, Connection};
use shellexpand;
use std::path::{Path, PathBuf};

use crate::{
    types::{File, StorageLocation},
    PILE_ROOT,
};

/// initialize the database. create tables if necessary
pub fn init_db(name: &str) -> Result<(), String> {
    let path = pile_path(name);
    let path = path.as_path();

    if Path::exists(path) {
        Err(format!("Pile with the name \"{}\" already exists.", name))
    } else {
        match create_db(path) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("{:?}", e)),
        }
    }
}

fn pile_path(name: &str) -> PathBuf {
    let filename = format!("{}/{}.pile", PILE_ROOT, name);
    let filename = shellexpand::tilde(&filename);
    let filename = filename.into_owned();

    PathBuf::from(filename)
}

pub fn pile_exists(name: &str) -> bool {
    Path::exists(pile_path(name).as_path())
}

fn create_db(path: &Path) -> rusqlite::Result<()> {
    let conn = Connection::open(path)?;

    // why are hashes text? because I don't want to figure out the BLOB type
    conn.execute(
        "CREATE TABLE file (
            path        TEXT PRIMARY KEY,
            hash        TEXT NOT NULL
        )",
        params![],
    )?;

    conn.execute(
        "CREATE TABLE backup (
            storage_location        TEXT NOT NULL,
            path                    TEXT NOT NULL,
            FOREIGN KEY (path) REFERENCES file (path)
        )",
        params![],
    )?;

    Ok(())
}

pub fn add_files(name: &str, files: &Vec<File>) -> rusqlite::Result<()> {
    let path = pile_path(name);
    let path = path.as_path();

    let conn = Connection::open(path)?;

    files.into_iter().progress().for_each(|f| {
        conn.execute(
            "INSERT INTO file (path, hash) VALUES (?1, ?2)",
            params![f.path, f.hash],
        )
        .expect(&format!("Couldn't save file {:?}", f));
    });

    Ok(())
}

pub fn generate_storage_candidates(
    space: u64,
    locations: Vec<StorageLocation>,
) -> Vec<StorageLocation> {
    locations.into_iter().map(|s| s.size)
}
