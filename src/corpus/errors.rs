//! # Error Types
//!
//! This module defines custom error types used throughout the [`corpus`] module.

use std::{io, path::PathBuf};

use reqwest;
use thiserror;

/// Errors that occur when a metadata file is being parsed.
#[derive(thiserror::Error, Debug)]
pub enum ParserError {
    /// Failed to read a file or directory.
    #[error("Failed to read '{path}': {error}")]
    IoRead {
        /// The path that could not be read.
        path: PathBuf,
        /// The underlying I/O error.
        #[source]
        error: io::Error,
    },

    /// Failed to deserialize some JSON string to Rust structs.
    #[error("Failed to deserialize to JSON: {error}")]
    Deserialize {
        /// The underlying deserialization error.
        #[source]
        error: serde_json::Error,
    },

    /// Failed to serialize some Rust struct to a JSON value.
    #[error("Failed to deserialize to JSON: {error}")]
    Serialize {
        /// The underlying serialization error.
        #[source]
        error: serde_json::Error,
    },

    /// Failed to validate some JSON schema.
    #[error("Failed to deserialize to JSON: {error}")]
    Validation {
        /// The underlying `jsonschema::ValidationError`.
        /// Type string because `ValidationError` requires lifetimes.
        error: String,
    },
}

/// Errors that occur in the Downloader program.
///
/// Some of these relate to downloads from a git repository.  Others relate
/// to local conditions such as file reading.
#[derive(thiserror::Error, Debug)]
pub enum DownloaderError {
    /// Failed to read a file or directory.
    #[error("Failed to read '{path}': {error}")]
    IoRead {
        /// The path that could not be read.
        path: PathBuf,
        /// The underlying I/O error.
        #[source]
        error: io::Error,
    },

    /// Failed to create a file or directory.
    #[error("Failed to create '{path}': {error}")]
    IoCreate {
        /// The path that could not be created.
        path: PathBuf,
        /// The underlying I/O error.
        #[source]
        error: io::Error,
    },

    /// Failed to copy a file or directory from `source` to `destination`.
    #[error("Failed to copy '{source}' to '{destination}': {error}")]
    IoCopy {
        /// The source file path.
        source: PathBuf,
        /// The destination file path.
        destination: PathBuf,
        /// The underlying I/O error.
        #[source]
        error: io::Error,
    },

    /// Failed to rename a file or directory from `old` to `new`.
    #[error("Failed to rename '{old}' to '{new}': {error}")]
    IoRename {
        /// The old file path.
        old: PathBuf,
        /// The new file path.
        new: PathBuf,
        /// The underlying I/O error.
        #[source]
        error: io::Error,
    },

    /// Generic I/O error.
    #[error("IO error: {0}")]
    Io(String),

    /// Fail to clone a git repository.
    #[error("Failed to clone repository '{repository_url}': {error}")]
    CloneRepository {
        /// The URL of the repository that failed to clone.
        repository_url: String,
        /// The underlying git error.
        #[source]
        error: git2::Error,
    },

    /// Fail to download a tarball.
    #[error("Failed to download tarball from '{tarball_url}': {error}")]
    TarballDownload {
        /// The URL used to download the tarball.
        tarball_url: String,
        /// The underlying HTTP request error.
        #[source]
        error: reqwest::Error,
    },

    /// Fail to read a tarball as bytes.
    #[error("Failed to read tarball body from '{tarball_url}': {error}")]
    TarballRead {
        /// The URL used to download the tarball.
        tarball_url: String,
        /// The underlying HTTP request error.
        #[source]
        error: reqwest::Error,
    },

    /// Fail to unpack a tarball.
    #[error("Failed to unpack tarball for '{repository_name}': {error}")]
    TarballUnpack {
        /// The name of the repository that is being unpacked.
        repository_name: String,
        /// The underlying HTTP request error.
        #[source]
        error: io::Error,
    },

    /// Fail to make an API request.
    #[error("Failed to make API request to '{url}': {error}")]
    ApiError {
        /// The name of the API URL.
        url: String,
        /// The underlying HTTP request error.
        #[source]
        error: reqwest::Error,
    },

    /// Failed to create a progress bar.
    #[error("Failed to create progress bar: {0}")]
    ProgressBar(String),
}
