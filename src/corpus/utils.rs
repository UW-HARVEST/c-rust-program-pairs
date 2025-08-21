//! # Utility Functions
//!
//! This module provides utility functions needed in other parts of our code.

use std::{error::Error, path::Path};

/// Helper function to count the number of files in a directory.
///
/// # Arguments
///
/// `directory` - A path to the directory.
///
/// # Returns
///
/// A Result containing the number of files.
pub fn count_files(directory: &Path) -> Result<i32, Box<dyn Error>> {
    let mut total_files = 0;
    for file in directory.read_dir()? {
        let file = file?;
        if file.file_type()?.is_file() {
            total_files += 1;
        }
    }
    Ok(total_files)
}

/// Helper function to extract a repository's name from its URL.
///
/// # Arguments
///
/// - `url` - Git repository URL; must point to a valid, accessible repo.
///
/// # Returns
///
/// The name of the repository.
pub fn get_repository_name(url: &str) -> Result<String, Box<dyn Error>> {
    let last_segment = url
        .trim_end_matches('/')
        .split('/')
        .last()
        .expect("Error is unreachable since split always returns at least 1 element");
    let name = last_segment.strip_suffix(".git").unwrap_or(last_segment);
    Ok(name.to_string())
}
