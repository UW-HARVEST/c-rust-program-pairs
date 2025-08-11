use std::{error::Error, fs, path::Path};

use jsonschema::validator_for;
use serde::Serialize;
use serde_json::Value;
use typify::import_types;

use crate::{
    parser::canonical::{Features, Language, Metadata, Program, ProgramPair},
    paths::METADATA_SCHEMA_FILE,
};

import_types!(schema = "metadata/metadata.schema.json");

/// Validates metadata against a JSON schema file.
///
/// Serializes the provided metadata to JSON and validates it against the schema
/// loaded from `METADATA_SCHEMA_FILE`.
///
/// # Arguments
/// * `metadata` - Any serializable type to validate
///
/// # Errors
/// Returns an error if the schema file can't be read, contains invalid JSON,
/// the metadata can't be serialized, or fails validation.
fn validate_metadata<T: Serialize>(metadata: &T) -> Result<(), Box<dyn Error>> {
    let schema_str = fs::read_to_string(METADATA_SCHEMA_FILE)?;
    let schema: Value = serde_json::from_str(&schema_str)?;
    let validator = validator_for(&schema)?;
    let metadata_json = serde_json::to_value(metadata)?;

    if let Err(error) = validator.validate(&metadata_json) {
        return Err(format!("Failed to validate metadata: {error}").into());
    }

    Ok(())
}

// Parses a project metadata file into a schema::Metadata object.
pub fn parse(path: &Path) -> Result<Metadata, Box<dyn Error>> {
    let raw_metadata = fs::read_to_string(path)?;
    let metadata: CToRustTranslationSchema = serde_json::from_str(&raw_metadata)?;

    // Validate metadata file with our JSON schema.
    validate_metadata(&metadata)?;

    match metadata {
        CToRustTranslationSchema::IndividualPairsMetadata { pairs } => {
            let metadata = parse_individual(&pairs)?;
            return Ok(metadata);
        }
        CToRustTranslationSchema::ProjectPairsMetadata {
            pairs,
            project_information,
        } => {
            let metadata = parse_project(&pairs, &project_information)?;
            return Ok(metadata);
        }
    }
}

fn parse_individual(pairs: &[IndividualProgramPair]) -> Result<Metadata, Box<dyn Error>> {
    let pairs: Vec<ProgramPair> = pairs
        .into_iter()
        .map(|pair| ProgramPair {
            program_name: pair.program_name.to_string(),
            program_description: pair.program_description.to_string(),
            translation_tools: pair.translation_tools.0.clone(),
            feature_relationship: match pair.feature_relationship {
                FeatureRelationship::RustSubsetOfC => Features::Subset,
                FeatureRelationship::RustSupersetOfC => Features::Superset,
                FeatureRelationship::RustEquivalentToC => Features::Equivalent,
                FeatureRelationship::Overlapping => Features::Overlapping,
            },
            c_program: Program {
                language: Language::C,
                documentation_url: pair.c_program.documentation_url.to_string(),
                repository_url: pair.c_program.repository_url.to_string(),
                source_paths: pair.c_program.source_paths.0.clone(),
            },
            rust_program: Program {
                language: Language::Rust,
                documentation_url: pair.rust_program.documentation_url.to_string(),
                repository_url: pair.rust_program.repository_url.to_string(),
                source_paths: pair.rust_program.source_paths.0.clone(),
            },
        })
        .collect();

    Ok(Metadata { pairs })
}

fn parse_project(
    pairs: &[ProjectProgramPair],
    project_information: &ProjectPairsMetadataProjectInformation,
) -> Result<Metadata, Box<dyn Error>> {
    let pairs: Vec<ProgramPair> = pairs
        .into_iter()
        .map(|pair| ProgramPair {
            program_name: pair.program_name.to_string(),
            program_description: pair.program_description.to_string(),
            translation_tools: project_information.translation_tools.0.clone(),
            feature_relationship: match project_information.feature_relationship {
                FeatureRelationship::RustSubsetOfC => Features::Subset,
                FeatureRelationship::RustSupersetOfC => Features::Superset,
                FeatureRelationship::RustEquivalentToC => Features::Equivalent,
                FeatureRelationship::Overlapping => Features::Overlapping,
            },
            c_program: Program {
                language: Language::C,
                documentation_url: project_information.c_program.documentation_url.to_string(),
                repository_url: project_information.c_program.repository_url.to_string(),
                source_paths: pair.c_program.source_paths.0.clone(),
            },
            rust_program: Program {
                language: Language::Rust,
                documentation_url: project_information
                    .rust_program
                    .documentation_url
                    .to_string(),
                repository_url: project_information.rust_program.repository_url.to_string(),
                source_paths: pair.rust_program.source_paths.0.clone(),
            },
        })
        .collect();

    Ok(Metadata { pairs })
}
