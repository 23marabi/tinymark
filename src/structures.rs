use chrono::prelude::*;
use clap::{AppSettings, Parser, Subcommand};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::path::PathBuf;
use url::Url;
use uuid::Uuid;
use uuid_simd::UuidExt;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub tui: bool,
    pub json: bool,
    pub storage_location: Option<PathBuf>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            tui: false,
            json: false,
            storage_location: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Bookmark {
    //pub id: Uuid,
    pub link: Url,
    pub label: String,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub container: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

fn do_nothing() {
    //yea
}

impl fmt::Display for Bookmark {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        println!("Bookmark: {}", &self.label);
        println!("{}", &self.link);

        match &self.description {
            Some(s) => println!("{}", s),
            None => do_nothing(),
        };

        print!("Tags: [");
        for i in &self.tags {
            print!("{},", i);
        }
        print!("]");

        write!(
            f,
            "\nCreated at: {}\n",
            &self.created_at.with_timezone(&Local).to_rfc2822()
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Container {
    pub id: Uuid,
    pub label: String,
    pub container: Option<Uuid>,
    pub container_type: ContainerTypes,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ContainerTypes {
    Folder,
    Group,
}

#[derive(Serialize, Deserialize)]
pub enum Thingy {
    Bookmark(Bookmark),
    Container(Container),
}

pub struct Heirarchy {
    pub root: Container,
    pub heirarchy: Vec<Vec<Thingy>>,
}

#[derive(Parser)]
pub struct Cli {
    /// Output as JSON
    #[clap(long)]
    pub json: bool,

    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a folder
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    New_Folder { name: String },

    /// Add a bookmark
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Add {
        /// The URL to add
        url: Url,

        /// The name of the bookmark
        name: String,

        /// A short description
        description: Option<String>,

        /// Optional comma-seperated tags
        tags: Vec<String>,
    },

    /// Edit a bookmark
    Edit { url: Option<Url> },

    /// Delete a bookmark
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Delete {
        // The bookmark to delete
        url: Url,
    },

    /// List all bookmarks
    List,

    /// Export the bookmarks to a JSON file
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Export {
        /// The output file
        file: PathBuf,
    },

    /// Import bookmarks from a JSON file
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Import {
        /// The input file
        file: PathBuf,
    },
}
