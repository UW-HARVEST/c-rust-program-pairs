//! # C-Rust program pair downloader
//!
//! This crate parses metadata files and downloads the C and Rust programs,
//! storing them in the `programs/` directory.

use std::env;

mod corpus;
mod paths;

/// Entry point for the metadata downloader.
///
/// # Usage
///
/// Download full metadata corpus:
/// ```bash
/// cargo run
/// ```
///
/// Download demo/sample metadata only:
/// ```bash
/// cargo run -- demo
/// ```
fn main() {
    let args: Vec<String> = env::args().collect();

    match args.get(1).map(|s| s.as_str()) {
        None => corpus::download_metadata(false).expect("Failed to download program pairs"),
        Some("demo") => corpus::download_metadata(true).expect("Failed to run demo"),
        Some(arg) => eprintln!("Invalid argument: {arg}"),
    }
}
