//! # Build Script
//!
//! This build script generates Rust type definitions from the JSON Schema
//! located at `metadata/metadata.schema.json`. The generated code is written
//! to `metadata/metadata_schema.rs` and is included in the build output.
//!
//! ## Purpose
//!
//! - Ensure the Rust data structures used in this crate stay in sync with the
//!   JSON schema.
//! - Avoid manual type definition updates when the schema changes.
//!
//! ## How It Works
//!
//! 1. Reads the JSON Schema from `metadata/metadata.schema.json`.
//! 2. Parses it into a `schemars::schema::RootSchema`.
//! 3. Uses [`typify`] with [`TypeSpace`] to generate Rust types, enabling
//!    the `struct_builder` option for builder-pattern struct construction.
//! 4. Formats the generated Rust code using [`prettyplease`].
//! 5. Writes the result to `metadata/metadata_schema.rs`.
//!
//! ## Notes
//!
//! - This script runs **before every build**. Cargo will rerun it only if
//!   `metadata/metadata.schema.json` changes.
//! - The generated file is **checked in** (if you want reproducible builds)
//!   or **.gitignored** (if you prefer to regenerate each time).

use std::{fs, path::Path};

use typify::{TypeSpace, TypeSpaceSettings};

fn main() {
    let content = std::fs::read_to_string("metadata/metadata.schema.json").unwrap();
    let schema = serde_json::from_str::<schemars::schema::RootSchema>(&content).unwrap();

    let mut type_space = TypeSpace::new(TypeSpaceSettings::default().with_struct_builder(true));
    type_space.add_root_schema(schema).unwrap();

    let contents =
        prettyplease::unparse(&syn::parse2::<syn::File>(type_space.to_stream()).unwrap());

    let mut out_file = Path::new("metadata").to_path_buf();
    out_file.push("metadata_schema.rs");
    fs::write(out_file, contents).unwrap();
}
