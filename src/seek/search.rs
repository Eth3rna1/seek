//! Contains the search function

// importing from external crates
use regex::Regex;

// Use of the standard library
use std::ffi::OsStr;
use std::path::PathBuf;

/// Takes in a regex with a few arguments to give
/// the best filtered result possible
pub fn search(
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
