//! # Paths
//!
//! This module defines constant file system paths used throughout the project.
//! All paths are relative to the project root unless otherwise specified.
//!
//! Centralizing paths here ensures consistency and makes it easier to update
//! directory structures in the future.

/// Path to the JSON schema describing the metadata format used to validate
/// metadata files.
pub const METADATA_SCHEMA_FILE: &str = "./metadata/metadata.schema.json";

/// Directory containing metadata for projects containing multiple programs
/// (e.g. coreutils).
pub const PROJECT_METADATA_DIRECTORY: &str = "./metadata/project";

/// Directory containing metadata for individual projects containing 1
/// program (e.g. git).
pub const INDIVIDUAL_METADATA_DIRECTORY: &str = "./metadata/individual";

/// Directory containing C to Rust program pairs.
pub const PROGRAMS_DIRECTORY: &str = "./programs";

/// Directory used as a local cache for repository data to avoid repeatedly
/// downloading repositories.
pub const REPOSITORY_CACHE_DIRECTORY: &str = "./repository_cache";
