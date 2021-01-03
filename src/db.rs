use core::result::Result;
use rusqlite::{params, Connection};
use shellexpand;
use std::path::Path;

use crate::PILE_ROOT;

/// initialize the database. create tables if necessary
pub fn init_db(name: &str) -> Result<(), String> {
    let filename = format!("{}/{}.pile", PILE_ROOT, name);
    let filename = shellexpand::tilde(&filename);
    let filename = filename.as_ref();
    let path = Path::new(filename);

    if Path::exists(path) {
        Err(format!("Pile with the name \"{}\" already exists.", name))
    } else {
        match create_db(path) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("{:?}", e)),
        }
    }
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
        "
        ",
        params![],
    )?;

    Ok(())
}
