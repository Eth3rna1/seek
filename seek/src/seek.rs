/*
    Seeking functionality with asynchronous programming
*/
use serde_json::Value;
use std::ffi::OsStr;
use std::ops::Deref;
use std::path::PathBuf;
use tokio::task::JoinHandle;
use tqdm::tqdm;
use walkdir::WalkDir;
#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

/// Struct that contains the walking functionality
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Seek {
    hint: PathBuf,
    objects: Vec<PathBuf>,
}

impl Seek {
    /// Initiates a new instance of Seek
    pub fn new(hint: &str) -> Self {
        Self {
            hint: PathBuf::from(hint),
            objects: Vec::new(),
        }
    }

    /// Scans all directories starting from the `.hint` field name you provided
    pub async fn scan(&mut self) {
        let mut workers: Vec<JoinHandle<PathBuf>> = Vec::new();
        for entry in WalkDir::new(&self.hint) {
            if entry.is_err() {
                continue;
            }
            let worker: JoinHandle<PathBuf> = tokio::spawn(async move {
                let entry = entry.unwrap().path().to_path_buf();
                entry
            });
            workers.push(worker);
        }
        for worker in workers {
            let result = worker.await.unwrap();
            self.objects.push(result);
        }
    }

    /// Will return paths that resemble the arguments given depeding on what you give
    ///
    /// PARAMETERS
    /// ----------
    ///     &str object    -> The directory or file stem you are searching for
    ///     &str extension -> The extension of the file, if directory, pass ""
    ///     bool exact     -> true, if you're looking for an exact match, false otherwise
    pub fn search(&self, object: &str, extension: &str, exact: bool) -> Option<Vec<PathBuf>> {
        let object_lowercase = object.to_lowercase();
        let extension_lowercase = extension.to_lowercase();

        // let mut workers: Vec<JoinHandle<Option<PathBuf>>> = Vec::new();
        let mut results: Vec<PathBuf> = Vec::new();

        if exact {
            for entry in tqdm(self.objects.iter()) {
                let entry_stem = entry
                    .file_stem()
                    .unwrap_or(OsStr::new(""))
                    .to_str()
                    .unwrap_or("");
                let entry_extension = entry
                    .extension()
                    .unwrap_or(OsStr::new(""))
                    .to_str()
                    .unwrap_or("");

                if entry_stem.eq_ignore_ascii_case(&object_lowercase)
                    && entry_extension.eq_ignore_ascii_case(&extension_lowercase)
                {
                    results.push(entry.to_path_buf());
                }
            }
        } else {
            for entry in tqdm(self.objects.iter()) {
                let entry_stem = entry
                    .file_stem()
                    .unwrap_or(OsStr::new(""))
                    .to_str()
                    .unwrap_or("")
                    .to_lowercase();
                let entry_extension = entry
                    .extension()
                    .unwrap_or(OsStr::new(""))
                    .to_str()
                    .unwrap_or("")
                    .to_lowercase();

                if entry_stem.contains(&object_lowercase)
                    && entry_extension.contains(&extension_lowercase)
                {
                    results.push(entry.to_path_buf());
                }
            }
        }

        if results.is_empty() {
            return None;
        }
        Some(results)
    }
}

impl Deref for Seek {
    type Target = Vec<PathBuf>;
    fn deref(&self) -> &Vec<PathBuf> {
        &self.objects
    }
}

/// Similar to the `.search()` method from the seek struct. Except this searches within
/// a serde Value. After having read from the cache
///
/// PARAMETERS
/// ----------
///     &str object    -> The directory or file stem you are searching for
///     &str extension -> The extension of the file, if directory, pass ""
///     bool exact     -> true, if you're looking for an exact match, false otherwise
pub fn search_value(
    value: &Value,
    object: &str,
    extension: &str,
    exact: bool,
) -> Option<Vec<PathBuf>> {
    let object = object.to_lowercase();
    let extension = extension.to_lowercase();
    let mut results: Vec<PathBuf> = Vec::new();
    let data = &value["data"];
    if let Value::Array(entries) = data {
        if exact {
            for _entry in tqdm(entries.iter()) {
                let entry: PathBuf;
                if let Value::String(path) = _entry {
                    entry = PathBuf::from(path);
                } else {
                    continue;
                }
                let entry_stem = entry
                    .file_stem()
                    .unwrap_or(OsStr::new(""))
                    .to_str()
                    .unwrap_or("");
                let entry_extension = entry
                    .extension()
                    .unwrap_or(OsStr::new(""))
                    .to_str()
                    .unwrap_or("");
                if entry_stem.eq_ignore_ascii_case(&object)
                    && entry_extension.eq_ignore_ascii_case(&extension)
                {
                    results.push(entry);
                }
            }
        } else {
            for _entry in tqdm(entries.iter()) {
                let entry: PathBuf;
                if let Value::String(path) = _entry {
                    entry = PathBuf::from(path);
                } else {
                    continue;
                }
                let entry_stem = entry
                    .file_stem()
                    .unwrap_or(OsStr::new(""))
                    .to_str()
                    .unwrap_or("");
                let entry_extension = entry
                    .extension()
                    .unwrap_or(OsStr::new(""))
                    .to_str()
                    .unwrap_or("");
                if entry_stem.contains(&object) && entry_extension.contains(&extension) {
                    results.push(entry);
                }
            }
        }
    }
    if results.is_empty() {
        return None;
    }
    Some(results)
}
