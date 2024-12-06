/*
    Seeking functionality with asynchronous programming
*/
use std::fs;
use std::path::Path;
use serde_json::Value;
use std::ffi::OsStr;
use std::ops::Deref;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use tokio::task::JoinHandle;
use tqdm::tqdm;
use walkdir::WalkDir;
#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

fn walk(hint : &Path, depth : usize) -> Vec<PathBuf> {
    let mut entries: Vec<PathBuf> = Vec::new();
    let mut depth_count = 0;
    for entry in WalkDir::new(hint) {
        if depth_count == depth { break; }
        depth_count += 1;
        if entry.is_err() { continue }
        let bind = entry.unwrap().path().to_path_buf();
        entries.push(bind);
    }
    entries
}

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

//    /// Scans all directories starting from the `.hint` field name you provided
//    pub async fn scan(&mut self) {
//        let paths: Arc<RwLock<Vec<PathBuf>>> = Arc::new(RwLock::new(Vec::new()));
//        let path_dirs = path_entries(&self.hint);
//        let mut workers: Vec<JoinHandle<()>> = Vec::new();
//        for dir in path_dirs {
//            let pointer = Arc::clone(&paths);
//            let worker = tokio::spawn(async move {
//                walk(&dir, pointer);
//            });
//            workers.push(worker);
//        }
//        for worker in workers {
//            let _ = worker.await;
//        }
//        let binding = (*paths).read();
//        let paths = binding.unwrap();
//        self.objects = (*paths).to_owned();
//    }

    /// Scans all directories starting from the `.hint` field name you provided
    pub async fn scan(&mut self, _depth : usize) {
        let entries: Arc<RwLock<Vec<PathBuf>>> = Arc::new(RwLock::new(Vec::new()));
        // the first starting directories which are then going
        // to be scanned in individual threads.
        let initial_dirs: Vec<PathBuf> = {
            let mut bind = Vec::new();
            for i in fs::read_dir(&self.hint).unwrap() {
                if i.is_err() { continue }
                let i = i.unwrap().path();
                if !i.is_dir() {
                    let mut entries_ptr = entries.write().unwrap();
                    entries_ptr.push(i.to_path_buf());
                    continue;
                }
                bind.push(i.to_path_buf());
            }
            bind
        };
        let cores: usize = std::thread::available_parallelism().unwrap().into();
        let jobs: Vec<Vec<PathBuf>> = {
            //let mut bind: Vec<Vec<PathBuf>> = (0..cores).map(|_| Vec::new()).collect();
            let mut bind: Vec<Vec<PathBuf>> = Vec::new();
            let mut buffer: Vec<PathBuf> = Vec::new();
            // this is an internal anonymous function, does not return for the outer function
            let max = (|n : usize, total : usize| {
                if n < total { return n }
                n / total
            })(initial_dirs.len(), cores);
            let mut count = 0;
            for entry in initial_dirs.iter() {
                if count == max {
                    bind.push(buffer.clone());
                    buffer.clear();
                }
                buffer.push(entry.to_owned());
                count += 1;
            }
            // if buffer is not empty, increases the workload to the first core
            if !buffer.is_empty() {
                match bind.len() {
                    0 => bind.push(buffer),
                    _ => bind[0].extend(buffer)
                }
            }
            bind
        };
        let mut threads: Vec<_> = Vec::new();
        // starting up each core
        for job in jobs {
            let entry_ptr = Arc::clone(&entries);
            let job = job.clone();
            let thread = tokio::spawn(async move {
                let mut entry_ptr_ptr = entry_ptr.write().unwrap();
                for j in job {
                    let result = walk(&j, _depth);
                    entry_ptr_ptr.extend(result);
                }
            });
            threads.push(thread);
        }
        for thread in threads {
            let _ = thread.await;
        }
        let bind = entries.read().unwrap();
        self.objects = bind.clone();
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
