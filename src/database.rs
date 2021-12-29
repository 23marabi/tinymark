use crate::structures::Bookmark;
use crate::commands::env_err;

use paris::*;
use url::Url;
use std::env;
use serde_json::json;
use std::path::PathBuf;

fn open_database(json: bool, path: Option<PathBuf>) -> Option<sled::Db> {
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
                    tmp_path
                },
                Err(e) => {
                    env_err(json, e);
                    std::process::exit(exitcode::DATAERR);
                },
            }
        },
    };
    match sled::open(database_path) {
        Ok(database) => db = database,
        Err(error) => {
            if json {
                println!("{}", json!({
                    "status": "fail",
                    "reason": error.to_string(),
                }));
            } else {
                error!("error in opening database: {}", error);
            }
            return None;
        },
    };

    return Some(db);
}

pub fn insert_entry(entry: &Bookmark, json: bool, path: Option<PathBuf>) {
    let db = match open_database(json, path) {
        Some(database) => database,
        None => std::process::exit(exitcode::NOINPUT),
    };

    let bytes;
    match bincode::serialize(&entry) {
        Ok(result) => bytes = result,
        Err(error) => {
            if json {
                println!("{}", json!({
                    "status": "fail",
                    "reason": error.to_string(),
                }));
            } else {
                error!("failed serializing entry: {}", error);
            }
            std::process::exit(exitcode::DATAERR);
        },
    };

    match db.insert(entry.link.to_string(), bytes) {
        Ok(_) => {
            if json {
                println!("{}", json!({
                    "status": "success",
                    "reason": "inserted entry",
                }));
            } else {
                info!("succesfully inserted entry <i>{}", entry.link);
            }
        },
        Err(error) => {
            if json {
                println!("{}", json!({
                    "status": "fail",
                    "reason": error.to_string(),
                }));
            } else {
                error!("failed to insert entry <i>{}</i>!\n {}", entry.link, error);
            }
            std::process::exit(exitcode::IOERR);
        },
    }

    db.flush().unwrap();
}

pub fn remove_entry(link: &Url, json: bool, path: Option<PathBuf>) {
    let db = match open_database(json, path) {
        Some(database) => database,
        None => std::process::exit(exitcode::NOINPUT),
    };

    match db.remove(link.to_string()) {
        Ok(_) => info!("succesfully removed entry <i>{}", link),
        Err(error) => {
            if json {
                println!("{}", json!({
                    "status": "fail",
                    "reason": error.to_string(),
                }));
            } else {
                error!("failed to remove entry <i>{}</i>!\n {}", link, error);
            }
            std::process::exit(exitcode::IOERR);
        },
    }

    db.flush().unwrap();
}

pub fn get_all(json: bool, path: Option<PathBuf>) -> Option<Vec<Bookmark>> {
    let db = match open_database(json, path) {
        Some(database) => database,
        None => return None,
    };

    let first_key = match db.first() {
        Ok(pair) => {
            pair.unwrap().0
        },
        Err(error) => {
            if json {
                println!("{}", json!({
                    "status": "fail",
                    "reason": error.to_string(),
                }));
            } else {
                error!("failed to get first key: {}", error);
            }
            return None;
        },
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
                            println!("{}", json!({
                                "status": "fail",
                                "reason": error.to_string(),
                            }));
                        } else {
                            error!("failed deserializing entry: {}", error);
                        }
                        return None;
                    },
                }
                bookmarks_vector.push(read_entry);
            },
            None => { break },
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
