/*
    Module containing necessary but general functions
*/
use chrono::prelude::*;
use std::path::PathBuf;

/// Given a string, the function will verify if all the characters are valid utf-8
///
/// PARAMETERS:
/// -----------
///     &str os_str -> The string to check the characters for
pub fn is_utf8(os_str: &str) -> bool {
    if let Some(os_str_bytes) = os_str.as_bytes().to_vec().as_slice().get(0..) {
        return std::str::from_utf8(os_str_bytes).is_ok();
    }
    false
}

/// Returns a formatted and enumerated list containing all the paths that were given
///
/// PARAMETERS:
/// -----------
///     &Vec<PathBuf> values -> The paths you are trying to show the client
pub fn string_interface(values: &Vec<PathBuf>) -> String {
    let mut interface: String = String::new();
    for (index, value) in values.iter().enumerate() {
        interface += format!("{}.) {}", index + 1, value.display()).as_str();
        if index != values.len() - 1 {
            interface += "\n======\n";
        }
    }
    interface
}

/// Returns todays numerical day in u8
pub fn todays_day() -> u8 {
    let local = Local::now();
    local.day() as u8
}

/// Given the object (folder or file name) the function will parse and return a tuple with the stem name
/// and the extension if any in an Option enum
///
/// PARAMETERS:
/// -----------
///     &str object -> the file or directory name
pub fn parse_object(object: &str) -> (String, Option<String>) {
    match object.chars().filter(|charac| *charac == '.').count() {
        0 => (object.to_string(), None),
        1 => {
            let splitted = object.split('.').collect::<Vec<&str>>();
            (splitted[0].to_string(), Some(splitted[1].to_string()))
        }
        _ => {
            let splitted = object.split('.').collect::<Vec<&str>>();
            (
                splitted[..splitted.len() - 1].join(".").to_string(),
                Some(splitted[splitted.len() - 1].to_string()),
            )
        }
    }
}

pub fn format_num(n: usize) -> String {
    let mut buffer: Vec<char> = Vec::new();
    let mut i: usize = 0;
    for c in n.to_string().chars().rev() {
        if i % 3 == 0 && i != 0 {
            buffer.push(',');
        }
        buffer.push(c);
        i += 1;
    }
    buffer.reverse();
    buffer.iter().collect()
}
