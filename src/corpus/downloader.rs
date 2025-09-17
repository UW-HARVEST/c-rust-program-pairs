//! # Program Pair Downloader
//!
//! This module downloads our corpus of C-Rust program pairs.
//!
//! First it reads program pairs from metadata files.  Then it
//! downloads all program pairs from the
//! repository URLs provided in the metadata.

use std::{
    fs,
    path::{Path, PathBuf},
};

use git2::{ConfigLevel, FetchOptions, RemoteCallbacks, Repository, build::RepoBuilder, opts};
use indicatif::{ProgressBar, ProgressStyle};

use crate::{
    corpus::{
        self,
        errors::DownloaderError,
        schema::{Language, Metadata, ProgramPair},
        utils,
    },
    paths::{
        DEMO_METADATA_DIRECTORY, INDIVIDUAL_METADATA_DIRECTORY, PROGRAM_PAIRS_DIRECTORY,
        PROJECT_METADATA_DIRECTORY, REPOSITORY_CLONES_DIRECTORY,
    },
};

/// Reads all metadata files in `metadata/` and downloads all program pairs.
///
/// A progress bar tracks the number of metadata files processed.
///
/// # Arguments
///
/// - `demo` - True if a demo is being run, in which case the function downloads
///            only the program pairs specified `metadata/demo/`.
///
/// # Returns
///
/// Returns `Ok(())` on success, or a [`DownloaderError`] if any step fails.
pub fn download_metadata(demo: bool) -> Result<(), DownloaderError> {
    // Temporarily override the user's global and system Git configuration.
    // This is to ensure reliability when using the clone operation to
    // download repositories.
    unsafe {
        opts::set_search_path(ConfigLevel::Global, "/dev/null").unwrap();
        opts::set_search_path(ConfigLevel::System, "/dev/null").unwrap();
    }

    let directories = if demo {
        vec![PathBuf::from(DEMO_METADATA_DIRECTORY)]
    } else {
        vec![
            PathBuf::from(PROJECT_METADATA_DIRECTORY),
            PathBuf::from(INDIVIDUAL_METADATA_DIRECTORY),
        ]
    };

    // Count total metadata files in the directories being processed.
    let mut total_files = 0;
    for directory in &directories {
        total_files += utils::count_files(&directory)?;
    }

    // Create a progress bar to track the number of metadata files that have
    // been processed.
    let progress_bar = ProgressBar::new(total_files as u64);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("{bar:40.white/white} {pos}/{len} {msg}")
            .map_err(|error| DownloaderError::ProgressBar(error.to_string()))?
            .progress_chars("##-"),
    );
    progress_bar.set_message(format!("Processing metadata files..."));

    for directory in &directories {
        download_from_metadata_directory(&directory, &progress_bar)?;
    }

    progress_bar.finish_with_message("Downloaded all program pairs!");
    Ok(())
}

/// Download program pairs in the given metadata files.
///
/// The program iterates through each metadata JSON file, then parses and
/// downloads the program pairs.
///
/// # Arguments
///
/// - `directory` - The directory containing the metadata JSON files,
///   typically `metadata/individual/` or `metadata/projects/`.
/// - `progress_bar` - Update each time a metadata file is processed.
///
/// # Returns
///
/// Returns `Ok(())` on success, or a [`DownloaderError`] if directory reading
/// fails.
pub fn download_from_metadata_directory(
    directory: &Path,
    progress_bar: &ProgressBar,
) -> Result<(), DownloaderError> {
    let metadata_files = directory
        .read_dir()
        .map_err(|error| DownloaderError::IoRead {
            path: directory.to_path_buf(),
            error,
        })?;

    for metadata_file in metadata_files {
        let metadata_file = metadata_file.map_err(|error| DownloaderError::IoRead {
            path: directory.to_path_buf(),
            error,
        })?;

        // Parse the contents of `metadata_file`.
        match corpus::parse(&metadata_file.path()) {
            // Download the program pairs listed in the metadata file.
            Ok(metadata) => download_from_metadata_file(&metadata, progress_bar),

            // If there is an error parsing the current file,
            // display an error and move on to the next file.
            Err(error) => eprintln!(
                "Failed to parse '{}': {}",
                metadata_file.path().display(),
                error
            ),
        }
    }

    Ok(())
}

/// Downloads all program pairs in a given Metadata object.
///
/// The program continues, rather than halts, if it fails to download
/// a program pair.
///
/// Increments the progress bar each time a metadata file is finished
/// processing.
///
/// # Arguments
///
/// - `metadata` - The program pairs to download.
/// - `progress_bar` - Is updated each time a metadata file is processed.
fn download_from_metadata_file(metadata: &Metadata, progress_bar: &ProgressBar) {
    for pair in metadata.pairs.iter() {
        if let Err(error) = download_program_pair(pair) {
            eprintln!("Failed to download '{}': {}", pair.program_name, error)
        };
    }
    progress_bar.inc(1);
}

