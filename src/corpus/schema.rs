//! Metadata Schema
//!
//! This module defines data structures that store information about program pairs.

use serde::{Deserialize, Serialize};

/// The metadata from a single .json metadata file, containing
/// an array of program pairs.
#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub pairs: Vec<ProgramPair>,
}

/// One C-Rust program pair.
#[derive(Debug, Serialize, Deserialize)]
pub struct ProgramPair {
    pub program_name: String,
    pub program_description: String,
    pub translation_tools: Vec<String>,
    pub feature_relationship: Features,
    pub c_program: Program,
    pub rust_program: Program,
}

/// One C or Rust program.
#[derive(Debug, Serialize, Deserialize)]
pub struct Program {
    pub language: Language,
    pub documentation_url: String,
    pub repository_url: String,
    pub source_paths: Vec<String>,
}

/// Specifies the feature set of the Rust project in relation to its C counterpart.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Features {
    #[serde(rename = "rust_subset_of_c")]
    Subset,
    #[serde(rename = "rust_equivalent_to_c")]
    Equivalent,
    #[serde(rename = "rust_superset_of_c")]
    Superset,
    #[serde(rename = "overlapping")]
    Overlapping,
}

/// The language in which the program is written.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
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
