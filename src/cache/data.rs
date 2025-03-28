use crate::utils;

use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::to_string_pretty;

use std::io::Result;
use std::path::PathBuf;

/// Structure that defines the JSON data
/// when reading or writing into the JSON file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Data {
    pub day: u8,
    pub size: usize,
    pub data: Vec<PathBuf>,
}

impl Data {
    /// It's a problem writing a vector of PathBuf's,
    /// this function turnes this structure into a serde_json Value
    /// making it better to write into a file
    pub fn to_string(&self) -> Result<String> {
        let json = json!({
            "day": self.day,
            "size": self.size,
            "data": self.data.iter().map(|p| p.display().to_string()).collect::<Vec<String>>(),
        });
        Ok(to_string_pretty(&json)?)
    }
}

impl From<Vec<PathBuf>> for Data {
    fn from(data: Vec<PathBuf>) -> Self {
        Self {
            day: utils::todays_day(),
            size: data.len(),
            data,
        }
    }
}
