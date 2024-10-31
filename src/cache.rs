/*
    Caching functionality for faster performance
*/
use crate::tool::{is_utf8, todays_day};
use serde_json::json;             // macro to convert a hashmap into JSON
use serde_json::to_string_pretty; // converts a JSON object into a prettified string
use serde_json::Value;            // A way to represent an object like a number or string from JSON
use std::fs;
use std::io::Result as IOResult;
use std::path::PathBuf;
#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

/// Struct that saves the data in a file for later use
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Cache {
    pub name: String,
    pub data: Vec<PathBuf>,
    pub value: Option<Value>,
}

impl Cache {
    /// Create a new instance of Cache
    pub fn new(data: &[PathBuf]) -> Self {
        Self {
            name: String::from("info.json"),
            data: data.to_vec(),
            value: None,
        }
    }

    /// If the file does not exist, this function will create a file whose name
    /// is the string given to the .name field
    pub fn summon(&self) -> IOResult<()> {
        if !self.exists() {
            fs::write(&self.name, "{}")?;
        }
        Ok(())
    }

    /// Given the collection of paths, the function will parse the objects and place them in a
    /// file named in the `.name` field
    ///
    /// PARAMETERS
    /// ----------
    ///     &[PathBuf] _data -> The collection of paths
    pub fn save(&self, _data: &[PathBuf]) -> IOResult<()> {
        let mut data: Vec<String> = Vec::new();
        for entry in _data.iter() {
            let entry = entry.as_os_str().to_str();
            if let Some(string) = entry {
                if !is_utf8(string) {
                    continue;
                }
                data.push(string.to_string());
            }
        }
        let data: Value = json!({
            "data" : data,
            "day" : todays_day(),
            "size" : data.len()
        });
        fs::write(&self.name, to_string_pretty(&data)?)?;
        Ok(())
    }

    /// Returns true if the file for caching exists, false otherwise.
    pub fn exists(&self) -> bool {
        PathBuf::from(&self.name).exists()
    }

    /// Returns a serde_json Value type (being the cached data) wrapped in a Result enum
    pub fn retrieve(&mut self) -> IOResult<Value> {
        if !self.exists() {
            panic!("{} does not exist!", self.name);
        }
        let content: String = String::from_utf8_lossy(&fs::read(&self.name)?).to_string();
        let data: Value = serde_json::from_str(&content)?;
        self.value = Some(data.clone());
        Ok(data)
    }

    /// Checks if the file has been made in the same day
    pub fn made_today(&mut self) -> bool {
        if self.value.is_none() {
            let _ = self.retrieve();
        }
        self.value.as_ref().unwrap()["day"] == todays_day()
    }
}
