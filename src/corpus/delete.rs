//! # Delete Operations
//!
//! This module provides functionality for cleaning up downloaded program pairs
//! and repository clones.

use std::{fs, io::Error, path::Path};

use crate::paths::{PROGRAM_PAIRS_DIRECTORY, REPOSITORY_CLONES_DIRECTORY};

/// Removes all downloaded program-pairs and repository clones.
///
/// This deletes the directories specified by
/// [`PROGRAM_PAIRS_DIRECTORY`] and [`REPOSITORY_CLONES_DIRECTORY`],
/// along with all their contents, if they exist.
pub fn delete() -> Result<(), Error> {
    if Path::new(PROGRAM_PAIRS_DIRECTORY).exists() {
        fs::remove_dir_all(PROGRAM_PAIRS_DIRECTORY)?;
    };
    if Path::new(REPOSITORY_CLONES_DIRECTORY).exists() {
        fs::remove_dir_all(REPOSITORY_CLONES_DIRECTORY)?;
    };
    Ok(())
}
