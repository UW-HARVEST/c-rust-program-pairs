//! # Utility Functions for Git Repositories

use std::collections::HashMap;

use reqwest::blocking::Client;
use serde::Deserialize;

use crate::corpus::errors::DownloaderError;

#[derive(Debug, Deserialize)]
/// Represents the data received from a request to GitHub's API.
struct Repository {
    default_branch: String,
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
        .trim_end_matches("/")
        .split("/")
        .last()
        .expect("Unreachable because split always returns at least 1 element");
    let name = last_segment.strip_suffix(".git").unwrap_or(last_segment);
    Ok(name.to_string())
}

/// Extract a repository's owner from a GitHub URL.
///
/// # Example
///
/// The repository owner of
/// "https://github.com/eza-community/eza.git" is "eza-community".
///
/// # Arguments
///
/// - `url` - Git repository URL; must point to a valid, accessible repo.
///
/// # Returns
///
/// The owner of the repository on success or [`DownloaderError`] on failure.
pub fn get_repository_owner(url: &str) -> Result<String, DownloaderError> {
    if !url.contains("github") {
        return Err(DownloaderError::Io("Invalid URL".to_string()));
    }

    let url_parts: Vec<&str> = url.trim_end_matches("/").split("/").collect();
    let owner = url_parts[url_parts.len() - 2].to_string();
    Ok(owner)
}

/// Get the default branch for a GitHub repository.
///
/// # Arguments
///
/// - `url` - Git repository URL.
/// - `cache` - Caches repository URLs and their default branches. This is
///             important because there's a rate limit on GitHub's API.
///
/// # Returns
///
/// The name of the default branch on success or [`DownloaderError`] on
/// failure.
pub fn get_repository_default_branch(
    url: &str,
    cache: &mut HashMap<(String, String), String>,
) -> Result<String, DownloaderError> {
    if !url.contains("github") {
        return Err(DownloaderError::Io("Invalid URL".to_string()));
    }

    let owner = get_repository_owner(url)?;
    let repository = get_repository_name(url)?;

    // Check if result has already been cached.
    let key = (owner.to_string(), repository.to_string());
    if let Some(branch) = cache.get(&key) {
        return Ok(branch.to_string());
    }

    let api_url = format!("https://api.github.com/repos/{owner}/{repository}");

    let client = Client::new();
    let response = client
        .get(&api_url)
        .header("User-Agent", "c-rust-program-pairs")
        .send()
        .map_err(|error| DownloaderError::ApiError {
            url: api_url.clone(),
            error,
        })?;
    let data: Repository = response.json().map_err(|error| DownloaderError::ApiError {
        url: api_url.clone(),
        error,
    })?;

    // Add result to cache.
    cache.insert(key, data.default_branch.clone());

    Ok(data.default_branch)
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

    #[test]
    /// Tests that a repository owner can be extracted from a URL.
    fn test_get_repository_owner() {
        assert_eq!(
            "eza-community",
            get_repository_owner("https://github.com/eza-community/eza.git").unwrap()
        );
        assert_eq!(
            "eza-community",
            get_repository_owner("https://github.com/eza-community/eza").unwrap()
        );
    }

    #[test]
    /// Tests that a repository's default branch can be found from its URL.
    fn test_get_repository_default_branch() {
        let branch = "main";
        let mut cache = HashMap::new();

        // Returns correct repository branch.
        assert_eq!(
            branch,
            get_repository_default_branch("https://github.com/eza-community/eza.git", &mut cache)
                .unwrap()
        );

        // Correctly caches branch.
        let key = (String::from("eza-community"), String::from("eza"));
        assert_eq!(cache.get(&key).unwrap(), branch);
    }
}
