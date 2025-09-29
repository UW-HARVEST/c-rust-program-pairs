//! # C-Rust program pair downloader

mod corpus;
mod paths;

use std::{env, fs, io::Error, path::Path};

pub use corpus::download_program_pairs;

use crate::paths::{PROGRAM_PAIRS_DIRECTORY, REPOSITORY_CLONES_DIRECTORY};

use crate::paths::{PROGRAM_PAIRS_DIRECTORY, REPOSITORY_CLONES_DIRECTORY};

/// Downloads program pairs.
///
/// Reads the command-line arguments supplied. If none are given, download
/// all program pairs. If argument "demo" is given, download program pairs
/// specified within the `demo/` directory.
pub fn run() {
    let args: Vec<String> = env::args().collect();

    match args.get(1).map(|arg| arg.as_str()) {
        None => download_program_pairs(false).expect("Failed to download program pairs"),
        Some("demo") => download_program_pairs(true).expect("Failed to run demo"),
        Some("delete") => delete().expect("Failed to delete directories"),
        Some(arg) => eprintln!("Invalid argument: {arg}"),
    }
}

/// Removes all downloaded program-pairs and repository clones.
///
/// This deletes the directories specified by
/// [`PROGRAM_PAIRS_DIRECTORY`] and [`REPOSITORY_CLONES_DIRECTORY`],
/// along with all their contents, if they exist.
fn delete() -> Result<(), Error> {
    if Path::new(PROGRAM_PAIRS_DIRECTORY).exists() {
        fs::remove_dir_all(PROGRAM_PAIRS_DIRECTORY)?;
    };
    if Path::new(REPOSITORY_CLONES_DIRECTORY).exists() {
        fs::remove_dir_all(REPOSITORY_CLONES_DIRECTORY)?;
    };
    Ok(())
}
