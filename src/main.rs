mod checksum;
mod db;
mod types;

use anyhow::{Context, Error, Result};
use byte_unit::Byte;
use checksum::hash_file;
use clap::Clap;
use db::{add_files, add_pile, init_db, pile_exists};
use indicatif::ProgressIterator;
use log::{debug, error};
use std::{fs, path::Path};
use types::{Config, File, Pile};
use walkdir::{DirEntry, WalkDir};

const PILE_ROOT: &str = "~/.pile/";

/// test doc
#[derive(Clap, Debug)]
struct Opts {
    #[clap(subcommand)]
    cmd: SubCommand,

    #[clap(short, long)]
    verbose: bool,
}

#[derive(Clap, Debug)]
enum SubCommand {
    Add(AddCommand),
    Verify(VerifyCommand),
}

#[derive(Clap, Debug)]
struct AddCommand {
    /// the pile name to create
    #[clap(short, long)]
    name: String,

    /// the path you want to add
    #[clap(short, long)]
    path: String,
}

#[derive(Clap, Debug)]
struct VerifyCommand {}

fn main() -> Result<()> {
    env_logger::init();

    debug!("{}", checksum::hash_file(Path::new("Cargo.toml"))?);

    let opts = Opts::parse();
    debug!("{:?}", opts);

    // load config
    let base_path = shellexpand::tilde(&PILE_ROOT);
    let base_path = base_path.as_ref();
    let base_path = Path::new(base_path);
    fs::create_dir_all(base_path).context("Could not access pile root")?;

    let config_path = base_path.join("config.toml");
    let config = fs::read_to_string(config_path).context("Could not read config")?;
    let config: Config = toml::from_str(&config).context("Could not deserialize config")?;

    debug!("{:?}", Path::new("foo///asdf///t/w/w/e").to_str().unwrap());

    debug!("Available storage locations: {:?}", config.drives);

    init_db()?;

    match opts.cmd {
        SubCommand::Add(addcmd) => {
            if pile_exists(&addcmd.name) {
                // TODO: allow adding to existing pile (ask for user confirmation)
                error!("Pile \"{}\" already exists", &addcmd.name)
            } else {
                let pile = Pile {
                    name: addcmd.name.clone(),
                };
                add_pile(&pile)?;
            }

            // first, analyze the directory to see storage usage
            let files: Vec<DirEntry> = WalkDir::new(&addcmd.path)
                .follow_links(true)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.path().is_file())
                .collect();

            println!("Calculating file usage...");

            let files: Result<Vec<File>, Error> = files
                .into_iter()
                .progress()
                .map(|e| {
                    let path = e.path().strip_prefix(&addcmd.path)?;
                    let path = path.to_str().ok_or("Failed to decode path as string?");
                    let path = path.map_err(anyhow::Error::msg)?;
                    debug!("{}", path);
                    Ok(File {
                        path: path.to_string(),
                        hash: hash_file(e.path())?,
                        size: e.metadata()?.len(),
                    })
                })
                .collect();
            let files = files?;

            println!("Adding files to database...");
            add_files(&addcmd.name, &files).context("Unable to add all files")?;

            let total_storage: u64 = files.into_iter().map(|f| f.size).sum();
            let total_bytes = Byte::from_bytes(total_storage.into());
            println!(
                "Total file usage is {}",
                total_bytes.get_appropriate_unit(true).to_string()
            );

            // let best_locations: Vec<Drive> = config
            //     .drive
            //     .into_iter()
            //     .filter(|location| {
            //         let total_bytes = Byte::from_str(location.size.clone()).unwrap();

            //         //location.mountpoint
            //         true
            //     })
            //     .collect();
        }
        SubCommand::Verify(_verifycmd) => {
            error!("Not implemented");
        }
    }

    //db::init_db()?;

    Ok(())
}
