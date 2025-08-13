//! # Metadata
//!
//! Loads, validates, and transforms metadata files into strongly-typed Rust
//! structures for use in the application.
//!
//! ## Modules
//!
//! - [`parser`] — Reads and deserializes raw JSON files into intermediate structures.
//! - [`schema`] — Defines the final strongly-typed Rust structures.

pub mod parser;
pub mod schema;

pub use parser::parse;
