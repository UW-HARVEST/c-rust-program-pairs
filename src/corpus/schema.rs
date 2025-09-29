//! Parsed Metadata Schema
//!
//! This module defines data structures that represent the parsed output from
//! JSON metadata files. These structures store the actual metadata information
//! about program pairs after JSON parsing is complete. By contrast, structs
//! defined in file `metadata-structs.rs` are used during JSON parsing.

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
/// The metadata from a single .json metadata file, containing
/// an array of program pairs.
pub struct Metadata {
    pub pairs: Vec<ProgramPair>,
}

#[derive(Debug, Serialize, Deserialize)]
/// One C-Rust program pair.
pub struct ProgramPair {
    pub program_name: String,
    pub program_description: String,
    pub translation_tools: Vec<String>,
    pub feature_relationship: Features,
    pub c_program: Program,
    pub rust_program: Program,
}

#[derive(Debug, Serialize, Deserialize)]
/// One C or Rust program.
pub struct Program {
    pub language: Language,
    pub documentation_url: String,
    pub repository_url: String,
    pub source_paths: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
/// Specifies the feature set of the Rust project in relation to its C counterpart.
pub enum Features {
    RustSubsetOfC,
    RustEquivalentToC,
    RustSupersetOfC,
    Overlapping,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
/// The language in which the program is written.
pub enum Language {
    C,
    Rust,
}

impl Language {
    /// Converts the enum type to a string.
    ///
    /// # Returns
    ///
    /// The string "c" or "rust".
    pub fn to_str(&self) -> &'static str {
        match self {
            Language::C => "c",
            Language::Rust => "rust",
        }
    }
}
