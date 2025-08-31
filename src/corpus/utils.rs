//! # Utility Functions
//!
//! This module provides utility functions used in other parts of our code.

use std::{fs, path::Path};

use walkdir::WalkDir;

use crate::corpus::errors::DownloaderError;

/// Count the number of immediate files in a directory, not including any
/// files in sub-directories.
///
/// # Arguments
///
/// `directory` - A path to the directory.
///
/// # Returns
///
/// The number of immediate files in the directory, or a [`DownloaderError`]
/// if the operation fails.
pub fn count_files(directory: &Path) -> Result<usize, DownloaderError> {
    let entries = directory
        .read_dir()
        .map_err(|error| DownloaderError::IoRead {
            path: directory.to_path_buf(),
            error,
        })?;

    let mut total_files = 0;
    for entry in entries {
        let entry = entry.map_err(|error| DownloaderError::IoRead {
            path: directory.to_path_buf(),
            error,
        })?;

        let file_type = entry.file_type().map_err(|error| DownloaderError::IoRead {
            path: directory.to_path_buf(),
            error,
        })?;

        if file_type.is_file() {
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
/// The name of the repository on success or [`DownloaderError`] on failure.
///
/// # Example
///
/// ```rust
/// let url = "https://github.com/eza-community/eza.git";
/// let name = get_repository_name = get_repository_name(url);
/// assert_eq!(name, "eza");
/// ```
pub fn get_repository_name(url: &str) -> Result<String, DownloaderError> {
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
///
/// Returns `Ok(())` on success and [`DownloaderError`] on failure.
pub fn copy_files_from_directory(source: &Path, destination: &Path) -> Result<(), DownloaderError> {
    // Create destination directory in case it doesn't exist.
    fs::create_dir_all(destination).map_err(|error| DownloaderError::IoCopy {
        source: source.to_path_buf(),
        destination: destination.to_path_buf(),
        error,
    })?;

    for entry in WalkDir::new(source).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            if let Some(extension) = path.extension() {
                let extension = extension.to_str().ok_or_else(|| {
                    DownloaderError::Io("Failed to retrieve file extension".to_string())
                })?;

                // Copy all `.c`, `.h`, and `.rs` files.
                if matches!(extension, "c" | "h" | "rs") {
                    let filename = path.file_name().expect("Failed to extract file name");
                    fs::copy(path, destination.join(filename)).map_err(|error| {
                        DownloaderError::IoCopy {
                            source: source.to_path_buf(),
                            destination: destination.to_path_buf(),
                            error,
                        }
                    })?;
                }
            }
        }
    }

    Ok(())
}
