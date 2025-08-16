# Metadata

Metadata files contain information about our C-Rust program pairs and can be validated with our [JSON schema](./metadata.schema.json).

## Automatically Generated Structs

We use a [`build.rs`](../build.rs) script to automatically convert our JSON schema to Rust structs at build time. The generated rust structs can be found at [`metadata/metadata_schema.rs`](./metadata_schema.rs).

To use any of these structs, we use the `import_types` macro provided by `typify`:

```rust
import_types!(schema = "metadata/metadata.schema.json");
```

## Schema

We have two metadata schema types - an *individual* or *project* schema found in the `metadata/individual/` and `metadata/project/` directories respectively . In an individual metadata file, we group together unrelated C-Rust projects that each only contain one program.

When downloading program pairs, we first validate metadata files with this schema using the `jsonschema` crate, but you could also do so with any schema validation tool.

### Individual Metadata Schema

- Individual metadata files consist of an array `pairs` containing all C to Rust program pairs.

```json
{
  "pairs": [
    {
      "program_name": "simple-grep",
      "program_description": "A basic text search utility",
      "translation_tool": ["manual"],
      "feature_relationship": "rust_subset_of_c",
      "c_program": {
        "documentation_url": "https://example.com/c-grep",
        "repository_url": "https://github.com/user/c-grep",
        "source_paths": ["grep.c", "utils.h"],
      },
      "rust_program": {
        "documentation_url": "https://docs.rs/simple-grep",
        "repository_url": "https://github.com/user/rust-grep",
        "source_paths": ["src/main.rs", "src/lib.rs"],
      }
    }
  ]
}
```

### Project Metadata Schema

- Similar to the individual metadata schema, we have a `pairs` field that contains information about specific C to Rust program pairs.
- However, since all programs are related under the same project, the `project_information` field contains fields that apply to every single program pair.
  - For example, the `c_program` and `rust_program` fields under `project_information` describes the common `documentation_url` and `repository_url` for all program pairs.
  - However, the `c_program` and `rust_program` fields found under `pairs` contain the `source_files` specific to each individual program pair.

```json
{
  "project_information": {
    "program_name": "coreutils",
    "translation_tools": ["c2rust", "manual"],
    "feature_relationship": "rust_equivalent_to_c",
    "c_program": {
      "documentation_url": "https://www.gnu.org/software/coreutils/",
      "repository_url": "https://github.com/coreutils/coreutils",
    },
    "rust_program": {
      "documentation_url": "https://github.com/uutils/coreutils",
      "repository_url": "https://github.com/uutils/coreutils",
    }
  },
  "pairs": [
    {
      "program_name": "ls",
      "program_description": "List directory contents",
      "c_program": {
        "source_paths": ["src/ls.c"],
      },
      "rust_program": {
        "source_paths": ["src/uu/ls/src/ls.rs"],
      }
    },
  ]
}
```

### Schema fields

| Field | Type | Description | Valid Values/Examples |
|-------|------|-------------|----------------------|
| `program_name` | string | Name of the program | `"grep"`, `"ls"` |
| `program_description` | string | Brief description of program functionality | `"Text search utility"` |
| `documentation_url` | URL | Documentation or project homepage URL | `"https://docs.rs/crate"` |
| `repository_url` | URL | Source code repository URL | `"https://github.com/user/repo"` |
| `translation_tools` | array of strings | Tools used for translation | `"c2rust"`, `"manual"` |
| `feature_relationship` | string | Feature comparison with C version | `"overlapping"` |
| `source_paths` | array of paths | Paths to source files/directories | `["src/main.rs", "src/"]` |

- `source_paths`: This field should include only files that contain the source code of the program.
  - A Single File: A single file of source code.
  - Directory: Includes all files including the directory. Only specify directories if we are sure that every file in there is source code (no READMEs, etc.).
- `feature_relationship` Enum:
  - `rust_superset_of_c` - Rust has all C features plus more
  - `rust_subset_of_c` - Rust implements only some C features
  - `rust_equivalent_to_c` - Same feature set as C version
  - `overlapping` - Some matching, some different features
