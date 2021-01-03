mod checksum;
mod db;
mod types;

use clap::Clap;
use db::init_db;
use std::{fs, path::Path};

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
    #[clap(short, long)]
    pile: String,
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
        SubCommand::Add(_addcmd) => {}
        SubCommand::Verify(_verifycmd) => {}
    }

    //db::init_db()?;
}
