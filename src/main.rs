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
use seek::scan;
use seek::search;
use seek::ScanResult;

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
    #[arg(long, default_value_t=String::from("./info.json"))]
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
    let data: Data = if args.cache || args.use_cache || args.update_cache {
        // If user wants to do anything with the cache
        // obtaining the data from cache
        cache.summon()?;
        let data: Data = if cache.is_valid() && !args.update_cache {
            // if cache is valid
            cache.read()?
        } else {
            // if cache is invalid or user wants to force an update
            if !args.ignore_update {
                // If the user didn't specify to ignore the validity of the cache
                if args.log {
                    println!("Scanning directories...");
                }
                let start = Instant::now();
                let result: ScanResult = scan(&path, args.depth, args.log).await?;
                let end = Instant::now();
                let data: Data = Data::from(result.paths);
                // cache is now updated
                cache.write(&data)?;
                if args.log {
                    println!("Updated cache.");
                    println!("Cached into `{}`", cache.location().display());
                    println!("Scanned in: {:?}", end - start);
                    println!("Success: {}", utils::format_num(result.success_count));
                    println!("Errors: {}", utils::format_num(result.error_count));
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
            println!("Scanning directories...");
        }
        let start = Instant::now();
        let result: ScanResult = scan(&path, args.depth, args.log).await?;
        let end = Instant::now();
        let data: Data = Data::from(result.paths);
        if args.log {
            println!("\nScanned in: {:?}\n", end - start);
            println!("Success: {}", utils::format_num(result.success_count));
            println!("Errors: {}", utils::format_num(result.error_count));
        }
        data
    };
    // Next Step: Searching data
    if args.log {
        println!("Searching...");
    }
    let query: Regex = build_regex(args.query, args.cs, args.exact)?;
    let start = Instant::now();
    let matches: Vec<String> = search(
        &data.data,
        query,
        args.log,
        args.dirs,
        args.files,
        args.symlinks,
    )
    .await?;
    let end = Instant::now();
    if matches.is_empty() {
        eprintln!("\nNo matches were found.\n");
        exit(1);
    }
    // Displays the interface
    println!("\n{}\n", utils::pretty_interface(&matches));
    if args.log {
        println!("Searched in: {:?}\n", end - start);
    }
    // An interface to select and copy a path
    utils::copy_shell(&matches);
    Ok(())
}
