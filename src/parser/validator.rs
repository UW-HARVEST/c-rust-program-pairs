use std::{error::Error, fs};

use jsonschema::validator_for;
use serde::Serialize;
use serde_json::Value;

use crate::paths::METADATA_SCHEMA_FILE;

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
pub fn validate_metadata<T: Serialize>(metadata: &T) -> Result<(), Box<dyn Error>> {
    let schema_str = fs::read_to_string(METADATA_SCHEMA_FILE)?;
    let schema: Value = serde_json::from_str(&schema_str)?;
    let validator = validator_for(&schema)?;
    let metadata_json = serde_json::to_value(metadata)?;

    if let Err(error) = validator.validate(&metadata_json) {
        return Err(format!("Failed to validate metadata: {error}").into());
    }

    Ok(())
}
