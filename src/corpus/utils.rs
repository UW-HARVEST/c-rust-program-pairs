//! # Utility Functions
//!
//! This module provides utility functions used in other parts of our code.

use std::{
    fs,
    path::{Path, MAIN_SEPARATOR_STR},
};

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

/// Copies all .c, .h, and .rs files from a directory to the destination.
///
/// Copied files will all be directly under the destination directory;
/// nested directories will not be copied. Files will have their paths
/// included in their name. For example, a file found in a subdirectory
/// "module/file.txt" will have a final name of module-file.txt.
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

    // Iterate recursively through every file in `source`.
    for entry in WalkDir::new(source).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            if let Some(extension) = path.extension() {
                let extension = extension.to_str().ok_or_else(|| {
                    DownloaderError::Io("Failed to retrieve file extension".to_string())
                })?;

                // Copy all `.c`, `.h`, and `.rs` files.
                if matches!(extension, "c" | "h" | "rs") {
                    // Include full path as filename but replace path
                    // separator with '-' in filename.
                    let relative_path = path.strip_prefix(source).unwrap_or(path);
                    let filename = relative_path
                        .to_str()
                        .ok_or_else(|| {
                            DownloaderError::Io(format!(
                                "Failed to get filename for '{}'",
                                path.display()
                            ))
                        })?
                        .replace(MAIN_SEPARATOR_STR, "-");
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

/// Extract a repository's name from its URL.
///
/// # Example
///
/// The repository name of
/// "https://github.com/eza-community/eza.git" is "eza".
///
/// # Arguments
///
/// - `url` - Git repository URL; must point to a valid, accessible repo.
///
/// # Returns
///
/// The name of the repository on success or [`DownloaderError`] on failure.
pub fn get_repository_name(url: &str) -> Result<String, DownloaderError> {
    let last_segment = url
        .trim_end_matches('/')
        .split('/')
        .last()
        .expect("Unreachable because split always returns at least 1 element");
    let name = last_segment.strip_suffix(".git").unwrap_or(last_segment);
    Ok(name.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Tests that a repository name can be extracted from a URL.
    fn test_get_repository_name() {
        assert_eq!(
            "eza",
            get_repository_name("https://github.com/eza-community/eza.git").unwrap()
        );
        assert_eq!(
            "eza",
            get_repository_name("https://github.com/eza-community/eza").unwrap()
        );
    }
}
