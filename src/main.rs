//! A rust binary to seek objects quickly via the terminal with caching functionality
#![allow(warnings)]
/// Defining modules
mod cache;
mod options;
mod regex_builder;
mod seek;
mod utils;

/// Importing from internal and external libraries and modules
use cache::Cache;
use cache::Data;
use clap::Parser;
use regex::Regex;
use regex_builder::build_regex;
use seek::filter_excluded_dirs;
use seek::filter_included_dirs;
use seek::scan;
use seek::search;
use seek::ScanResult;
use log::{
    warn,
    error,
    info
};

use std::collections::HashSet;
/// Making use of the standard library
use std::env::consts::OS;
use std::env::current_dir;
use std::io::Result;
use std::path::PathBuf;
use std::path::MAIN_SEPARATOR;
use std::process::exit;
use std::thread;
use std::time::Instant;

/// Seek, any object via the terminal with caching functionality.
///
/// The program doesn't automatically cache to not make it cumbersome.
///
/// To start caching, make use of the `-u` flag, which indicates to use the cache.
///
/// For further help, reference the help menu.
#[derive(Debug, Clone, Parser)]
struct Arguments {
    /// The regex query to apply on the base name of paths
    query: String,

    /// The initial path to seek from [default: current working directory]
    #[arg(short, long)]
    path: Option<String>,

    /// The recursion depth limit when walking directories
    #[arg(long, default_value_t = 1_000_000)]
    depth: usize,

    /// Logs the state of the program to the standard output
    #[arg(short, long)]
    log: bool,

    /// Signals to start seeking from the root directory
    #[arg(short, long)]
    root: bool,

    /// Only seek files
    #[arg(short, long)]
    files: bool,

    /// Only seek directories
    #[arg(short, long)]
    dirs: bool,

    /// Output file you want to store the final result if any
    #[arg(long)]
    output_file: Option<String>,

    /// Used alongside `--output-file`, indicates to append the result instead of overwriting
    #[arg(long)]
    append: bool,

    /// Used alongside `--output-file`, indicates to write the result enumerated
    #[arg(long)]
    enumerate: bool,

    /// Only seek symbolic links
    #[arg(short, long)]
    symlinks: bool,

    /// Case sensitive regex matching
    #[arg(long)]
    cs: bool,

    /// Modifies the regex query to match
    /// the exact string literal
    #[arg(short, long)]
    exact: bool,

    /// The cache location to store or read from
    #[arg(long, default_value_t=String::from("./.info.json"))]
    cache_location: String,

    /// Signals to only scan and cache without any search
    #[arg(short, long)]
    cache: bool,

    /// Ignores cache invalidity, searching the existing cache anyway
    #[arg(short, long)]
    ignore_update: bool,

    /// Signals to update the cache regardless of its validity
    #[arg(long)]
    update_cache: bool,

    /// Uses the cache instead of scanning directories
    #[arg(short, long)]
    use_cache: bool,

    /// Specifies what parent directory name should be present within
    /// the found paths. If not present, automatically discards path
    #[arg(long)]
    include: Vec<String>,

    /// Specifies what parent directory name should NOT be present
    /// within the found paths. If present, automatically discards path
    #[arg(long, short = 'x')]
    exclude: Vec<String>,

    /// Instead of copying the selected path, the
    /// file is ran in an attempt to open it
    #[arg(short)]
    open: bool,
}

