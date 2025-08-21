//! # Program Pair Downloader
//!
//! This module helps with downloading our corpus of C-Rust program pairs.
//!
//! We downloading program-pairs from metadata files, for which
//! [`download_metadata`] is used to download all JSON metadata files in
//! metadata/.

use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

use fs_extra::dir::{self, CopyOptions};
use git2::{FetchOptions, RemoteCallbacks, Repository, build::RepoBuilder};
use indicatif::{ProgressBar, ProgressStyle};

use crate::{
    corpus::{
        self,
        schema::{Language, Metadata, ProgramPair},
    },
    paths::{
        INDIVIDUAL_METADATA_DIRECTORY, PROGRAMS_DIRECTORY, PROJECT_METADATA_DIRECTORY,
        REPOSITORY_CLONES_DIRECTORY,
    },
};

/// Download all metadata in `metadata/`.
///
/// We also have a progress bar to track the total number of metadata files
/// processed.
///
/// # Arguments
///
/// - `demo` - Specifies if we are running a demo, so we only download the
///            program-pairs specified in the metadata in `metadata/demo/`.
pub fn download_metadata(demo: bool) {
    let directories = if demo {
        vec![PathBuf::from("metadata/demo")]
    } else {
        vec![
            PathBuf::from(PROJECT_METADATA_DIRECTORY),
            PathBuf::from(INDIVIDUAL_METADATA_DIRECTORY),
        ]
    };

    let mut total_files = 0;
    for directory in &directories {
        // TODO: Better error handling.
        total_files += count_files(&directory).expect(&format!(
            "Failed to read directory: {}",
            directory.display()
        ));
    }

    let progress_bar = ProgressBar::new(total_files as u64);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.white} {bar:40.white/white} {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("##-"),
    );
    progress_bar.set_message(format!("Downloading program-pairs..."));

    for directory in &directories {
        download_from_metadata_directory(&directory, &progress_bar);
    }
}

/// Helper function to count the number of files in a directory.
///
/// # Arguments
///
/// `directory` - A path to the directory.
///
/// # Returns
///
/// A Result containing the number of files.
fn count_files(directory: &Path) -> Result<i32, Box<dyn Error>> {
    let mut total_files = 0;
    for file in directory.read_dir()? {
        let file = file?;
        if file.file_type()?.is_file() {
            total_files += 1;
        }
    }
    Ok(total_files)
}

/// Download all program pairs in metadata files from either
/// metadata/individual/ or metadata/projects.
///
/// We iterate through each metadata JSON file, then parse and download
/// the program pairs.
///
/// # Arguments
///
/// - `directory` - The directory containing the metadata JSON files.
/// - `progress_bar` - Update each time we finish downloading program pairs
///                    from each metadata file.
pub fn download_from_metadata_directory(directory: &Path, progress_bar: &ProgressBar) {
    for metadata_file in directory
        .read_dir()
        .expect(&format!("Failed to read: {}", directory.display()))
    {
        if let Ok(metadata_file) = metadata_file {
            let parsed_result = corpus::parse(&metadata_file.path());
            match parsed_result {
                Ok(metadata) => {
                    download_metadata_file(&metadata, &progress_bar);
                }
                Err(error) => println!(
                    "Failed to parse {:?}: {error}",
                    metadata_file.path().display()
                ),
            }
        }
    }
}

/// Downloads all program-pairs in a given Metadata object.
///
/// Note that we don't want to panic if we fail to download a program pair as
/// we would rather continue downloading the remaining pairs.
///
/// # Arguments
/// - `metadata` - Contains all program-pairs we want to download.
/// - `progress_bar` - Update each time we finish downloading program pairs
///                    from each metadata file.
fn download_metadata_file(metadata: &Metadata, progress_bar: &ProgressBar) {
    for pair in metadata.pairs.iter() {
        if let Err(error) = download_program_pair(pair) {
            println!("Failed to download {}: {}", pair.program_name, error)
        };
    }
    progress_bar.inc(1);
}

/// Downloads a C-Rust program pair.
///
/// Check if the C and Rust repositories exist, and clone them if they don't
/// Copy the C source files to programs/<program_name>/c-program.
/// Copy the Rust source files to programs/<program_name>/rust-program.
///
/// # Arguments
///
/// - `pair` - A program pair.
fn download_program_pair(pair: &ProgramPair) -> Result<(), Box<dyn Error>> {
    let program_name = &pair.program_name;
    let base_program_path = Path::new(PROGRAMS_DIRECTORY).join(program_name);
    let c_program_path = base_program_path.join("c-program");
    let rust_program_path = base_program_path.join("rust-program");

    fs::create_dir_all(&c_program_path)?;
    fs::create_dir_all(&rust_program_path)?;

    download_files(
        program_name,
        Language::C,
        &c_program_path,
        &pair.c_program.repository_url,
        &pair.c_program.source_paths,
    )?;
    download_files(
        program_name,
        Language::Rust,
        &rust_program_path,
        &pair.rust_program.repository_url,
        &pair.rust_program.source_paths,
    )?;

    Ok(())
}

