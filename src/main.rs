/*
    Main file
*/
mod cache;
mod seek;
mod tool;
use cache::Cache;
use clap::Parser;
use seek::search_value;
use seek::Seek;
use std::env::{self, current_dir};
use std::path::PathBuf;
use std::process::exit;
use std::time::Instant;

const DEPTH: usize = 1_000_000; // default max depth

/// Seek files or directories from any child tree starting from your current directory or root directory
#[derive(Parser, Debug, PartialEq)]
struct Arguments {
    /// The file or directory you are looking for
    object: String,

    /// The extension of the file you are searching for if applicable
    #[arg(short, long)]
    extension: Option<String>,

    /// True, if you want to search for the exact file stem and extension, false otherwise
    #[arg(long)]
    exact: bool,

    /// The path by which you want to start searching, DEFAULT = current working directory
    #[arg(short, long)]
    path: Option<String>,
    /// If raised, the search will start from the root directory `c:\\`
    #[arg(short, long)]
    root: bool,

    /// If raised, instead of starting a new process,
    /// a file containing the files and directories will be created and read.
    /// NOTE: cache is ignored by default
    #[arg(short, long, name = "use-cache")]
    use_cache: bool,

    /// Outputs messages describing the process
    #[arg(short, long)]
    log: bool,

    /// Paths are saved into a file for later use, no paths are outputted
    #[arg(short, long)]
    cache: bool,

    /// The name of the cache file
    #[arg(short, long, default_value_t = String::from("info.json"))]
    name: String,

    /// Update the cache file. Use along with the --cache (-c) or --use-cache (-u) flags.
    #[arg(long)]
    update_cache : bool,

    /// The amount of recursion wanted to search in
    #[arg(long, short)]
    depth : Option<usize>
}

impl Arguments {
    pub fn get_path(&self) -> String {
        if self.root {
            match env::consts::OS {
                "windows" => return "C:\\".to_string(),
                _ => return "/".to_string(),
            };
        } else if let Some(path) = &self.path {
            return path.to_string();
        }
        current_dir()
            .unwrap_or(PathBuf::from("."))
            .to_str()
            .unwrap_or(".")
            .to_string()
    }
}

#[tokio::main]
async fn main() {
    let mut args = Arguments::parse();
    // separating the object from the extension
    let depth: usize;
    if let Some(_depth) = args.depth {
        depth = _depth;
    } else {
        depth = DEPTH;
    }
    let (object, extension) = tool::parse_object(&args.object);
    args.object = object;
    if args.object == "*" {
        // making sure to follow file special characters
        args.object = String::from("");
    }
    // if the extension flag wasn't raised and specified
    if args.extension.is_none() {
        if let Some(ref _ext) = extension {
            // reassigning the .extension field to the object
            args.extension = Some(_ext.to_string());
        }
    }
    let path = args.get_path();
    let mut seek = Seek::new(&path);
    let result: Option<Vec<PathBuf>>;
    if args.use_cache || args.cache {
        let mut cache = Cache::new(&Vec::new());
        // regardless if the client specified a name, the file name is reassigned without even if it's still `info.json`
        cache.name = args.name.clone();
        if args.log {
            if !args.cache {
                println!("Seeking...");
            }
            if !cache.exists() {
                cache.summon().expect("Unable to create the file");
                println!("Created file `{}`.", &cache.name);
            } else { // nothing happens in this else statement
                // Meaning cache file does exist
                if args.update_cache {
                // an if statement for more accurate logging messages
                    println!("Updating cache...");
                }
            }
            if !cache.made_today() || args.update_cache {
                println!("Scanning directories...");
                let start = Instant::now();
                seek.scan(depth).await;
                let end = Instant::now();
                println!("Scanned in: {:?}", end - start);
                let mut cache = Cache::new(&(*seek).clone());
                cache.name = args.name;
                println!("Saving cache...");
                cache.save(&seek).expect("Unable to save cache!");
                println!("Cache saved.");
            }
            if args.cache {
                // client just wanted to cache
                exit(0);
            }
            println!("Reading from cache...");
        } else {
            if !cache.exists() {
                cache.summon().expect("Unable to create the file");
            }
            if !cache.made_today() || args.update_cache {
                seek.scan(depth).await;
                let mut cache = Cache::new(&(*seek).clone());
                cache.name = args.name;
                cache.save(&seek).expect("Unable to save cache!");
            }
            if args.cache {
                // client just wanted to cache
                exit(0)
            }
        }
        let value = cache.retrieve().expect("Couldn't retrieve cache!");
        // I have to read from the cache which returns an IO Result enum containing Value, using the .value
        result = search_value(
            &value,
            &args.object,
            &args.extension.unwrap_or("".to_string()),
            args.exact,
        );
    } else {
        if args.log {
            if !args.cache {
                println!("Seeking...");
            }
            let start = Instant::now();
            println!("Scanning directories...");
            seek.scan(depth).await;
            let end = Instant::now();
            println!("Scanned in: {:?}", end - start);
            println!("Searching...");
        } else {
            seek.scan(depth).await;
        }
        result = seek.search(
            &args.object,
            &args.extension.unwrap_or("".to_string()),
            args.exact,
        );
    }
    if let Some(paths) = result {
        let interface = tool::string_interface(&paths);
        println!("{}", interface);
    } else {
        eprintln!("No instances were found.")
    }
}
