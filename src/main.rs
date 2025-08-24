//! # C-Rust program pair downloader
//!
//! This crate parses metadata files and downloads the C and Rust programs,
//! storing them in the `programs/` directory.
//!
//! ## Usage
//!
//! Running the binary will download the latest program-pairs for both
//! projects and individual metadata files into their respective directories.

mod corpus;
mod paths;

/// Entry point for the metadata downloader.
///
/// Downloads metadata for both projects and individual files into
/// the directories defined in [`paths`].
fn main() {
    corpus::download_metadata(true);
}
