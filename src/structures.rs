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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Bookmark {
    pub container: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub description: Option<String>,
    pub label: String,
    //pub id: Uuid,
    pub link: Url,
    pub tags: Vec<String>,
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
    pub container: Option<Uuid>,
    pub container_type: ContainerTypes,
    pub id: Uuid,
    pub label: String,
}

impl Container {
    pub fn new(
        container: Option<Uuid>,
        container_type: ContainerTypes,
        id: Uuid,
        label: String,
    ) -> Self {
        Self {
            container,
            container_type,
            id,
            label,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ContainerTypes {
    Folder,
    Group,
}

#[derive(Serialize, Deserialize)]
pub enum Keyspace {
    Bookmarks,
    Containers,
}
/*
impl Keyspace {
    pub fn as_bookmarks(&self) -> Option<&Option<Bookmark>> {
        if let Self::Bookmarks(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_containers(&self) -> Option<&Option<Container>> {
        if let Self::Containers(v) = self {
            Some(v)
        } else {
            None
        }
    }
}*/

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
