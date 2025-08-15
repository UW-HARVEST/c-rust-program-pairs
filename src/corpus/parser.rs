//! # Metadata Parsing and Validation
//!
//! This module loads C-to-Rust metadata files, validates them against the
//! project's JSON schema, and converts them into a normalized [`Metadata`]
//! structure.
//!
//! The main entry point is [`parse`], which takes a path to a JSON metadata
//! file and returns a fully validated [`Metadata`] instance ready for use.

use std::{error::Error, fs, path::Path};

use jsonschema::validator_for;
use serde::Serialize;
use serde_json::Value;
use typify::import_types;

use crate::{
    corpus::schema::{Features, Language, Metadata, Program, ProgramPair},
    paths::METADATA_SCHEMA_FILE,
};

// Import automatically generated types from our metadata schema.
import_types!(schema = "metadata/metadata.schema.json");

/// Parses a JSON metadata file describing C-to-Rust program pairs into a
/// normalized [`Metadata`] struct.
///
/// Steps:
///
/// 1. Reads the JSON file from `path`.
/// 2. Deserializes it into a [`CToRustTranslationSchema`] enum.
/// 3. Validates it against the metadata JSON schema.
/// 4. Converts it into the [`Metadata`] format used throughout the project.
///
/// # Arguments
///
/// - `path` — Path to the JSON metadata file.
///
/// # Returns
///
/// A [`Metadata`] instance containing validated and normalized program pair data.
///
/// # Errors
///
/// Returns an error if:
/// - The file cannot be read.
/// - The JSON cannot be deserialized into [`CToRustTranslationSchema`].
/// - Schema validation fails.
///
/// # Example
/// ```no_run
/// use std::path::Path;
/// use mycrate::map::parse;
///
/// let metadata = parse(Path::new("metadata.json")).unwrap();
/// println!("Loaded {} program pairs", metadata.pairs.len());
/// ```
pub fn parse(path: &Path) -> Result<Metadata, Box<dyn Error>> {
    // Read metadata file.
    let raw_metadata = fs::read_to_string(path)?;
    let metadata: CToRustTranslationSchema = serde_json::from_str(&raw_metadata)?;

    // Validate metadata file with our JSON schema.
    validate_metadata(&metadata)?;

    // Create data structure conditioned on the metadata type.
    match metadata {
        CToRustTranslationSchema::IndividualPairsMetadata { pairs } => {
            let metadata = parse_individual(&pairs);
            return Ok(metadata);
        }
        CToRustTranslationSchema::ProjectPairsMetadata {
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
/// Serializes `metadata` to JSON and checks it against the schema in
/// [`METADATA_SCHEMA_FILE`].
///
/// # Errors
///
/// Returns an error if:
/// - The schema file cannot be read or parsed.
/// - The metadata cannot be serialized.
/// - Validation fails.
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

/// Parses an invidual-type metadata and returns a `Metadata` data structure.
///
/// # Arguments
///
/// - `pairs` - An array of `IndividualProgramPair` specified in the JSON schema.
///
/// # Returns
///
/// A `Metadata` data structure.
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

/// Parses an project-type metadata and returns a `Metadata` data structure.
///
/// # Arguments
///
/// - `pairs` - An array of `ProjectProgramPair` specified in the JSON schema.
///
/// # Returns
///
/// A `Metadata` data structure.
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

/// Helper function to convert from the `feature_relationship` field in
/// metadata files to the `Feature` enum used in our final schema.
///
/// # Arguments
///
/// - `relationship` - The enum representing the `feature_relationship` field.
///
/// # Returns
///
/// The corresponding `Feature` used in our final schema.
fn map_feature_relationship(relationship: FeatureRelationship) -> Features {
    match relationship {
        FeatureRelationship::RustSubsetOfC => Features::Subset,
        FeatureRelationship::RustSupersetOfC => Features::Superset,
        FeatureRelationship::RustEquivalentToC => Features::Equivalent,
        FeatureRelationship::Overlapping => Features::Overlapping,
    }
}
