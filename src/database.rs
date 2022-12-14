use crate::commands::env_err;
use crate::structures::{Bookmark, Keyspace};

use paris::*;
use serde_json::json;
use std::env;
use std::path::PathBuf;
use url::Url;

fn open_database(json: bool, path: Option<PathBuf>, keyspace: Keyspace) -> Option<sled::Tree> {
    let db: sled::Db;
    let database_path = match path {
        Some(path) => path,
        None => {
            let key = "HOME";
            match env::var(key) {
                Ok(val) => {
                    let mut tmp_path = PathBuf::new();
                    tmp_path.push(val);
                    tmp_path.push(".local/share/tinymark");
                    tmp_path.push("database");
                    tmp_path
                }
                Err(e) => {
                    env_err(json, e);
                    std::process::exit(exitcode::DATAERR);
                }
            }
        }
    };

    match sled::open(database_path) {
        Ok(database) => db = database,
        Err(error) => {
            if json {
                println!(
                    "{}",
                    json!({
                        "status": "fail",
                        "reason": error.to_string(),
                    })
                );
            } else {
                error!("error in opening database: {}", error);
            }
            return None;
        }
    };

    let keyspace_str = match keyspace {
        Keyspace::Bookmarks => "bookmarks",
        Keyspace::Containers => "containers",
    };

    let database: Option<sled::Tree> = match db.open_tree(keyspace_str) {
        Ok(nya) => Some(nya),
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
                error!("error in opening Tree: {}", e);
            }
            None
        }
    };
    return database;
}

pub fn insert_multiple(
    entries: &Vec<Bookmark>,
    json: bool,
    path: Option<PathBuf>,
    keyspace: Keyspace,
) {
    let db = match open_database(json, path, keyspace) {
        Some(database) => database,
        None => std::process::exit(exitcode::NOINPUT),
    };

    let mut batch = sled::Batch::default();

    for i in entries {
        let bytes;
        match bincode::serialize(&i) {
            Ok(result) => bytes = result,
            Err(error) => {
                if json {
                    println!(
                        "{}",
                        json!({
                            "status": "fail",
                            "reason": error.to_string(),
                        })
                    );
                } else {
                    error!("failed serializing entry: {}", error);
                }
                std::process::exit(exitcode::DATAERR);
            }
        }

        batch.insert(i.link.as_str(), bytes);
    }

    match db.apply_batch(batch) {
        Ok(_) => {
            if !json {
                info!("succesfully applied batch insert");
            }
        }
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
                warn!("error in applying batch insert: {}", e);
            }
        }
    }
}

pub fn insert_entry(json: bool, path: Option<PathBuf>, keyspace: Keyspace, _entry: &Bookmark) {
    let entry = _entry;
    let db = match open_database(json, path, keyspace) {
        Some(database) => database,
        None => std::process::exit(exitcode::NOINPUT),
    };

    let bytes;
    match bincode::serialize(&entry) {
        Ok(result) => bytes = result,
        Err(error) => {
            if json {
                println!("{}",
                    json!({
                        "status": "fail",
                        "reason": error.to_string(),
                    })
                );
            } else {
                error!("failed serializing entry: {}", error);
            }
            std::process::exit(exitcode::DATAERR);
        }
    }

    let name = &entry.link.to_string();

    match db.insert(&name, bytes) {
        Ok(_) => {
            if json {
                println!(
                    "{}",
                    json!({
                        "status": "success",
                        "reason": "inserted entry",
                    })
                );
            } else {
                info!("succesfully inserted entry <i>{}", name);
            }
        }
        Err(error) => {
            if json {
                println!(
                    "{}",
                    json!({
                        "status": "fail",
                        "reason": error.to_string(),
                    })
                );
            } else {
                error!("failed to insert entry <i>{}</i>!\n {}", name, error);
            }
            std::process::exit(exitcode::IOERR);
        }
    }

    db.flush().unwrap();
}

pub fn remove_entry(link: &Url, json: bool, path: Option<PathBuf>, keyspace: Keyspace) {
    let db = match open_database(json, path, keyspace) {
        Some(database) => database,
        None => std::process::exit(exitcode::NOINPUT),
    };

    match db.remove(link.to_string()) {
        Ok(_) => info!("succesfully removed entry <i>{}", link),
        Err(error) => {
            if json {
                println!(
                    "{}",
                    json!({
                        "status": "fail",
                        "reason": error.to_string(),
                    })
                );
            } else {
                error!("failed to remove entry <i>{}</i>!\n {}", link, error);
            }
            std::process::exit(exitcode::IOERR);
        }
    }

    db.flush().unwrap();
}

pub fn get_all(json: bool, path: Option<PathBuf>, keyspace: Keyspace) -> Option<Vec<Bookmark>> {
    let db = match open_database(json, path, keyspace) {
        Some(database) => database,
        None => return None,
    };

    let first_key = match db.first() {
        Ok(pair) => match pair {
            Some(key) => key.0,
            None => {
                if json {
                    println!(
                        "{}",
                        json!({
                        "status": "error",
                        "reason": "could not get first key",
                        })
                    );
                } else {
                    error!("error in getting first key");
                }
                std::process::exit(exitcode::IOERR);
            }
        },
        Err(error) => {
            if json {
                println!(
                    "{}",
                    json!({
                        "status": "fail",
                        "reason": error.to_string(),
                    })
                );
            } else {
                error!("failed to get first key: {}", error);
            }
            return None;
        }
    };

    let mut bookmarks_vector: Vec<Bookmark> = Vec::new();
    let mut iter = db.range(first_key..);

    loop {
        match iter.next() {
            Some(x) => {
                let read_entry;
                match bincode::deserialize(&x.unwrap().1) {
                    Ok(result) => read_entry = result,
                    Err(error) => {
                        if json {
                            println!(
                                "{}",
                                json!({
                                    "status": "fail",
                                    "reason": error.to_string(),
                                })
                            );
                        } else {
                            error!("failed deserializing entry: {}", error);
                        }
                        return None;
                    }
                }
                bookmarks_vector.push(read_entry);
            }
            None => break,
        }
    }
    return Some(bookmarks_vector);
}

/* not working for some reason
pub fn update_entry(entry: &Bookmark) {
    let db = open_database();

    let bytes = match bincode::serialize(&entry) {
        Ok(bytes) => bytes,
        Err(error) => panic!("failed to serialize entry: {}", error),
    };

    let old_entry: Bookmark = get_entry(&entry.id.to_simple().to_string()).unwrap();

    let old_bytes = match bincode::serialize(&old_entry) {
        Ok(bytes) => bytes,
        Err(error) => panic!("failed to serialize entry: {}", error),
    };

    match db.compare_and_swap(
        entry.id.to_simple().to_string(),
        Some(&old_bytes),
        Some(&bytes)) {
            Ok(_) => info!("succesfully swapped entry <i>{}", entry.id),
            Err(error) => warn!("failed to swap entry <i>{}</i>!\n {}", entry.id, error),
    }

    db.flush();
}

pub fn get_entry(id: &str) -> Option<Bookmark> {
    let db = open_database();

    let db_entry = match db.get(id) {
        Ok(entry) => entry,
        Err(error) => panic!("failed to get old entry: {}", error),
    };

    if let Some(entry) = db_entry {
        let read_entry = match bincode::deserialize(&entry) {
            Ok(entry) => entry,
            Err(error) => panic!("failed to deserialize entry: {}", error),
        };
        return Some(read_entry);
    } else {
        warn!("failed to find entry");
        return None;
    };
}
*/
