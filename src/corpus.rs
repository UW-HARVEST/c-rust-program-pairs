use crate::{
    parser::schema::{Language, Metadata, ProgramPair},
    paths::{PROGRAMS_DIRECTORY, REPOSITORY_CACHE_DIRECTORY},
};

use git2::{FetchOptions, Progress, RemoteCallbacks, Repository, build::RepoBuilder};
use indicatif::{ProgressBar, ProgressStyle};
use std::{error::Error, fs, path::Path};

// Downloads all program-pairs in a given Metadata object.
pub fn download_metadata(metadata: &Metadata) {
    for pair in metadata.pairs.iter() {
        match download_program_pair(pair) {
            Ok(_) => {}
            Err(error) => println!("Failed to download {}: {}", pair.program_name, error),
        }
    }
}

// Downloads a program-pair into the ./programs directory.
// The C source files will be downloaded to ./programs/<program_name>/c-program.
// The Rust source files will be downloaded to ./programs/<program_name>/rust-program.
fn download_program_pair(pair: &ProgramPair) -> Result<(), Box<dyn Error>> {
    let program_name = &pair.program_name;
    let base_program_path = Path::new(PROGRAMS_DIRECTORY).join(program_name);
    let c_program_path = base_program_path.join("c-program");
    let rust_program_path = base_program_path.join("rust-program");

    // Don't clone repositories if directories already exist.
    if base_program_path.exists() {
        println!("{program_name} already exists!");
        return Ok(());
    }

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

// Clones a repository copy the relevant files to the program directory.
// Also prints a progress bar to standard output to track progress.
fn download_files(
    program_name: &str,
    program_language: Language,
    program_directory: &Path,
    repository_url: &str,
    files: &[String],
) -> Result<(), Box<dyn Error>> {
    // Create a progress bar.
    let progress_bar = ProgressBar::new(100);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.cyan} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} {msg}",
            )
            .unwrap()
            .progress_chars("##-"),
    );
    progress_bar.set_message(format!("Cloning {program_name}..."));

    // Set up remote callbacks for progress tracking.
    let mut remote_callbacks = RemoteCallbacks::new();
    remote_callbacks.transfer_progress(|progress: Progress| {
        update_progress_bar_callback(progress, &progress_bar)
    });

    // Setup up fetch options with our callbacks.
    let mut fetch_options = FetchOptions::new();
    fetch_options.remote_callbacks(remote_callbacks);

    // Clone only the latest commit to save time / space.
    fetch_options.depth(1);

    // Check if repository exists in cache, if not clone it.
    // We store repositories in repository_cache/<language>/<repository_name>.
    let language = match program_language {
        Language::C => "c",
        Language::Rust => "rust",
    };
    let repository_path = Path::new(REPOSITORY_CACHE_DIRECTORY).join(language);
    let repository_name =
        get_repository_name(repository_url).ok_or("Failed to get repository name from URL.")?;
    let repository = match Repository::open(repository_path.join(&repository_name)) {
        Ok(repository) => repository,
        Err(_) => {
            let mut builder = RepoBuilder::new();
            builder.fetch_options(fetch_options);
            builder.clone(repository_url, &repository_path.join(&repository_name))?
        }
    };

    progress_bar.set_message("Copying files...");
    progress_bar.set_style(ProgressStyle::default_spinner());

    // Copy given files from the repository to the given directory.
    for file_path in files.iter() {
        let file_name = Path::new(file_path)
            .file_name()
            .ok_or("Invalid file path.")?;
        let source = repository
            .workdir()
            .ok_or("Failed to find source file in repository.")?
            .join(&file_path);
        let destination = program_directory.join(file_name);
        if !destination.exists() {
            if source.is_dir() {
                copy_dir(&source, &destination)?;
            } else {
                fs::copy(source, destination)?;
            }
        }
    }

    progress_bar.finish_with_message(format!(
        "Successfully downloaded {} ({})!",
        program_name, language
    ));
    Ok(())
}

// Update progress bar with cloning progress.
fn update_progress_bar_callback(progress: Progress, progress_bar: &ProgressBar) -> bool {
    let received_objects = progress.received_objects();
    let received_bytes = progress.received_bytes();
    let total_objects = progress.total_objects();
    let indexed_objects = progress.indexed_objects();

    // Downloading objects.
    if received_objects < total_objects {
        progress_bar.set_length(total_objects as u64);
        progress_bar.set_position(received_objects as u64);
        progress_bar.set_message(format!(
            "Receiving objects: {received_objects}/{total_objects} ({received_bytes:.1} B)"
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

// Get the name of the repository from the URL.
fn get_repository_name(url: &str) -> Option<String> {
    url.split("/")
        .last()?
        .strip_suffix(".git")
        .or_else(|| url.split("/").last())
        .map(|s| s.to_string())
}

// Copy a directory recursively from source to destination.
fn copy_dir(source: &Path, destination: &Path) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(destination)?;

    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let entry_source = entry.path();
        let entry_destination = destination.join(entry.file_name());

        if file_type.is_dir() {
            copy_dir(&entry_source, &entry_destination)?;
        } else {
            fs::copy(&entry_source, &entry_destination)?;
        }
    }

    Ok(())
}
