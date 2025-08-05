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
    corpus::download_metadata_dir,
    parser::schema::MetadataType,
    paths::{INDIVIDUAL_METADATA_DIRECTORY, PROJECT_METADATA_DIRECTORY},
};

use std::path::Path;

fn main() {
    download_metadata_dir(Path::new(PROJECT_METADATA_DIRECTORY), MetadataType::Project);
    download_metadata_dir(
        Path::new(INDIVIDUAL_METADATA_DIRECTORY),
        MetadataType::Individual,
    );
}
