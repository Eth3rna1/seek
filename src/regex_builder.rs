//!
//! Modifies and builds the regex

// Importing from external libraries
use regex::escape;
use regex::Regex;
use regex::RegexBuilder;

// Making use of the standard library
use std::io::Result;
use std::io::{Error, ErrorKind};

/// given the regex string query, if exact is true,
/// the function will escape any special characters, matching the raw string
pub fn build_regex(query: String, case_sensitive: bool, exact: bool) -> Result<Regex> {
    let query = if exact {
        format!("^{}$", escape(&query))
    } else {
        query
    };
    return match RegexBuilder::new(&query)
        // inverting the booleans to match logic
        .case_insensitive(!case_sensitive || !exact)
        .build()
    {
        Ok(reg) => Ok(reg),
        Err(error) => Err(Error::new(ErrorKind::Other, error.to_string())),
    };
}