/// Downloads the specified source files from a Git repository.
///
/// This function clones the repository (if not already cached) into
/// `repository_clones/<language>/<repository_name>`, then copies the listed
/// `source_files` into the given `program_directory`.
///
/// A progress bar is displayed on standard output to track cloning and copying.
///
/// Side effects:
///
/// - Creates cache and destination directories if they don’t exist.
/// - May overwrite files in the destination.
///
/// # Arguments
///
/// - `program_name` — Name of the program being downloaded (used for progress messages).
/// - `program_language` — Language of the program (affects repository cache path).
/// - `program_directory` — Destination directory for the downloaded source files.
/// - `repository_url` — Git URL of the repository to clone.
/// - `source_files` — Paths (relative to repo root) of files or directories to copy.
///
/// # Returns
///
/// Returns `Ok(())` if all files were successfully downloaded and copied.
fn download_files(
    program_name: &str,
    program_language: Language,
    program_directory: &Path,
    repository_url: &str,
    source_files: &[String],
) -> Result<(), Box<dyn Error>> {
    let repository_cache_path =
        Path::new(REPOSITORY_CLONES_DIRECTORY).join(program_language.to_str());
    let repository_name = get_repository_name(repository_url)?;

    // Create a progress bar.
    let progress_bar = ProgressBar::new(100);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.white} {bar:40.white/white} {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("##-"),
    );
    progress_bar.set_message(format!("Cloning repository {repository_name}..."));

    // Set up remote callbacks for progress tracking.
    let mut remote_callbacks = RemoteCallbacks::new();
    remote_callbacks.transfer_progress(|progress: git2::Progress| {
        update_progress_bar_callback(progress, &repository_name, &progress_bar)
    });

    // Check if repository exists in cache, if not clone it.
    // We store repositories in repository_cache/<language>/<repository_name>.
    let repository = match Repository::open(repository_cache_path.join(&repository_name)) {
        Ok(repository) => repository,
        Err(_) => {
            // Setup fetch options with our callbacks.
            let mut fetch_options = FetchOptions::new();
            fetch_options.remote_callbacks(remote_callbacks);

            // Clone only the latest commit to save time and space.
            fetch_options.depth(1);

            // Clone the repository.
            let mut builder = RepoBuilder::new();
            builder.fetch_options(fetch_options);
            builder.clone(
                repository_url,
                &repository_cache_path.join(&repository_name),
            )?
        }
    };

    progress_bar.set_style(ProgressStyle::default_spinner());
    progress_bar.set_message("Copying files...");

    // Define options used when copying directories.
    let copy_options = CopyOptions::new();

    // Copy given files from the repository to the given directory.
    let repository_directory = repository
        .workdir()
        .ok_or(format!("Failed to find repository: {repository_name}"))?;
    for file_path in source_files.iter() {
        let file_name = Path::new(file_path)
            .file_name()
            .ok_or(format!("Invalid file path: {}", file_path))?;
        let source = repository_directory.join(&file_path);
        let destination = program_directory.join(file_name);

        // Copy files from destination to source.
        if source.is_dir() {
            dir::create_all(&destination, false)?;
            dir::copy(&source, &destination, &copy_options)?;
        } else {
            fs::copy(source, destination)?;
        }
    }

    progress_bar.finish_with_message(format!(
        "Downloaded {} ({}).",
        program_name,
        program_language.to_str()
    ));
    Ok(())
}

/// Callback used to update the progress bar as a repository is cloned.
///
/// # Arguments
///
/// - `repository_name` - The repository being cloned.
/// - `progress` - Contains information about the current status of the download.
/// - `progress_bar` - The progress bar to update.
///
/// # Returns
///
/// The callback function must return `true`.
fn update_progress_bar_callback(
    progress: git2::Progress,
    repository_name: &str,
    progress_bar: &ProgressBar,
) -> bool {
    let received_objects = progress.received_objects();
    let received_bytes = progress.received_bytes();
    let total_objects = progress.total_objects();
    let indexed_objects = progress.indexed_objects();

    // Downloading objects.
    if received_objects < total_objects {
        progress_bar.set_length(total_objects as u64);
        progress_bar.set_position(received_objects as u64);
        progress_bar.set_message(format!(
            "Cloning {repository_name}: ({received_bytes:.1} B)"
        ));
    }
    // Processing downloaded objects.
    else if indexed_objects < total_objects {
        progress_bar.set_position(indexed_objects as u64);
        progress_bar.set_message(format!(
            "Indexing objects: {indexed_objects}/{total_objects}"
        ));
    }
    // Done with objects.
    else {
        progress_bar.set_message("Resolving deltas...");
    }

    true
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
fn get_repository_name(url: &str) -> Result<String, Box<dyn Error>> {
    let last_segment = url
        .trim_end_matches('/')
        .split('/')
        .last()
        .expect("Error is unreachable since split always returns at least 1 element");
    let name = last_segment.strip_suffix(".git").unwrap_or(last_segment);
    Ok(name.to_string())
}