/// Downloads a C-Rust program pair.
///
/// Checks if the C and Rust repositories exist, and clone them if they don't.
/// Copy the C source files to program_pairs/<program_name>/c-program.
/// Copy the Rust source files to program_pairs/<program_name>/rust-program.
///
/// # Side Effects
///
/// - Creates destination directories for program pairs at
///   `program_pairs/<program-name>/`.
///
/// # Arguments
///
/// - `pair` - A program pair.
///
/// # Returns
///
/// Returns `Ok(())` on success, or a [`DownloaderError`] on failure.
fn download_program_pair(pair: &ProgramPair) -> Result<(), DownloaderError> {
    let program_name = &pair.program_name;
    let base_program_path = Path::new(PROGRAM_PAIRS_DIRECTORY).join(program_name);
    let c_program_path = base_program_path.join("c-program");
    let rust_program_path = base_program_path.join("rust-program");

    // Create the destination directories for the C and Rust source files.
    fs::create_dir_all(&c_program_path).map_err(|source| DownloaderError::IoCreate {
        path: c_program_path.clone(),
        error: source,
    })?;
    fs::create_dir_all(&rust_program_path).map_err(|source| DownloaderError::IoCreate {
        path: rust_program_path.clone(),
        error: source,
    })?;

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
/// A progress bar is displayed on standard output to track cloning progress.
///
/// Side effects:
///
/// - Creates `repository_clones/`, which is a local cache for git clones.
/// - May overwrite files at `program_directory`.
///
/// # Arguments
///
/// - `program_name` - Name of the program being downloaded (used for progress messages).
/// - `program_language` - Language of the program (affects repository clone path).
/// - `program_directory` - Destination directory for the downloaded source files.
/// - `repository_url` - Git URL of the repository to clone.
/// - `source_files` - Paths (relative to repo root) of files or directories to copy.
///
/// # Returns
///
/// Returns `Ok(())` if all files were successfully downloaded and copied, or
/// [`DownloadError`] on failure.
fn download_files(
    program_name: &str,
    program_language: Language,
    program_directory: &Path,
    repository_url: &str,
    source_files: &[String],
) -> Result<(), DownloaderError> {
    let progress_bar = ProgressBar::new(80);

    let repository_directory = download_with_git(&program_language, repository_url, &progress_bar)?;

    progress_bar.set_style(ProgressStyle::default_spinner());
    progress_bar.set_message("Copying files...");

    // Copy given files from the repository to the given directory.
    for file_path in source_files {
        let file_name = Path::new(file_path).file_name().ok_or_else(|| {
            DownloaderError::Io(format!("Failed to get file name for path '{file_path}'"))
        })?;

        let source = repository_directory.join(file_path);
        let destination = program_directory.join(file_name);

        // Copy files from destination to source.
        if source.is_dir() {
            utils::copy_files_from_directory(&source, &program_directory)?;
        } else {
            fs::copy(&source, &destination).map_err(|error| DownloaderError::IoCopy {
                source: source.to_path_buf(),
                destination: destination.to_path_buf(),
                error,
            })?;
        }
    }

    progress_bar.finish_with_message(format!(
        "Downloaded '{}' ({})",
        program_name,
        program_language.to_str()
    ));
    Ok(())
}

/// Downloads a git repository using git clone.
///
/// # Arguments
///
/// - `program_language` - Either C or Rust.
/// - `repository_url` - The URL to download with git.
/// - `progress_bar` - A `ProgressBar` used to show the progress of the
///                    download status of the current program-pair.
///
/// # Returns
///
/// A `PathBuf` to the downloaded repository on success, or a
/// [`DownloaderError`] on failure.
fn download_with_git(
    program_language: &Language,
    repository_url: &str,
    progress_bar: &ProgressBar,
) -> Result<PathBuf, DownloaderError> {
    let repository_clones_path =
        Path::new(REPOSITORY_CLONES_DIRECTORY).join(program_language.to_str());
    let repository_name = utils::get_repository_name(repository_url)?;

    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("{bar:40.white/white} {pos}/{len} {msg}")
            .map_err(|error| DownloaderError::ProgressBar(error.to_string()))?
            .progress_chars("##-"),
    );
    progress_bar.set_message(format!("Cloning repository {repository_name}..."));

    // Set up remote callbacks for progress tracking.
    let mut remote_callbacks = RemoteCallbacks::new();
    remote_callbacks.transfer_progress(|progress: git2::Progress| {
        update_progress_bar_callback(progress, &repository_name, &progress_bar)
    });

    // Check if repository exists in `repository_clones/`, if not clone it.
    // We store repositories in repository_clones/<language>/<repository_name>.
    let repository = match Repository::open(repository_clones_path.join(&repository_name)) {
        Ok(repository) => repository,
        Err(_) => {
            // Set up fetch options with progress-tracking callbacks.
            let mut fetch_options = FetchOptions::new();
            fetch_options.remote_callbacks(remote_callbacks);

            // Clone only the latest commit to save time and space.
            fetch_options.depth(1);

            // Clone the repository.
            let mut builder = RepoBuilder::new();
            builder.fetch_options(fetch_options);
            builder
                .clone(
                    repository_url,
                    &repository_clones_path.join(&repository_name),
                )
                .map_err(|error| DownloaderError::CloneRepository {
                    repository_url: repository_url.to_string(),
                    error,
                })?
        }
    };

    let repository_directory = repository
        .workdir()
        .ok_or_else(|| {
            DownloaderError::Io(format!(
                "Failed to find working directory for repository '{repository_name}'"
            ))
        })?
        .to_path_buf();
    Ok(repository_directory)
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
/// True.  A progress bar callback function must return `true`.
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
