//! # Corpus
//!
//! This module transforms schema files into strongly-typed Rust structs.

mod delete;
pub mod downloader;
pub mod errors;
mod metadata_structs;
pub mod parser;
pub mod schema;
mod utils;

pub use delete::delete;
pub use downloader::download_program_pairs;
pub use parser::parse;
