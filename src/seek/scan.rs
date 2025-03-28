//! Contains the seek functionality
//!
//! Two functions, `scan()` and `seek()`
//!
//! `scan()` is an asynchronous function meanwhile `seek()` isn't
// Other functions or structures located somewhere else within the crate
use crate::seek::ScanResult;
use crate::utils;

// Importing specific functions and structures from external crates
use tokio::task::JoinHandle;
use walkdir::WalkDir;

// Making use of the standard library
use std::fs;
use std::io::Result;
use std::io::{Error, ErrorKind};
use std::path::Path;
use std::path::PathBuf;
use std::thread;

/// Iterates through the directories given recursively
fn walk_all(dirs: &[PathBuf], depth: usize, log: bool) -> ScanResult {
    let mut result = ScanResult::new();
    if depth == 0 {
        return result;
    }
    for dir in dirs.iter() {
        let mut buffer = ScanResult::new();
        let mut d = 1;
        for entry in WalkDir::new(dir) {
            if d == depth {
                break;
            }
            d += 1;
            match entry {
                Ok(entry) => {
                    let entry = entry.path().to_path_buf();
                    buffer.push(entry);
                }
                Err(error) => {
                    buffer.increase_error(1);
                    if log {
                        eprintln!("{}", error.to_string());
                    }
                }
            }
        }
        // increasing the success counter at the end to decrease overhead
        buffer.increase_success(buffer.size());
        // merging the buffer into the main result structure
        result.append(buffer);
    }
    result
}

/// Scans all directories asynchronously keeping track of an error counter along the way
pub async fn scan(path: &Path, depth: usize, log: bool) -> Result<ScanResult> {
    if !path.exists() {
        return Err(Error::new(
            ErrorKind::NotFound,
            format!(
                "`{}` could not be found or directory doesn't exist",
                path.display()
            ),
        ));
    }
    if !path.is_dir() {
        return Err(Error::new(
            ErrorKind::Other,
            format!("`{}` is not a directory", path.display()),
        ));
    }

    //let mut collector = Vec::new();
    //let mut success: usize = 0;
    //let mut errors: usize = 0;
    //
    // Following struct encapsulates the previous logic
    let mut result = ScanResult::new();

    let initial_dirs: Vec<PathBuf> = {
        let mut bind = Vec::new();
        for entry in fs::read_dir(path)? {
            match entry {
                Ok(entry) => {
                    result.increase_success(1);
                    let entry = entry.path().to_path_buf();
                    if entry.is_dir() {
                        bind.push(entry.to_owned());
                    } else {
                        // not a directory, hence it doesn't need to be
                        // in the initial_dirs to be walked
                        result.push(entry.to_owned());
                    }
                }
                Err(error) => {
                    result.increase_error(1);
                    if log {
                        eprintln!("{}", error.to_string());
                    }
                }
            }
        }
        bind
    };

    let cores_amount: usize = thread::available_parallelism()?.into();
    // evenly distributing the workload per each core
    let mut workload_per_core: Vec<Vec<PathBuf>> =
        utils::distribute::<PathBuf>(&initial_dirs, cores_amount);
    let mut workers: Vec<JoinHandle<ScanResult>> = Vec::new();
    {
        // initializing asynchronous threads
        for workload in workload_per_core.iter() {
            let w = workload.clone();
            let worker: JoinHandle<ScanResult> =
                tokio::spawn(async move { walk_all(&w, depth, log) });
            workers.push(worker);
        }
    }
    {
        // awaiting all threads
        for worker in workers {
            let worker_result: ScanResult = worker.await?;
            result.append(worker_result);
        }
    }
    Ok(result)
}
