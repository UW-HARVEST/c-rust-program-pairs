//! # Corpus
//!
//! This module transforms metadata files into strongly-typed Rust structs.
//!
//! ## Modules
//!
//! - [`downloader`] - Downloads program pairs form metadata/.
//! - [`parser`] - Reads JSON files into data structures.
//! - [`schema`] - Defines the strongly-typed Rust data structures.
//! - [`utils`] - Utility functions.

pub mod downloader;
pub mod parser;
pub mod schema;
mod utils;

pub use downloader::download_metadata;
pub use parser::parse;