impl Arguments {
    fn get_path(&self) -> String {
        if self.root {
            match OS {
                "windows" => return "C:\\".to_string(),
                _ => return "/".to_string(),
            };
        } else if let Some(path) = &self.path {
            return path.replace("/", &MAIN_SEPARATOR.to_string());
        }
        current_dir()
            .unwrap_or(PathBuf::from("."))
            .to_str()
            .unwrap_or(".")
            .to_string()
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Arguments::parse();
    let path = PathBuf::from(args.get_path());
    let cache = Cache::new(&args.cache_location);

    // initializing the pretty logger with Info level tracing
    pretty_env_logger::formatted_builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    let data: Data = if args.cache || args.use_cache || args.update_cache {
        // If user wants to do anything with the cache
        // obtaining the data from cache
        cache.summon()?;
        let data: Data = if cache.is_valid() && !args.update_cache {
            // if cache is valid and user didn't specify to update the cache
            if args.cache {
                // it doesn't matter if the cache is valid,
                // raising the --cache (-c) flag is reserved for
                // solely caching; early exits. Otherwise, the user
                // should be using the --update-cache flag to force an update.
                exit(0); // user just wanted to cache
            }
            cache.read()?
        } else {
            // if cache is invalid or user wants to force an update
            if !args.ignore_update {
                // If the user didn't specify to ignore the validity of the cache
                if args.log {
                    warn!("Cache is invalid.");
                    info!("Scanning directories...");
                }

                let start = Instant::now();
                let result: ScanResult = scan(&path, args.depth, args.log).await?;
                let end = Instant::now();
                let data: Data = Data::from(result.paths);

                // cache is now updated
                cache.write(&data)?;

                if args.log {
                    info!("Updated cache.");
                    info!("Cached into `{}`", cache.location().display());
                    print!("\n"); // new line for better visuals
                    info!("Scanned in: {:?}\n", end - start);
                    info!("Success: {}", utils::format_num(result.success_count));
                    info!("Errors: {}", utils::format_num(result.error_count));
                }

                if args.cache {
                    exit(0); // user just wanted to cache
                }
            }

            cache.read()?
        };

        data
    } else {
        // no need to touch the cache because if was not indicated
        if args.log {
            info!("Scanning directories...");
        }

        let start = Instant::now();
        let result: ScanResult = scan(&path, args.depth, args.log).await?;
        let end = Instant::now();
        let data: Data = Data::from(result.paths);

        if args.log {
            print!("\n"); // a new line for better visuals
            info!("Scanned in: {:?}\n", end - start);
            info!("Success: {}", utils::format_num(result.success_count));
            info!("Errors: {}", utils::format_num(result.error_count));
        }

        data
    };

    // Next Step: Searching data
    if args.log {
        info!("Matching query...");
    }

    let query: Regex = build_regex(args.query, args.cs, args.exact)?;

    let start = Instant::now();
    let mut matches: Vec<PathBuf> =
        search(&data.data, query, args.dirs, args.files, args.symlinks).await?;
    let end = Instant::now();

    // filtering based on argument specifications
    matches = filter_included_dirs(matches, &args.include);
    matches = filter_excluded_dirs(matches, &args.exclude);

    if matches.is_empty() {
        print!("\n"); // just adding a new line for better visual
        error!("No matches were found.");
        exit(1);
    }

    // mapping to a different type for terminal output
    let matches: Vec<String> = matches.iter().map(|p| p.display().to_string()).collect();

    let beautified_ui: String = if !args.output_file.is_none() {
        // if an output file was specified
        //
        // `args.enumerate` by default false
        utils::pretty_interface(&matches, args.enumerate)
    } else {
        // no specification of an output file
        utils::pretty_interface(&matches, true)
    };

    // in case of wanting to save to a file instead
    if let Some(file) = args.output_file {
        // Reminder: args.append is a boolean flag
        let _ = utils::write_to(file, beautified_ui, args.append);
        return Ok(());
    }

    // Displays the interface
    println!("\n{}\n", beautified_ui);

    if args.log {
        info!("Searched in: {:?}\n", end - start);
    }

    let selected_path: Option<String> = {
        // prompting a different message based on the argument given
        match args.open {
            true => println!(
                "Please select the path via its index to open\n\
Or press `Enter` to exit"
            ),

            false => println!(
                "Please select the path via its index to copy onto the clipboard\n\
Or press `Enter` to exit"
            ),
        }

        utils::user_select(&matches)
    };

    // selected path
    let path: String = match selected_path {
        Some(p) => p,
        None => return Ok(()), // user didn't select anything
    };

    if args.open {
        // user wants to open the file

        utils::open_file(&path)?;
        return Ok(());
    }

    // An interface to select and copy a path
    utils::copy(path);
    println!("Copied path onto the clipboard");

    Ok(())
}
