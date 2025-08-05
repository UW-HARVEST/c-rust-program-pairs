mod corpus;
mod parser;
mod paths;

#[cfg(test)]
mod tests {
    use crate::{
        parser::{individual, project},
        paths::{INDIVIDUAL_METADATA_DIRECTORY, PROJECT_METADATA_DIRECTORY},
    };

    use std::path::Path;

    // Tests that a project-metadata file can be successfully parsed.
    #[test]
    fn test_parse_project() {
        let metadata_file = Path::new(PROJECT_METADATA_DIRECTORY).join("diffutils.json");
        let result = project::parse(&metadata_file);
        assert!(
            result.is_ok(),
            "Failed to parse project metadata: {:?}",
            result.err()
        );
    }

    // Tests that an individual-metadata file can be successfully parsed.
    #[test]
    fn test_parse_individual() {
        let metadata_file = Path::new(INDIVIDUAL_METADATA_DIRECTORY).join("system-tools.json");
        let result = individual::parse(&metadata_file);
        assert!(
            result.is_ok(),
            "Failed to parse individual metadata: {:?}",
            result.err()
        );
    }
}

use crate::{
    parser::{individual, project},
    paths::{INDIVIDUAL_METADATA_DIRECTORY, PROJECT_METADATA_DIRECTORY},
};

use std::path::Path;

fn main() {
    let project_metadata_directory = Path::new(PROJECT_METADATA_DIRECTORY);
    let individual_metadata_directory = Path::new(INDIVIDUAL_METADATA_DIRECTORY);

    for metadata_file in project_metadata_directory
        .read_dir()
        .expect("Failed to read project metadata directory")
    {
        if let Ok(metadata_file) = metadata_file {
            match project::parse(&metadata_file.path()) {
                Ok(metadata) => {
                    println!("Successfully parsed {:?}", metadata_file.path().file_name());
                    corpus::download_metadata(&metadata);
                }
                Err(error) => println!(
                    "Failed to parse {:?}: {}",
                    metadata_file.path().file_name(),
                    error
                ),
            }
        }
    }
}
