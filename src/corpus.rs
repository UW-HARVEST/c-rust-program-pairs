//! # Corpus
//!
//! This module handles the loading, validation, and transformation  of
//! metadata files into strongly-typed Rust structures for use.
//!
//! ## Modules
//!
//! - [`downloader`] — Downloads program pairs form metadata/.
//! - [`parser`] — Reads and deserializes raw JSON files into intermediate structures.
//! - [`schema`] — Defines the final strongly-typed Rust structures.

pub mod downloader;
pub mod parser;
pub mod schema;

pub use downloader::download_metadata;
pub use parser::parse;
