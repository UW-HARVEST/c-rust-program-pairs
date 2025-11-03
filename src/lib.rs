//! # C-Rust program pair downloader

mod cli;
mod corpus;
mod paths;

use clap::Parser;

use crate::cli::{Cli, Commands};

/// Downloads program pairs.
///
/// Reads the command-line arguments supplied. If none are given, download
/// all program pairs. If argument "demo" is given, download program pairs
/// specified within the `demo/` directory.
pub fn run() {
    let cli = Cli::parse();
    match cli.command {
        None => corpus::download_program_pairs(false).expect("Failed to download program pairs"),
        Some(Commands::Demo) => corpus::download_program_pairs(true).expect("Failed to run demo"),
        Some(Commands::Download) => {
            corpus::download_program_pairs(false).expect("Failed to download program pairs")
        }
        Some(Commands::Delete) => corpus::delete().expect("Failed to delete directories"),
    }
}
