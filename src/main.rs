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

    debug!("Available storage locations: {:?}", config.drives);

    init_db()?;

    match opts.cmd {
        SubCommand::Add(addcmd) => {
            if pile_exists(&addcmd.name) {
                // TODO: allow adding to existing pile (ask for user confirmation)
                info!("Pile \"{}\" already exists", &addcmd.name)
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

            info!("Calculating file usage...");

            let files: Vec<File> = files
                .into_iter()
                .map(|e| {
                    let path = e.path().strip_prefix(&addcmd.path)?;
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
            error!("Not implemented");
            panic!("actually no let's die for now");
        }
    }

    //db::init_db()?;

    Ok(())
}
