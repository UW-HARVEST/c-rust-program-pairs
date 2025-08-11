use serde::{Deserialize, Serialize};

// Indicates the type of metadata file.
pub enum MetadataType {
    Individual,
    Project,
}

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
    pub translation_tools: Vec<String>,
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

// Specifies the features of the Rust project in relation to its C counterpart.
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

// Specifies the language used for the project.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    C,
    Rust,
}
