use std::{error::Error, fs, path::Path};

use serde::{Deserialize, Serialize};

use crate::parser::{
    canonical::{Features, Language, Metadata, Program, ProgramPair},
    validator,
};

// Schema for individual metadata files.
#[derive(Debug, Serialize, Deserialize)]
struct IndividualMetadata {
    pairs: Vec<IndividualProgramPair>,
}

// Contains information for each program pair.
#[derive(Debug, Serialize, Deserialize)]
struct IndividualProgramPair {
    program_name: String,
    program_description: String,
    translation_tools: Vec<String>,
    feature_relationship: Features,
    c_program: IndividualProgram,
    rust_program: IndividualProgram,
}

// Contains information for each C / Rust program in each pair.
#[derive(Debug, Serialize, Deserialize)]
struct IndividualProgram {
    documentation_url: String,
    repository_url: String,
    source_paths: Vec<String>,
}

// Parses an individual metadata file into a schema::Metadata object.
pub fn parse(path: &Path) -> Result<Metadata, Box<dyn Error>> {
    let raw_metadata = fs::read_to_string(path)?;
    let individual_metadata: IndividualMetadata = serde_json::from_str(&raw_metadata)?;

    // Validate metadata file with our JSON schema.
    validator::validate_metadata(&individual_metadata)?;

    // Parse metadata into our program-pair data structure.
    let pairs: Vec<ProgramPair> = individual_metadata
        .pairs
        .into_iter()
        .map(|pair| ProgramPair {
            program_name: pair.program_name,
            program_description: pair.program_description,
            translation_tools: pair.translation_tools,
            feature_relationship: pair.feature_relationship,
            c_program: Program {
                language: Language::C,
                documentation_url: pair.c_program.documentation_url,
                repository_url: pair.c_program.repository_url,
                source_paths: pair.c_program.source_paths,
            },
            rust_program: Program {
                language: Language::Rust,
                documentation_url: pair.rust_program.documentation_url,
                repository_url: pair.rust_program.repository_url,
                source_paths: pair.rust_program.source_paths,
            },
        })
        .collect();

    Ok(Metadata { pairs })
}
