# Metadata

Metadata files contain information about C-Rust program pairs.

## Automatically Generated Structs

[`build.rs`](../build.rs) runs before compile time to automatically convert our [JSON schema](./metadata.schema.json) to Rust structs in [`metadata_structs.rs`](../src/corpus/metadata_structs.rs), which is included in the module tree. Then, before downloading program-pairs, our Rust program then validates metadata files using these automatically generated structs.

## Schema

There are two metadata schema types, *individual* and *project*. The `metadata/individual/` and `metadata/project/` directories contain JSON metadata files that conform to the individual and project schema types respectively.

An individual metadata file groups together unrelated C-Rust projects that each only contain one program.  Here is an example:

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

A project metadata file containing multiple C-Rust programs.  All the C programs are in a single project, and all the Rust programs are in a single project.

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

### Program Configuration

That metadata files in `project` have two program configurations. The *global program configuration* specifies fields that apply to every program pair in the project. This includes fields like `repository_url` and `documentation_url`. The *program configuration* applies to one program pair.

### Schema fields

| Field | Type | Description | Valid Values/Examples |
|-------|------|-------------|----------------------|
| `program_name` | string | Name of the Rust executable program | `"ripgrep"`, `"ls"` |
| `program_description` | string | Brief description of program functionality | `"Text search utility"` |
| `documentation_url` | URL | URL to detailed description or documentation | `"https://docs.rs/crate"` |
| `repository_url` | URL | Repository URL (GitHub, GitLab, etc.) | `"https://github.com/user/repo"` |
| `translation_tools` | array of strings | Tools used for the translation process | `"c2rust"`, `"manual"` |
| `feature_relationship` | string | How Rust features compare to C | `"overlapping"` |
| `source_paths` | array of paths | Paths to source files/directories | `["src/main.rs", "src/"]` |

- `source_paths`: Array of paths to files and directories containing source code. When specifying directories, only `.c`, `.h`, and `.rs` files will be included.
- `feature_relationship` Enum:
  - `rust_superset_of_c` - Rust has all C features plus more
  - `rust_subset_of_c` - Rust implements only some C features
  - `rust_equivalent_to_c` - Same feature set as C version
  - `overlapping` - Some matching features and some different features
