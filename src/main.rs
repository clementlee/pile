mod checksum;
mod filetree;
mod types;

use clap::Clap;
use rusqlite::{params, Connection, Result};
use walkdir::WalkDir;

/// test doc
#[derive(Clap, Debug)]
struct Opts {
    dir: String,
}

fn main() {
    println!(
        "{}",
        checksum::hash_file(std::path::Path::new("Cargo.toml")).expect("asdf")
    );

    let opts = Opts::parse();

    println!("{:?}", opts);

    for entry in WalkDir::new(opts.dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
    {
        let hash = checksum::hash_file(entry.path()).expect("blah");
        println!("{} {}", entry.path().display(), hash);
    }

    test_sql().expect("asdf");
}

fn test_sql() -> Result<()> {
    let conn = Connection::open_in_memory()?;
    conn.execute(
        "CREATE TABLE person (
                  id              INTEGER PRIMARY KEY
                  )",
        params![],
    )?;

    // conn.execute("INSERT INTO person (id) VALUES (?1)", params![1])?;

    Ok(())
}
