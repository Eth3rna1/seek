use crate::cache::Data;
use crate::utils;

//use serde::{Deserialize, Serialize};
use serde_json::json; // macro to convert a hashmap into JSON
use serde_json::to_string_pretty; // converts a JSON object into a prettified string
use serde_json::Value; // A way to represent an object like a number or string from JSON

use std::fs;
use std::io::Result;
use std::io::{Error, ErrorKind};
use std::path::Path;
use std::path::PathBuf;

/// A JSON Cache structure
#[derive(Clone, Debug)]
pub struct Cache {
    location: PathBuf,
}

impl Cache {
    /// Initializes Cache. Location must include the cache file name.
    ///
    /// Default: ./info.json
    pub fn new(location: &str) -> Self {
        Self {
            location: PathBuf::from(location),
        }
    }

    /// Returns a reference to the internal location
    pub fn location<'l>(&'l self) -> &'l Path {
        &self.location
    }

    pub fn exists(&self) -> bool {
        self.location.exists()
    }

    /// Creates the cache file if it doesn't exist
    pub fn summon(&self) -> Result<()> {
        if self.exists() {
            return Ok(());
        }
        fs::File::create(&self.location)?;
        Ok(())
    }

    /// Cache validity is defined daily
    pub fn is_valid(&self) -> bool {
        if !self.exists() {
            return false;
        }
        // if any error is propagated, automatically returns false
        if let Ok(content) = fs::read_to_string(&self.location) {
            match serde_json::from_str::<Data>(&content) {
                Ok(data) => {
                    if data.day == utils::todays_day() {
                        return true;
                    }
                    return false;
                }
                Err(_) => return false,
            }
        }
        false
    }

    pub fn read(&self) -> Result<Data> {
        let content = fs::read_to_string(&self.location)?;
        return match serde_json::from_str::<Data>(&content) {
            Ok(data) => Ok(data),
            Err(error) => Err(error.into()),
        };
    }

    pub fn write(&self, data: &Data) -> Result<()> {
        fs::write(&self.location, data.to_string()?)?;
        Ok(())
    }
}
