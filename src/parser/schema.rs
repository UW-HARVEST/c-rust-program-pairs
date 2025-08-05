use serde::{Deserialize, Serialize};

// Our final, metadata data structure that we extract from individual /
// project metadata files.
#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub pairs: Vec<ProgramPair>,
}

// Contains information about each C-to-Rust program pair.
#[derive(Debug, Serialize, Deserialize)]
pub struct ProgramPair {
    pub program_name: String,
    pub program_description: String,
    pub translation_method: Translation,
    pub translation_tool: String,
    pub feature_relationship: Features,
    pub c_program: Program,
    pub rust_program: Program,
}

// Contains information about each C or Rust program.
#[derive(Debug, Serialize, Deserialize)]
pub struct Program {
    pub language: Language,
    pub documentation_url: String,
    pub repository_url: String,
    pub source_paths: Vec<String>,
}

// Specifies the method of translating from C to Rust.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Translation {
    Manual,
    Automatic,
}

// Specifies the features of the Rust project in relation to its C counterpart.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Features {
    Subset,
    Equivalent,
    Superset,
}

// Specifies the language used for the project.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    C,
    Rust,
}
