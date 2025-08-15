//! # Corpus
//!
//! This module transforms metadata files into strongly-typed Rust structs.
//!
//! ## Modules
//!
//! - [`downloader`] — Downloads program pairs form metadata/.
//! - [`parser`] — Reads JSON files into data structures.
//! - [`schema`] — Defines the strongly-typed Rust data structures.

pub mod downloader;
pub mod parser;
pub mod schema;

pub use downloader::download_metadata;
pub use parser::parse;
