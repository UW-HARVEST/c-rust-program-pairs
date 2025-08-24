//! # Utility Functions
//!
//! This module provides utility functions needed in other parts of our code.

use std::{error::Error, fs, path::Path};

use walkdir::WalkDir;

/// Count the number of files in a directory.
///
/// # Arguments
///
/// `directory` - A path to the directory.
///
/// # Returns
///
/// The number of files.
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

/// Extract a repository's name from its URL.
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
        .expect("Unreachable because split always returns at least 1 element");
    let name = last_segment.strip_suffix(".git").unwrap_or(last_segment);
    Ok(name.to_string())
}

/// Copies all .c, .h, and .rs files from a directory to the destination.
///
/// Copied files will all be directly under the destination directory; any
/// nested directories will not be copied.
///
/// # Arguments
///
/// - `source` - The source directory to copy files from.
/// - `destination` - The destination directory to copy files to.
///
/// # Returns
pub fn copy_files_from_directory(source: &Path, destination: &Path) -> Result<(), Box<dyn Error>> {
    // Create destination directory if it doesn't exist.
    fs::create_dir_all(destination)?;

    for entry in WalkDir::new(source).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            if let Some(extension) = path.extension() {
                let extension = extension
                    .to_str()
                    .expect("Failed to extract file extension");
                if matches!(extension, "c" | "h" | "rs") {
                    let filename = path.file_name().expect("Failed to extract file name");
                    fs::copy(path, destination.join(filename))?;
                }
            }
        }
    }

    Ok(())
}
