use serde::Deserialize;
use serde::Serialize;
use serde_json;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Error, Write, self};
use std::path::{PathBuf};
#[derive(Debug, Serialize, Deserialize)]
pub struct Database {
    path: PathBuf,
    data: HashMap<std::string::String, Vec<HashMap<std::string::String, std::string::String>>>,
    filename: std::string::String,

}

impl Database {
    pub  fn new(name: &str) -> io::Result<Self> {
        let mut path = dirs::home_dir().ok_or(io::Error::new(
            io::ErrorKind::NotFound,
            "Failed to get home directory",
        ))?;
        path.push(".aidadb");
        path.push("databases");
        fs::create_dir_all(&path)?;
        path.push(name.to_owned()+".aidb");
        Ok(Self { path, data: Default::default(), filename: "".to_string() })
    }

    pub fn create_database(&mut self, name: &str) -> io::Result<()> {
        let mut path = dirs::home_dir().unwrap();
        path.push(".aidadb");
        path.push("databases");
        fs::create_dir_all(&path)?;
        path.push(name.to_owned()+".aidb");
        println!("Created database in {}.", path.to_string_lossy());
        File::create(&path)?;

        Ok(())
    }

    pub fn save(&self) -> Result<(), Error> {
        let serialized = serde_json::to_string(&self.data)?;
        let mut file = File::create(&self.path)?;
        file.write_all(serialized.as_bytes())?;
        Ok(())
    }

    pub fn insert(&mut self, table: &str, row: HashMap<std::string::String, std::string::String>) -> Result<(), Error> {
        if !self.data.contains_key(table) {
            self.data.insert(table.to_string(), Vec::new());
        }
        self.data.get_mut(table).unwrap().push(row);

        self.save()?;
        Ok(())
    }

    pub fn select(&self, table: &str, condition: Option<HashMap<std::string::String, std::string::String>>) -> Result<Vec<HashMap<std::string::String, std::string::String>>, Error> {
        if let Some(condition) = condition {
            if let Some(rows) = self.data.get(table) {
                let mut result = Vec::new();
                for row in rows {
                    if condition.iter().all(|(key, value)| row.get(key) == Some(value)) {
                        result.push(row.clone());
                    }
                }
                Ok(result)
            } else {
                Ok(Vec::new())
            }
        } else {
            if let Some(rows) = self.data.get(table) {
                Ok(rows.clone())
            } else {
                Ok(Vec::new())
            }
        }
    }

    pub fn update(&mut self, table: &str, condition: HashMap<std::string::String, std::string::String>, values: HashMap<std::string::String, std::string::String>) -> Result<(), Error> {
        if let Some(rows) = self.data.get_mut(table) {
            for row in rows {
                if condition.iter().all(|(key, value)| row.get(key) == Some(value)) {
                    for (key, value) in values.iter() {
                        row.insert(key.to_string(), value.to_string());
                    }
                }
            }
            self.save()?;
        }
        Ok(())
    }

    pub fn delete(&mut self, table: &str, condition: Option<HashMap<std::string::String, std::string::String>>) -> Result<(), Error> {
        if let Some(condition) = condition {
            if let Some(rows) = self.data.get_mut(table) {
                let filtered_rows = rows
                    .iter()
                    .filter(|row| !condition.iter().all(|(key, value)| row.get(key) == Some(value)))
                    .cloned()
                    .collect();
                *rows = filtered_rows;
                self.save()?;
            }
        } else {
            self.data.remove(table);
            self.save()?;
        }
        Ok(())
    }
}

