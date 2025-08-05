use crate::{
    parser::schema::{Features, Language, Metadata, Program, ProgramPair, Translation},
    paths::METADATA_SCHEMA_FILE,
};

use jsonschema::validator_for;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{error::Error, fs, path::Path};

// Schema for project metadata files.
#[derive(Debug, Serialize, Deserialize)]
struct ProjectMetadata {
    project_information: ProjectInformation,
    pairs: Vec<ProjectProgramPair>,
}

// Global information about the project that apply to every program pair.
#[derive(Debug, Serialize, Deserialize)]
struct ProjectInformation {
    program_name: String,
    translation_method: Translation,
    translation_tool: String,
    feature_relationship: Features,
    c_program: ProjectGlobalProgram,
    rust_program: ProjectGlobalProgram,
}

// Global information that applies to each specific C / Rust program.
#[derive(Debug, Serialize, Deserialize)]
struct ProjectGlobalProgram {
    documentation_url: String,
    repository_url: String,
}

// Specific information for each individual program pair.
#[derive(Debug, Serialize, Deserialize)]
struct ProjectProgramPair {
    program_name: String,
    program_description: String,
    c_program: ProjectProgram,
    rust_program: ProjectProgram,
}

// Specific information for each individual C / Rust program in a pair.
#[derive(Debug, Serialize, Deserialize)]
struct ProjectProgram {
    source_paths: Vec<String>,
}

// Parses a project metadata file into a schema::Metadata object.
pub fn parse(path: &Path) -> Result<Metadata, Box<dyn Error>> {
    let raw_metadata = fs::read_to_string(path)?;
    let project_metadata: ProjectMetadata = serde_json::from_str(&raw_metadata)?;

    // Validate metadata file with our JSON schema.
    let schema_str = fs::read_to_string(METADATA_SCHEMA_FILE)?;
    let schema: Value = serde_json::from_str(&schema_str)?;
    let validator = validator_for(&schema)?;
    let project_metadata_json = serde_json::to_value(&project_metadata)?;

    if let Err(error) = validator.validate(&project_metadata_json) {
        return Err(format!("Failed to validate metadata: {error}").into());
    }

    // Parse metadata into our program-pair data structure.
    let global_metadata = project_metadata.project_information;
    let pairs: Vec<ProgramPair> = project_metadata
        .pairs
        .into_iter()
        .map(|pair| ProgramPair {
            program_name: pair.program_name,
            program_description: pair.program_description,
            translation_method: global_metadata.translation_method.clone(),
            translation_tool: global_metadata.translation_tool.clone(),
            feature_relationship: global_metadata.feature_relationship.clone(),
            c_program: Program {
                language: Language::C,
                documentation_url: global_metadata.c_program.documentation_url.clone(),
                repository_url: global_metadata.c_program.repository_url.clone(),
                source_paths: pair.c_program.source_paths,
            },
            rust_program: Program {
                language: Language::Rust,
                documentation_url: global_metadata.rust_program.documentation_url.clone(),
                repository_url: global_metadata.rust_program.repository_url.clone(),
                source_paths: pair.rust_program.source_paths,
            },
        })
        .collect();

    Ok(Metadata { pairs })
}
