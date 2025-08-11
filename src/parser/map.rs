use std::{error::Error, fs, path::Path};

use typify::import_types;

use crate::parser::{
    canonical::{Language, Metadata, Program, ProgramPair},
    validator,
};

import_types!(schema = "metadata/metadata.schema.json");

// Parses a project metadata file into a schema::Metadata object.
pub fn parse(path: &Path) -> Result<Metadata, Box<dyn Error>> {
    let raw_metadata = fs::read_to_string(path)?;
    let metadata: CToRustTranslationSchema = serde_json::from_str(&raw_metadata)?;

    // Validate metadata file with our JSON schema.
    validator::validate_metadata(&metadata)?;

    match metadata {
        CToRustTranslationSchema::IndividualPairsMetadata { pairs } => {
            let metadata = parse_individual(&pairs)?;
            return Ok(metadata);
        }
        CToRustTranslationSchema::ProjectPairsMetadata {
            pairs,
            project_information,
        } => {
            let metadata = parse_project(&pairs, &project_information)?;
            return Ok(metadata);
        }
    }
}

fn parse_individual(pairs: &[IndividualProgramPair]) -> Result<Metadata, Box<dyn Error>> {
    let pairs: Vec<ProgramPair> = pairs
        .into_iter()
        .map(|pair| ProgramPair {
            program_name: pair.program_name.to_string(),
            program_description: pair.program_description.to_string(),
            translation_tools: pair.translation_tools.0,
            feature_relationship: pair.feature_relationship,
            c_program: Program {
                language: Language::C,
                documentation_url: pair.c_program.documentation_url.to_string(),
                repository_url: pair.c_program.repository_url.to_string(),
                source_paths: pair.c_program.source_paths.0,
            },
            rust_program: Program {
                language: Language::Rust,
                documentation_url: pair.rust_program.documentation_url.to_string(),
                repository_url: pair.rust_program.repository_url.to_string(),
                source_paths: pair.rust_program.source_paths.0,
            },
        })
        .collect();

    Ok(Metadata { pairs })
}

fn parse_project(
    pairs: &[ProjectProgramPair],
    project_information: &ProjectPairsMetadataProjectInformation,
) -> Result<Metadata, Box<dyn Error>> {
    let pairs: Vec<ProgramPair> = pairs
        .into_iter()
        .map(|pair| ProgramPair {
            program_name: pair.program_name.to_string(),
            program_description: pair.program_description.to_string(),
            translation_tools: project_information.translation_tools.0,
            feature_relationship: project_information.feature_relationship,
            c_program: Program {
                language: Language::C,
                documentation_url: project_information.c_program.documentation_url.to_string(),
                repository_url: project_information.c_program.repository_url.to_string(),
                source_paths: pair.c_program.source_paths.0,
            },
            rust_program: Program {
                language: Language::Rust,
                documentation_url: project_information
                    .rust_program
                    .documentation_url
                    .to_string(),
                repository_url: project_information.rust_program.repository_url.to_string(),
                source_paths: pair.rust_program.source_paths.0,
            },
        })
        .collect();

    Ok(Metadata { pairs })
}
