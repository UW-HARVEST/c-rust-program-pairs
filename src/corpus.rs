//! # Corpus
//!
//! This module transforms metadata files into strongly-typed Rust structs.
//!
//! ## Submodules
//!
//! - [`downloader`] - Downloads program pairs form metadata/.
//! - [`metadata_structs`] - The Rust structs generated from `metadata.schema.json`.
//! - [`parser`] - Reads JSON files into data structures.
//! - [`schema`] - Defines the strongly-typed Rust data structures.
//! - [`utils`] - Utility functions.

pub mod downloader;
mod metadata_structs;
pub mod parser;
pub mod schema;
mod utils;

pub use downloader::download_metadata;
pub use parser::parse;
