//! # Metadata Parsing and Validation
//!
//! The main entry point is [`parse`], which takes a path to a JSON metadata
//! file and returns a [`Metadata`] instance.

use std::{
    fs,
    path::{Path, PathBuf},
};

use jsonschema;
use serde::Serialize;
use serde_json::Value;

use crate::{
    corpus::{
        errors::ParserError,
        metadata_structs::{
            CRustProgramPairSchema, FeatureRelationship, IndividualProgramPair,
            ProjectPairsMetadataProjectInformation, ProjectProgramPair,
        },
        schema::{Features, Language, Metadata, Program, ProgramPair},
    },
    paths::METADATA_SCHEMA_FILE,
};

/// Parses a JSON metadata file describing C-Rust program pairs into a
/// [`Metadata`] struct.
///
/// # Arguments
///
/// - `path` - The JSON metadata file.
///
/// # Returns
///
/// A [`Metadata`] instance containing program pair data on success and
/// [`ParserError`] on failure.
pub fn parse(path: &Path) -> Result<Metadata, ParserError> {
    // Read metadata file and deserialize it into a
    // [`CRustProgramPairSchema`] enum.
    let raw_metadata = fs::read_to_string(path).map_err(|error| ParserError::IoRead {
        path: path.to_path_buf(),
        error,
    })?;
    let metadata: CRustProgramPairSchema =
        serde_json::from_str(&raw_metadata).map_err(|error| ParserError::Deserialize { error })?;

    // Validate metadata with our JSON schema.
    validate_metadata(&metadata)?;

    // Create data structure conditioned on the metadata type.
    match metadata {
        CRustProgramPairSchema::IndividualPairsMetadata { pairs } => {
            let metadata = parse_individual(&pairs);
            return Ok(metadata);
        }
        CRustProgramPairSchema::ProjectPairsMetadata {
            pairs,
            project_information,
        } => {
            let metadata = parse_project(&pairs, &project_information);
            return Ok(metadata);
        }
    }
}

/// Validates metadata against the project's JSON schema.
///
/// # Arguments
///
/// - `metadata` - A JSON serializable struct that represents some metadata.
///
/// # Returns
///
/// Returns `Ok(())` on success and [`ParserError`] on failure.
fn validate_metadata<T: Serialize>(metadata: &T) -> Result<(), ParserError> {
    // Create a validator based on the JSON schema.
    let schema_str =
        fs::read_to_string(METADATA_SCHEMA_FILE).map_err(|error| ParserError::IoRead {
            path: PathBuf::from(METADATA_SCHEMA_FILE),
            error,
        })?;
    let schema: Value =
        serde_json::from_str(&schema_str).map_err(|error| ParserError::Deserialize { error })?;
    let validator =
        jsonschema::validator_for(&schema).map_err(|error| ParserError::Validation {
            error: error.to_string(),
        })?;

    // Convert metadata to a JSON `Value` type.
    let metadata_json =
        serde_json::to_value(metadata).map_err(|error| ParserError::Serialize { error })?;

    if let Err(error) = validator.validate(&metadata_json) {
        return Err(ParserError::Validation {
            error: format!("Failed to validate metadata: {error}"),
        });
    }

    Ok(())
}

/// Parses an individual-type metadata and returns a [`Metadata`] data structure.
///
/// # Arguments
///
/// - `pairs` - An array of [`IndividualProgramPair`] specified in the JSON schema.
///
/// # Returns
///
/// A [`Metadata`] data structure.
fn parse_individual(pairs: &[IndividualProgramPair]) -> Metadata {
    let pairs: Vec<ProgramPair> = pairs
        .into_iter()
        .map(|pair| ProgramPair {
            program_name: pair.program_name.to_string(),
            program_description: pair.program_description.to_string(),
            translation_tools: pair.translation_tools.0.clone(),
            feature_relationship: map_feature_relationship(pair.feature_relationship),
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

    Metadata { pairs }
}

/// Parses an project-type metadata and returns a [`Metadata`] data structure.
///
/// # Arguments
///
/// - `pairs` - An array of [`ProjectProgramPair`] specified in the JSON schema.
///
/// # Returns
///
/// A [`Metadata`] data structure.
fn parse_project(
    pairs: &[ProjectProgramPair],
    project_information: &ProjectPairsMetadataProjectInformation,
) -> Metadata {
    let pairs: Vec<ProgramPair> = pairs
        .into_iter()
        .map(|pair| ProgramPair {
            program_name: pair.program_name.to_string(),
            program_description: pair.program_description.to_string(),
            translation_tools: project_information.translation_tools.0.clone(),
            feature_relationship: map_feature_relationship(
                project_information.feature_relationship,
            ),
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

    Metadata { pairs }
}

/// Convert from the `feature_relationship` field in
/// metadata files to the `Feature` enum used in our final schema.
///
/// # Arguments
///
/// - `relationship` - The enum representing the `feature_relationship` field.
///
/// # Returns
///
/// The corresponding [`Feature`] used in our final schema.
fn map_feature_relationship(relationship: FeatureRelationship) -> Features {
    match relationship {
        FeatureRelationship::RustSubsetOfC => Features::RustSubsetOfC,
        FeatureRelationship::RustSupersetOfC => Features::RustSupersetOfC,
        FeatureRelationship::RustEquivalentToC => Features::RustEquivalentToC,
        FeatureRelationship::Overlapping => Features::Overlapping,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::paths::{INDIVIDUAL_METADATA_DIRECTORY, PROJECT_METADATA_DIRECTORY};

    use std::path::Path;

    /// Tests that a project-metadata file can be successfully parsed.
    #[test]
    fn test_parse_project() {
        let metadata_file = Path::new(PROJECT_METADATA_DIRECTORY).join("diffutils.json");
        let result = parse(&metadata_file);
        assert!(
            result.is_ok(),
            "Failed to parse project metadata: {:?}",
            result.err()
        );
    }

    /// Tests that an individual-metadata file can be successfully parsed.
    #[test]
    fn test_parse_individual() {
        let metadata_file = Path::new(INDIVIDUAL_METADATA_DIRECTORY).join("system-tools.json");
        let result = parse(&metadata_file);
        assert!(
            result.is_ok(),
            "Failed to parse individual metadata: {:?}",
            result.err()
        );
    }
}
