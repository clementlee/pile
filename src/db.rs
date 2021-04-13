use anyhow::{anyhow, Context, Result};
use rusqlite::{params, Connection};
use shellexpand;
use std::path::{Path, PathBuf};

use crate::{
    types::{File, Pile},
    PILE_ROOT,
};

/// initialize the database. create tables if necessary
pub fn init_db() -> Result<()> {
    if Path::exists(&pile_path()) {
        // TODO: verify schema or something
        Ok(())
    } else {
        create_db()
    }
}

pub fn get_pile(name: &str) -> Result<Pile> {
    let conn = get_db()?;

    let mut stmt = conn.prepare("SELECT name FROM pile WHERE name = ?1")?;
    let iter = stmt.query_map(params![name], |row| Ok(Pile { name: row.get(0)? }))?;
    let mut iter = iter.filter_map(|res| res.ok());

    // only grab the first one
    let pile = iter.next();

    match pile {
        Some(pile) => Ok(pile),
        None => Err(anyhow!("No pile of name {} found", name)),
    }
}

pub fn pile_exists(name: &str) -> bool {
    get_pile(name).is_ok()
}

pub fn add_pile(pile: &Pile) -> Result<()> {
    let conn = get_db()?;

    conn.execute("INSERT INTO pile (name) VALUES (?1)", params![pile.name])?;

    Ok(())
}

pub fn add_files(name: &str, files: &Vec<File>) -> Result<()> {
    let conn = get_db()?;

    for file in files.iter() {
        conn.execute(
            "INSERT INTO file (path, hash, pile) VALUES (?1, ?2, ?3)
                ON CONFLICT(path, pile) DO UPDATE SET 
                path = ?1,
                hash = ?2,
                pile = ?3",
            params![file.path, file.hash, name],
        )
        .context(format!("Couldn't save file {:?}", file))?;
    }

    Ok(())
}

fn get_db() -> Result<Connection> {
    Ok(Connection::open(pile_path())?)
}

fn pile_path() -> PathBuf {
    let filename = format!("{}/pile.db", PILE_ROOT);
    let filename = shellexpand::tilde(&filename);
    let filename = filename.into_owned();

    PathBuf::from(filename)
}

fn create_db() -> Result<()> {
    let conn = get_db()?;

    conn.execute(
        "CREATE TABLE pile (
            name        TEXT PRIMARY KEY
        )",
        [],
    )
    .context("Failed to create pile table")?;

    // why are hashes text? because I don't want to figure out the BLOB type
    conn.execute(
        "CREATE TABLE file (
            path        TEXT NOT NULL,
            hash        TEXT NOT NULL,
            pile        TEXT NOT NULL,
            PRIMARY KEY (path, pile),
            FOREIGN KEY (pile) REFERENCES pile (name)
        )",
        [],
    )
    .context("Failed to create file table")?;

    conn.execute(
        "CREATE TABLE backup (
            storage_location        TEXT NOT NULL,
            path                    TEXT NOT NULL,
            FOREIGN KEY (path) REFERENCES file (path)
        )",
        [],
    )
    .context("Failed to create backup table")?;

    Ok(())
}
