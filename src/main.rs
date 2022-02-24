mod commands;
mod database;
pub mod structures;
mod tests;

use crate::structures::{Cli, Commands, Config, Keyspace};
use clap::Parser;
use paris::*;

fn main() {
    let cfg: Config = confy::load("tinymark").unwrap();
    confy::store("tinymark", &cfg).unwrap();
    /*
    println!("The configuration is:");
    println!("{:#?}", cfg);*/

    if cfg.tui {
        warn!("TUI is not implemented yet!");
        std::process::exit(exitcode::DATAERR);
    }

    let args = Cli::parse();
    let json: bool;

    if cfg.json {
        json = true;
    } else {
        json = args.json;
    }

    match &args.command {
        Commands::New_Folder { name } => commands::new_folder(name, json),
        Commands::Add {
            url,
            name,
            description,
            tags,
        } => commands::add_bookmark(url, name, description, tags, json, cfg.storage_location),
        Commands::Edit { url } => commands::edit_bookmark(json, url, cfg.storage_location),
        Commands::Delete { url } => {
            database::remove_entry(url, json, cfg.storage_location, Keyspace::Bookmarks)
        }
        Commands::List {} => commands::list_bookmarks(json, cfg.storage_location),
        Commands::Export { file } => {
            commands::export(file.to_path_buf(), json, cfg.storage_location)
        }
        Commands::Import { file } => {
            commands::import(file.to_path_buf(), json, cfg.storage_location)
        }
    }
}
