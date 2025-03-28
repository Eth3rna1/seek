//! Useful small functions that don't necessarily make sense belonging
//! in a specific module

// Importing from internal modules
use crate::options::Options;

// Importing from external modules
use chrono::prelude::*;
use clipboard::{ClipboardContext, ClipboardProvider};

// Using the standard library
use std::fmt::Display;
use std::io::{self, Write};
use std::str::from_utf8;

/// Adds commas to a large number
pub fn format_num(n: usize) -> String {
    // a vector instead of a string to avoid continuous dynamic sizing
    let mut buffer: Vec<char> = Vec::new();
    // Efficiency: Since we are only interested in the position (for every third character) and the character itself, manually tracking the count avoids the overhead of constructing and unpacking tuples, making the loop more efficient.
    let mut i = 0;
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

/// Attempts to evenly distribute an array into smaller buffers
pub fn distribute<T: Clone>(array: &[T], amount: usize) -> Vec<Vec<T>> {
    let mut buffer: Vec<Vec<T>> = (0..amount).map(|_| Vec::new()).collect();
    let mut pointer: usize = 0;
    for item in array {
        if pointer == buffer.len() {
            pointer = 0;
        }
        buffer[pointer].push(item.clone());
        pointer += 1;
    }
    buffer.into_iter().filter(|buf| !buf.is_empty()).collect()
}

/// Returns todays numerical day in u8
pub fn todays_day() -> u8 {
    let local = Local::now();
    local.day() as u8
}

/// Returns a pretty interface like list for the user to view
pub fn pretty_interface(data: &[String]) -> String {
    let mut buffer: Vec<String> = Vec::new();
    for (i, n) in data.iter().enumerate() {
        buffer.push(format!("{}.) {}", i + 1, n));
    }
    buffer.join("\n")
}

/// Outputs a prompt before taking in user input
fn input<T: Display + AsRef<str> + ?Sized>(prompt: &T) -> String {
    let mut stdout = io::stdout();
    write!(stdout, "{}", prompt).unwrap();
    let _ = stdout.flush();
    let mut response = String::new();
    let _ = io::stdin().read_line(&mut response);
    response = response.replace(['\n', '\r'], "");
    response
}

/// Copies a string onto the clipboard
fn copy(value: String) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(value)
        .expect("Could not copy contents into the clipboard");
}

/// A function that intakes user input to select a specific match
/// and copy onto the clipboard.
pub fn copy_shell(matches: &[String]) {
    println!(
        "Please select the path via its index to copy onto the clipboard\n\
Or press `Enter` to exit"
    );
    let mut options = Options::new(matches);
    loop {
        let response = input(">> ");
        if response.len() == 0 {
            return;
        }
        if let Some(value) = options.evaluate(&response) {
            copy(value);
            return;
        }
    }
}
