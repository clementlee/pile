mod checksum;
mod db;
mod fs_tools;
mod types;

use anyhow::{anyhow, Context, Error, Result};
use byte_unit::Byte;
use checksum::hash_file;
use clap::Clap;
use db::{add_files, add_pile, init_db, pile_exists};
use log::{debug, error, info};
use std::{
    fs,
    path::{Path, PathBuf},
};
use types::{Config, Drive, File, Pile};
use walkdir::{DirEntry, WalkDir};

const PILE_ROOT: &str = "~/.pile/";

/// test doc
#[derive(Clap, Debug)]
struct Opts {
    #[clap(subcommand)]
    cmd: SubCommand,
}

#[derive(Clap, Debug)]
enum SubCommand {
    Add(AddCommand),
    Find(FindCommand),
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
struct FindCommand {}

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

    debug!("Available storage locations: {:?}", config.drives);

    init_db()?;
    for drive in &config.drives {
        debug!(
            "BTW, usage of drive {} is {}",
            drive.name,
            db::get_usage(&drive)?
        );
    }

    match opts.cmd {
        SubCommand::Add(addcmd) => {
            if pile_exists(&addcmd.name) {
                // TODO: allow adding to existing pile (ask for user confirmation)
                error!("Pile \"{}\" already exists", &addcmd.name);

                todo!("Adding to existing pile isn't implemented yet")
            } else {
                let pile = Pile {
                    name: addcmd.name.clone(),
                };
                add_pile(&pile)?;
            }

            let files = get_files(&addcmd.path)?;

            add_files(&addcmd.name, &files).context("Unable to add all files")?;

            let total_storage: u64 = files.iter().map(|f| f.size).sum();
            let total_bytes = Byte::from_bytes(total_storage.into());
            info!(
                "Total file usage is {}",
                total_bytes.get_appropriate_unit(true).to_string()
            );

            let best_locations: Vec<Drive> = config
                .drives
                .into_iter()
                .filter_map(|drive| {
                    debug!("Checking likely space for drive {}", &drive.name);

                    let bytes = fs_tools::get_drive_capacity(&drive).ok()?;

                    Some(drive)
                    //location.mountpoint
                })
                .collect();
        }
        SubCommand::Verify(_verifycmd) => {
            todo!()
        }
        SubCommand::Find(_findcmd) => {
            todo!()
        }
    }

    //db::init_db()?;

    Ok(())
}

fn get_files(path: &str) -> Result<Vec<File>> {
    // first, analyze the directory to see storage usage
    let files: Vec<DirEntry> = WalkDir::new(&path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .collect();

    info!("Calculating file usage...");

    let files: Vec<File> = files
        .into_iter()
        .map(|e| {
            let path = e.path().strip_prefix(&path)?;
            let path = path
                .to_str()
                .ok_or(anyhow!("Failed to decode path as string?"))?;
            Ok(File {
                path: path.to_string(),
                hash: hash_file(e.path())?,
                size: e.metadata()?.len(),
            })
        })
        .collect::<Result<Vec<File>, Error>>()?;

    Ok(files)
}

fn get_drive_suggestions(bytes_required: u64, config: &Config) -> Result<Vec<&Drive>> {
    let best_locations: Vec<&Drive> = config
        .drives
        .iter()
        .filter_map(|drive| {
            debug!("Checking likely space for drive {}", &drive.name);

            let bytes = fs_tools::get_drive_capacity(&drive).ok()?;

            Some(drive)
            //location.mountpoint
        })
        .collect();

    Ok(best_locations)
}
