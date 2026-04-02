//! Contains the `search()` implementation

// importing from external crates
use regex::Regex;
use tokio::spawn;
use tokio::task::JoinHandle;

// Importing local modules
use crate::utils;

// Use of the standard library
use std::ffi::OsStr;
use std::fs;
use std::io::Result;
use std::path::Path;
use std::path::PathBuf;
use std::thread;

fn get_base_name(path: &Path) -> String {
    path.file_name()
        .unwrap_or(OsStr::new(""))
        .to_str()
        .unwrap_or("")
        .to_string()
}

/// Helper function used in `search`
///
/// Takes in a regex with a few arguments to give
/// the best filtered result possible
pub fn search_buffer(
    paths: &[PathBuf],
    reg: Regex,
    dirs: bool,
    files: bool,
    symlinks: bool,
) -> Vec<PathBuf> {
    let mut matches: Vec<PathBuf> = Vec::new();

    if files == dirs && dirs == symlinks {
        // if all object types are the same, (true or false), that means
        // no object type was specified or all types were specified, thus,
        // the function should consider all types
        for path in paths {
            let base_name = get_base_name(path);

            if reg.is_match(&base_name) {
                //matches.push(path.display().to_string());
                matches.push(path.to_owned());
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

            let base_name = get_base_name(path);

            if reg.is_match(&base_name) {
                matches.push(path.to_owned());
            }
        }
    }

    matches
}

/// Asynchronous searching for optimized performance
pub async fn search(
    paths: &[PathBuf],
    reg: Regex,
    dirs: bool,
    files: bool,
    symlinks: bool,
) -> Result<Vec<PathBuf>> {
    let cores_amount: usize = thread::available_parallelism()?.into();
    let buffers: Vec<Vec<PathBuf>> = utils::distribute(paths, cores_amount);
    let mut workers: Vec<JoinHandle<Vec<PathBuf>>> = Vec::new();

    for buffer in buffers {
        let regex = reg.clone();
        let worker: JoinHandle<Vec<PathBuf>> =
            spawn(async move { search_buffer(&buffer, regex, dirs, files, symlinks) });
        workers.push(worker);
    }

    let mut found_result: Vec<PathBuf> = Vec::new();

    for worker in workers {
        let result: Vec<PathBuf> = worker.await?;
        found_result.extend(result);
    }

    Ok(found_result)
}

/// Filters out found instance paths
/// based if whether a parent directory
/// was specified to be explicitly present
/// given in the `excluded_names` parameter
pub fn filter_included_dirs(result_paths: Vec<PathBuf>, included_names: &[String]) -> Vec<PathBuf> {
    if included_names.is_empty() {
        return result_paths;
    }

    // initiating the vector with around the same capacity as the result paths
    let mut filtered: Vec<PathBuf> = Vec::with_capacity(result_paths.len());

    for path in result_paths.iter() {
        let full_path: PathBuf = if path.is_absolute() {
            path.to_owned()
        } else {
            if let Ok(_full_path) = fs::canonicalize(path) {
                _full_path
            } else {
                // returning the original
                // path if errors
                path.to_owned()
            }
        };

        // partitioning the path and checking the basenames

        let mut is_included = false;

        for ancestor in full_path.ancestors() {
            let base_name = get_base_name(ancestor);

            if included_names.contains(&base_name) {
                is_included = true;
                break;
            }
        }

        if is_included {
            filtered.push(path.to_owned());
        }
    }

    filtered
}

/// Filters out found instance paths
/// based if whether a parent directory
/// was specified to be explicitly *NOT* present
/// given in the `excluded_names` parameter
pub fn filter_excluded_dirs(result_paths: Vec<PathBuf>, excluded_names: &[String]) -> Vec<PathBuf> {
    if excluded_names.is_empty() {
        return result_paths;
    }

    let mut filtered: Vec<PathBuf> = Vec::with_capacity(result_paths.len());

    for path in result_paths.iter() {
        let full_path: PathBuf = if path.is_absolute() {
            path.to_owned()
        } else {
            if let Ok(_full_path) = fs::canonicalize(path) {
                _full_path
            } else {
                // returning the original
                // path if errors
                path.to_owned()
            }
        };

        // partitioning the path and checking the basenames

        let mut is_excluded = false;

        for ancestor in full_path.ancestors() {
            let base_name = get_base_name(ancestor);

            if excluded_names.contains(&base_name) {
                is_excluded = true;
                break;
            }
        }

        if !is_excluded {
            filtered.push(path.to_owned())
        }
    }

    filtered
}

#[cfg(test)]
mod tests {
    use super::*;
    //use seek::utils::format_num;

    #[test]
    fn t_get_base_name() {
        let pathbuf = PathBuf::from("./this/path");

        assert_eq!(get_base_name(&pathbuf), "path".to_string());
    }

    #[test]
    fn t_is_excluded() {
        let exclusion_names: &[String] = &["a".to_string(), "c".to_string()];

        let found_paths: Vec<PathBuf> = vec![
            PathBuf::from("./a/this/is/an/example/path/"),
            PathBuf::from("./b/this/is/an/example/path/"),
            PathBuf::from("./c/this/is/an/example/path/"),
        ];

        let filtered_result = filter_excluded_dirs(found_paths, &exclusion_names);
        let result = vec![PathBuf::from("./b/this/is/an/example/path/")];

        assert_eq!(filtered_result, result);
    }

    #[test]
    fn t_is_included() {
        let inclusion_names: &[String] = &["a".to_string(), "c".to_string()];

        let found_paths: Vec<PathBuf> = vec![
            PathBuf::from("./a/this/is/an/example/path/"),
            PathBuf::from("./b/this/is/an/example/path/"),
            PathBuf::from("./c/this/is/an/example/path/"),
        ];

        let filtered_result = filter_included_dirs(found_paths, &inclusion_names);
        let result = vec![
            PathBuf::from("./a/this/is/an/example/path/"),
            PathBuf::from("./c/this/is/an/example/path/"),
        ];

        assert_eq!(filtered_result, result);
    }
}
