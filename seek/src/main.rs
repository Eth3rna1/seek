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
use std::env::current_dir;
use std::path::PathBuf;
use std::process::exit;
use std::time::Instant;

/// Struct with parsed command line arguments
#[derive(Parser, Debug, PartialEq)]
struct Arguments {
    /// The file stem name of the desired folder or file you are searching for
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
}

impl Arguments {
    pub fn get_path(&self) -> String {
        if self.root {
            #[cfg(target_os = "windows")]
            return "c:\\".to_string();
            #[cfg(not(target_os = "windows"))]
            return "/".to_string();
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
            println!("Seeking...");
            if !cache.exists() {
                cache.summon().expect("Unable to create the file");
                println!("Created file `{}`.", &cache.name);
            }
            if !cache.made_today() {
                println!("Scanning directories...");
                let start = Instant::now();
                seek.scan().await;
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
            if !cache.made_today() {
                seek.scan().await;
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
            println!("Seeking...");
            let start = Instant::now();
            println!("Scanning directories...");
            seek.scan().await;
            let end = Instant::now();
            println!("Scanned in: {:?}", end - start);
            println!("Searching...");
        } else {
            seek.scan().await;
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
