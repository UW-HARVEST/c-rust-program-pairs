//! # C-Rust program pair downloader
//!
//! This crate parses metadata files and downloads the C and Rust programs,
//! storing them in the `programs/` directory.

mod corpus;
mod paths;

/// Entry point for the metadata downloader.
///
/// Downloads metadata for both projects and individual files into
/// the directories defined in [`paths`].
fn main() {
    corpus::download_metadata(true);
}
