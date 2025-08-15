//! # C-to-Rust Metadata Downloader
//!
//! This crate provides utilities for downloading and parsing metadata files
//! used in the C-to-Rust conversion research project. It supports both
//! project-level and individual-file metadata, storing them in the programs directory
//! for further analysis.
//!
//! ## Modules
//!
//! - [`corpus`] — Manages the corpus of C to Rust program pairs.
//! - [`paths`] — Defines filesystem paths used for metadata storage.
//!
//! ## Usage
//!
//! Running the binary will download the latest metadata for both
//! projects and individual files into their respective directories.

mod corpus;
mod paths;

#[cfg(test)]
mod tests {
    use crate::{
        corpus,
        paths::{INDIVIDUAL_METADATA_DIRECTORY, PROJECT_METADATA_DIRECTORY},
    };

    use std::path::Path;

    // Tests that a project-metadata file can be successfully parsed.
    #[test]
    fn test_parse_project() {
        let metadata_file = Path::new(PROJECT_METADATA_DIRECTORY).join("diffutils.json");
        let result = corpus::parse(&metadata_file);
        assert!(
            result.is_ok(),
            "Failed to parse project metadata: {:?}",
            result.err()
        );
    }

    // Tests that an individual-metadata file can be successfully parsed.
    #[test]
    fn test_parse_individual() {
        let metadata_file = Path::new(INDIVIDUAL_METADATA_DIRECTORY).join("system-tools.json");
        let result = corpus::parse(&metadata_file);
        assert!(
            result.is_ok(),
            "Failed to parse individual metadata: {:?}",
            result.err()
        );
    }
}

/// Entry point for the metadata downloader.
///
/// Downloads metadata for both projects and individual files into
/// the directories defined in [`paths`].
fn main() {
    corpus::download_metadata();
}
