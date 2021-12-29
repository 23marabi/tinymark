pub mod structures;
mod tests;
mod database;
mod commands;

use crate::structures::{Commands, Cli, Config};
use clap::Parser;
use paris::*;
use std::path::PathBuf;
use std::env;

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

    let storage_path: Option<PathBuf> = match &cfg.storage_location {
        Some(path) => {
            let HOME = env::var("HOME");
            match HOME {
                Ok(val) => {
                    let mut new_path = PathBuf::new();
                    new_path.push(val);
                    new_path.push(path);
                    Some(new_path)
                },
                Err(e) => {
                    commands::env_err(json, e);
                    None
                },
            }
        },
        None => None,
    };

    match &args.command {
        Commands::Add { url, name, description, tags } => commands::add(url, name, description, tags, json, storage_path),
        Commands::Edit { url } => commands::edit(json, url, storage_path),
        Commands::Delete { url } => database::remove_entry(url, json, storage_path),
        Commands::List { } => commands::list(json, storage_path),
        Commands::Export { file } => commands::export(file.to_path_buf(), json, storage_path),
        Commands::Import { file } => commands::import(file.to_path_buf(), json),
    }
}
