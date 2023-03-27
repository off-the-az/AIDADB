mod database;


use database::Database;
use std::collections::HashMap;
use std::io::{self, Write};
use std::ptr::{null, null_mut};

fn main() {
    let mut db = None;

    loop {
        let mut input = String::new();
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "exit" {
            break;
        }

        let mut parts = input.split_whitespace();
        let command = parts.next().unwrap();

        match command {
            "create" => {
                let db_name = parts.next().unwrap();
                let db_file_path = get_database_file_path(db_name);
                if db_file_path.exists() {
                    println!("Database already exists");
                } else {
                    let new_db = Database::new(db_file_path.to_str().unwrap()).unwrap();
                    println!("Database created successfully");
                    db = Some(new_db);
                }
            }
            "use" => {
                let db_name = parts.next().unwrap();
                let db_file_path = get_database_file_path(db_name);
                if db_file_path.exists() {
                    db = Some(select_database_by_name(db_name));
                    println!("Switched to database '{}'", db_name);
                    println!("Current database path: '{}'", db_file_path.to_string_lossy())
                } else {
                    println!("Database does not exist");
                }
            }
            "insert" => {
                if let Some(ref mut db) = db {
                    let table = parts.next().unwrap();
                    let mut row = HashMap::new();
                    for part in parts {
                        let mut split = part.split('=');
                        let key = split.next().unwrap();
                        let value = split.next().unwrap();
                        row.insert(key.to_string(), value.to_string());
                    }
                    if let Err(err) = db.insert(table, row) {
                        println!("Error inserting row: {}", err);
                    } else {
                        println!("Inserted row into table {}.", table);
                    }
                } else {
                    println!("No database selected");
                }
            }
            "select" => {
                if let Some(ref db) = db {
                    let table = parts.next().unwrap();
                    let condition = if let Some(where_index) = input.find("where") {
                        let where_clause = &input[where_index + 6..];
                        let mut where_parts = where_clause.split('=');
                        let key = where_parts.next().unwrap();
                        let value = where_parts.next().unwrap();
                        Some([(key.to_string(), value.to_string())].iter().cloned().collect())
                    } else {
                        None
                    };
                    match db.select(table, condition) {
                        Ok(rows) => {
                            if rows.is_empty() {
                                println!("No rows found in table {}.", table);
                            } else {
                                for row in rows {
                                    for (key, value) in row.iter() {
                                        print!("{}={}, ", key, value);
                                    }
                                    println!("");
                                }
                            }
                        }
                        Err(err) => println!("Error selecting rows: {}", err),
                    }
                } else {
                    println!("No database selected");
                }
            }
            _ => {
                println!("Invalid command.");
            }
        }
    }
}

fn select_database_by_name(name: &str) -> Database {
    let file_path = get_database_file_path(name);
    if !file_path.exists() {
        panic!("Database {} does not exist.", name);
    }
    Database::new(&name).unwrap()
}

fn get_database_file_path(name: &str) -> std::path::PathBuf {
    let mut path = match dirs::home_dir() {
        Some(home_dir) => home_dir,
        None => panic!("Failed to get home directory path."),
    };
    path.push(".aidadb");
    path.push("databases");
    std::fs::create_dir_all(&path).unwrap();
    path.push(name.to_owned()+".aidb");
    path
}