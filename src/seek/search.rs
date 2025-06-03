//! Contains the search function

// importing from external crates
use regex::Regex;
use tokio::spawn;
use tokio::task::JoinHandle;

// Importing local modules
use crate::utils;

// Use of the standard library
use std::ffi::OsStr;
use std::io::Result;
use std::path::PathBuf;
use std::thread;

/// Takes in a regex with a few arguments to give
/// the best filtered result possible
pub fn search_buffer(
    paths: &[PathBuf],
    reg: Regex,
    log: bool,
    dirs: bool,
    files: bool,
    symlinks: bool,
) -> Vec<String> {
    let mut matches: Vec<String> = Vec::new();
    if files == dirs && dirs == symlinks {
        // if all object types are the same, (true or false), that means
        // no object type was specified or all types were specified, thus,
        // the function should consider all types
        for path in paths {
            let str_path = path.display().to_string();
            let base_name = path
                .file_name()
                .unwrap_or(OsStr::new(""))
                .to_str()
                .unwrap_or("");
            if reg.is_match(base_name) {
                matches.push(path.display().to_string());
            }
        }
    } else {
        for path in paths {
            if !dirs && path.is_dir() {
                continue;
            }
            if !files && path.is_file() {
                continue;
            }
            if !symlinks && path.is_symlink() {
                continue;
            }
            let base_name = path
                .file_name()
                .unwrap_or(OsStr::new(""))
                .to_str()
                .unwrap_or("");
            if reg.is_match(base_name) {
                matches.push(path.display().to_string());
            }
        }
    }
    matches
}

/// Asynchronous searching for optimized performance
pub async fn search(
    paths: &[PathBuf],
    reg: Regex,
    log: bool,
    dirs: bool,
    files: bool,
    symlinks: bool,
) -> Result<Vec<String>> {
    let cores_amount: usize = thread::available_parallelism()?.into();
    let buffers: Vec<Vec<PathBuf>> = utils::distribute(paths, cores_amount);
    let mut workers: Vec<JoinHandle<Vec<String>>> = Vec::new();
    for buffer in buffers {
        let regex = reg.clone();
        let worker: JoinHandle<Vec<String>> =
            spawn(async move { search_buffer(&buffer, regex, log, dirs, files, symlinks) });
        workers.push(worker);
    }
    let mut found_result: Vec<String> = Vec::new();
    for worker in workers {
        let result = worker.await?;
        found_result.extend(result);
    }
    Ok(found_result)
}
