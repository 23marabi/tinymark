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

    match &args.command {
        Commands::Add { url, name, description, tags } => commands::add(url, name, description, tags, json, cfg.storage_location),
        Commands::Edit { url } => commands::edit(json, url, cfg.storage_location),
        Commands::Delete { url } => database::remove_entry(url, json, cfg.storage_location),
        Commands::List { } => commands::list(json, cfg.storage_location),
        Commands::Export { file } => commands::export(file.to_path_buf(), json, cfg.storage_location),
        Commands::Import { file } => commands::import(file.to_path_buf(), json, cfg.storage_location),
    }
}
