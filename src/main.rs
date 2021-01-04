mod checksum;
mod db;
mod types;

use byte_unit::Byte;
use checksum::hash_file;
use clap::Clap;
use db::{add_files, init_db, pile_exists};
use indicatif::ProgressIterator;
use std::{fs, path::Path};
use types::{File, StorageLocation};
use walkdir::{DirEntry, WalkDir};

const PILE_ROOT: &str = "~/.config/pile/";

/// test doc
#[derive(Clap, Debug)]
struct Opts {
    #[clap(subcommand)]
    cmd: SubCommand,
}

#[derive(Clap, Debug)]
enum SubCommand {
    New(NewCommand),
    Add(AddCommand),
    Verify(VerifyCommand),
}

#[derive(Clap, Debug)]
struct NewCommand {
    #[clap(short, long)]
    name: String,
}

#[derive(Clap, Debug)]
struct AddCommand {
    /// the pile to add to
    #[clap(short, long)]
    pile: String,

    /// the path you want to add
    #[clap(short, long)]
    path: String,
}

#[derive(Clap, Debug)]
struct VerifyCommand {}

fn main() {
    println!(
        "{}",
        checksum::hash_file(std::path::Path::new("Cargo.toml")).expect("asdf")
    );

    let opts = Opts::parse();

    // show the opts
    println!("{:?}", opts);

    // load config
    let base_path = shellexpand::tilde(&PILE_ROOT);
    let base_path = base_path.as_ref();
    let base_path = Path::new(base_path);
    fs::create_dir_all(base_path).expect("Could not access pile root");

    let config_path = base_path.join("config.toml");
    let config = fs::read_to_string(config_path).expect("Could not read config");
    let config: types::Config =
        toml::from_str(config.as_ref()).expect("Could not deserialize config");

    println!("Available storage locations: {:?}", config.drive);

    match opts.cmd {
        SubCommand::New(newcmd) => {
            init_db(newcmd.name.as_str()).expect("Couldn't create new pile");
        }
        SubCommand::Add(addcmd) => {
            if !pile_exists(&addcmd.pile) {
                panic!("Pile \"{}\" does not exist", addcmd.pile)
            }

            // first, analyze the directory to see storage usage
            let files: Vec<DirEntry> = WalkDir::new(addcmd.path)
                .follow_links(true)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.path().is_file())
                .collect();

            println!("Calculating file usage...");

            let files: Vec<File> = files
                .into_iter()
                .progress()
                .map(|e| File {
                    path: String::from(e.path().to_str().expect("huh?")),
                    hash: hash_file(e.path()).expect("huh?"),
                    size: e.metadata().expect("huh?").len(),
                })
                .collect();

            println!("Adding files to database...");
            add_files(&addcmd.pile, &files).expect("Unable to add all files");

            let total_storage: u64 = files.into_iter().map(|f| f.size).sum();
            println!("Total file usage is {} bytes", total_storage);

            let best_locations: Vec<StorageLocation> = config
                .drive
                .into_iter()
                .filter(|location| {
                    let total_bytes = Byte::from_str(location.size.clone()).unwrap();

                    //location.mountpoint
                    true
                })
                .collect();
        }
        SubCommand::Verify(_verifycmd) => {}
    }

    //db::init_db()?;
}
