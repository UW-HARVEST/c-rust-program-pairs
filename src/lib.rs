//! # C-Rust program pair downloader

mod corpus;
mod paths;

use std::env;

pub use corpus::download_metadata;

/// Downloads program-pairs.
///
/// Reads the command-line arguments supplied. If none are given, download
/// all program-pairs. If argument "demo" is given, download program-pairs
/// specified within the `demo/` directory.
pub fn run() {
    let args: Vec<String> = env::args().collect();

    match args.get(1).map(|arg| arg.as_str()) {
        None => download_metadata(false).expect("Failed to download program pairs"),
        Some("demo") => download_metadata(true).expect("Failed to run demo"),
        Some(arg) => eprintln!("Invalid argument: {arg}"),
    }
}
