//! Useful small functions that don't necessarily make sense belonging
//! in a specific module

// Importing from internal modules
use crate::options::Options;

// Importing from external modules
use chrono::prelude::*;
use clipboard::{ClipboardContext, ClipboardProvider};

// Using the standard library
use std::env::consts::OS;
use std::fmt::Display;
use std::fs::write;
use std::fs::OpenOptions;
use std::io::Result;
use std::io::{self, Write};
use std::io::{Error, ErrorKind};
use std::process::Command;

pub fn open_file(file: &str) -> Result<()> {
    #[cfg(target_os = "windows")]
    let mut cmd = Command::new("cmd")
        .arg("/C")
        .arg(file)
        .spawn()?;

    #[cfg(target_os = "linux")]
    let mut cmd = Command::new("xdg-open")
        .arg(file)
        .spawn()?;

    #[cfg(target_os = "macos")]
    let mut cmd = Command::new("open")
        .arg(file)
        .spawn()?;

    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    return Err(Error::new(ErrorKind::Unsupported, "OS not supported."));

    cmd.wait()?;
    Ok(())
}

/// An abstract function to write string content into a file
/// giving the option to append to such file via a parameter
pub fn write_to(loc: String, content: String, append: bool) -> Result<()> {
    if append {
        let mut file = OpenOptions::new().append(true).open(loc)?;
        file.write_all(("\n".to_owned() + &content).as_bytes())?;
        return Ok(());
    }
    write(loc, content)?;
    Ok(())
}

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
    for i in 0..array.len() {
        buffer[i % amount].push(array[i].clone());
    }
    buffer.into_iter().filter(|buf| !buf.is_empty()).collect()
}

/// Returns todays numerical day in u8
pub fn todays_day() -> u8 {
    let local = Local::now();
    local.day() as u8
}

/// Returns a pretty interface like list for the user to view
pub fn pretty_interface(data: &[String], enumerate: bool) -> String {
    let mut buffer: Vec<String> = Vec::new();
    for (i, n) in data.iter().enumerate() {
        buffer.push(if enumerate {
            format!("{}.) {}", i + 1, n)
        } else {
            n.to_owned()
        });
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
pub fn copy(value: String) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(value)
        .expect("Could not copy contents into the clipboard");
}

/// Prompts the user to a UI where the user
/// selects the file path he sought for
pub fn user_select(matches: &[String]) -> Option<String> {
    let options = Options::new(matches);

    loop {
        let response = input(">> ");
        if response.len() == 0 {
            return None;
        }

        if let Some(value) = options.evaluate(&response) {
            // returning the Some variant only,
            // otherwise it was a mistake and the user
            // probably wants to reselect
            return Some(value);
        }
    }
}

pub fn interpolate_to_command(cmd: String, path: &str) -> String {
    return cmd.replace("{}", path);
}

#[cfg(target_os = "windows")]
pub fn run_cmd(cmd_query: String) -> Result<()> {
    let mut cmd = Command::new("cmd")
        .arg("/C")
        .arg(cmd_query)
        .spawn()?;

    cmd.wait()?;

    return Ok(())
}

#[cfg(not(target_os = "windows"))]
pub fn run_cmd(cmd_query: String) -> Result<()> {
    let mut cmd = Command::new("sh")
        .arg("-c")
        .arg(cmd_query)
        .spawn()?;

    cmd.wait()?;

    return Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    //use seek::utils::format_num;

    #[test]
    fn t_interpolation() {
        let cmd = "type {} | clip".to_string();
        let path = "./hello.go";
        let expected_result = "type ./hello.go | clip".to_string();
        assert_eq!(interpolate_to_command(cmd, path), expected_result);
    }

    #[test]
    fn t_format_number_1000() {
        assert_eq!(format_num(1000), "1,000");
    }

    #[test]
    fn t_format_number_no_commas() {
        assert_eq!(format_num(0), "0");
    }

    #[test]
    fn t_distribute_even_data() {
        let data = vec![1, 2, 3, 4];
        let buckets = 2;

        let result = distribute::<i32>(&data, buckets);
        assert_eq!(result, vec![vec![1, 3], vec![2, 4]]);
    }

    #[test]
    fn t_distribute_uneven_data() {
        let data = vec![1, 2, 3];
        let buckets = 2;

        let result = distribute::<i32>(&data, buckets);
        assert_eq!(result, vec![vec![1, 3], vec![2]]);
    }

    #[test]
    fn t_distribute_no_data() {
        let data = vec![];
        let buckets = 2;

        let result: Vec<Vec<i32>> = distribute::<i32>(&data, buckets);
        let expected: Vec<Vec<i32>> = Vec::new(); // empty vec
        assert_eq!(result, expected);
    }
}
