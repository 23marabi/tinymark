use crate::database;
use crate::structures::{Bookmark, Container, ContainerTypes, Keyspace};
use chrono::Utc;
use paris::*;
use serde_json::json;
use std::env::VarError;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use url::Url;
use uuid::Uuid;
use uuid_simd::UuidExt;

use dialoguer::{console::Term, theme::ColorfulTheme, Select};

pub fn edit_bookmark(json: bool, url: &Option<Url>, path: Option<PathBuf>) {
    if json {
        println!(
            "{}",
            json!({
                "status": "fail",
                "reason": "unsupported command",
            })
        );
        std::process::exit(exitcode::DATAERR);
    } else {
        match url {
            Some(link) => {
                println!("User selected item :\n{}", link);
            }
            None => match database::get_all(json, path, Keyspace::Bookmarks) {
                Some(bookmarks) => {
                    let mut items: Vec<&Url> = Vec::new();
                    for i in &bookmarks {
                        items.push(&i.link);
                    }
                    let selection = Select::with_theme(&ColorfulTheme::default())
                        .items(&items)
                        .default(0)
                        .interact_on_opt(&Term::stderr())
                        .unwrap();

                    match selection {
                        Some(index) => {
                            println!(
                                "User selected item :\n{}",
                                bookmarks[index]
                            );
                        }
                        None => println!("User did not select anything"),
                    }
                }
                None => {
                    if json {
                        println!(
                            "{}",
                            json!({
                                "status": "fail",
                                "reason": "an error ocurred in running your command",
                            })
                        );
                    } else {
                        warn!("an error ocurred in running your command");
                    }
                }
            },
        }
    }
}

pub fn add_bookmark(
    url: &Url,
    name: &String,
    description: &Option<String>,
    tags: &Vec<String>,
    json: bool,
    path: Option<PathBuf>,
) {
    let bookmark = Bookmark {
        link: url.to_owned(),
        label: name.to_string(),
        description: description.to_owned(),
        tags: tags.to_vec(),
        container: None,
        created_at: Utc::now(),
    };

    database::insert_entry(json, path, Keyspace::Bookmarks, &bookmark);
    if json {
        println!("{}", serde_json::to_string(&bookmark).unwrap());
    } else {
        println!("Added new bookmark!");
        println!("{}", bookmark);
    };
}

pub fn list_bookmarks(json: bool, path: Option<PathBuf>) {
    match database::get_all(json, path, Keyspace::Bookmarks) {
        Some(bookmarks) => {
            for i in bookmarks {
                if json {
                    println!(
                        "{}",
                        serde_json::to_string(&i).unwrap()
                    );
                } else {
                    println!("{}", &i);
                }
            }
        }
        None => {
            if json {
                println!(
                    "{}",
                    json!({
                        "status": "fail",
                        "reason": "an error ocurred in running your command",
                    })
                );
            } else {
                warn!("an error ocurred in running your command");
            }
        }
    }
}

pub fn env_err(json: bool, e: VarError) {
    if json {
        println!(
            "{}",
            json!({
                "status": "fail",
                "reason": e.to_string(),
            })
        );
    } else {
        warn!("couldn't read $HOME environment variable: {}", e);
    }
}

pub fn export(file_path: PathBuf, json: bool, path: Option<PathBuf>) {
    let file = match File::create(&file_path) {
        Ok(f) => f,
        Err(e) => {
            if json {
                println!(
                    "{}",
                    json!({
                        "status": "fail",
                        "reason": e.to_string(),
                    })
                );
            } else {
                warn!("error opening file! {}", e);
            }
            std::process::exit(exitcode::DATAERR);
        }
    };
    let writer = BufWriter::new(file);

    match database::get_all(json, path, Keyspace::Bookmarks) {
        Some(bookmarks) => serde_json::to_writer(writer, &bookmarks).unwrap(),
        None => std::process::exit(exitcode::IOERR),
    }

    if json {
        println!(
            "{}",
            json!({
                "status": "success",
                "reason": format!("exported bookmarks to {}", file_path.to_str().unwrap()),
            })
        );
    } else {
        info!(
            "Succesfully exported bookmarks to {}!",
            file_path.to_str().unwrap()
        );
    }
}

pub fn import(file_path: PathBuf, json: bool, store_path: Option<PathBuf>) {
    let file = match File::open(&file_path) {
        Ok(f) => f,
        Err(e) => {
            if json {
                println!(
                    "{}",
                    json!({
                        "status": "fail",
                        "reason": e.to_string(),
                    })
                );
            } else {
                warn!("error opening file! {}", e);
            }
            std::process::exit(exitcode::DATAERR);
        }
    };
    let reader = BufReader::new(file);

    let bookmarks: Vec<Bookmark> = match serde_json::from_reader(reader) {
        Ok(contents) => contents,
        Err(e) => {
            if json {
                println!(
                    "{}",
                    json!({
                        "status": "fail",
                        "reason": e.to_string(),
                    })
                );
            } else {
                warn!("error serializing file! {}", e);
            }
            std::process::exit(exitcode::DATAERR);
        }
    };

    database::insert_multiple(&bookmarks, json, store_path, Keyspace::Bookmarks);

    if json {
        println!(
            "{}",
            json!({
                "status": "success",
                "reason": format!("imported bookmarks from {}", file_path.to_str().unwrap()),
            })
        );
    } else {
        info!(
            "succesfully imported bookmarks from {}!",
            file_path.to_str().unwrap()
        );
    }
}

pub fn new_folder(label: &String, json: bool) {
    let folder = Container {
        id: Uuid::new_v4(),
        label: label.to_string(),
        container: None,
        container_type: ContainerTypes::Folder,
    };
    println!("{:?}", folder);
    if json {
        println!("{}", serde_json::to_string(&folder).unwrap());
    } else {
        println!("{:?}", folder);
    }
}

pub fn create_heirarchy(bookmarks: Vec<Bookmark>, containers: Vec<Container>) {}
