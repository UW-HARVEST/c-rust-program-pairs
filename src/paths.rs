//! # Paths
//!
//! This module defines file system paths used throughout the project.
//! All paths are relative to the project root.

/// Path to the JSON schema for metadata files.
pub const METADATA_SCHEMA_FILE: &str = "metadata/metadata.schema.json";

/// Directory containing metadata files for projects containing multiple
/// programs (e.g. coreutils).
pub const PROJECT_METADATA_DIRECTORY: &str = "metadata/project";

/// Directory containing metadata files for individual projects containing 1
/// program (e.g. git).
pub const INDIVIDUAL_METADATA_DIRECTORY: &str = "metadata/individual";

/// Directory containing a small subset of the total metadata files; used
/// for demo and testing purposes.
pub const DEMO_METADATA_DIRECTORY: &str = "metadata/demo";

/// Directory containing C-Rust program pairs.
pub const PROGRAMS_DIRECTORY: &str = "programs";

/// Directory used as a local cache for repository clones to avoid repeatedly
/// cloning repositories.
pub const REPOSITORY_CLONES_DIRECTORY: &str = "repository_clones";
