//! # CLI
//!
//! This module defines the data structures used to parse command line
//! arguments when running the program.

use clap::{Parser, Subcommand};

/// This struct represents the top-level CLI entry point for the tool.
#[derive(Parser)]
#[command(about = "Manages the corpus of C-Rust program pairs", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// This struct represents the different commands available.
#[derive(Subcommand)]
pub enum Commands {
    /// Downloads a subset of the corpus; used for demonstration.
    Demo,

    /// Downloads all C-Rust program pairs.
    Download,

    /// Delete the `program_pairs` and `repository_clones` directories.
    Delete,
}
