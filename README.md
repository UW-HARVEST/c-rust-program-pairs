# C-Rust Program Pairs

This repository makes available a list of C to Rust program pairs that will be available in a `programs/` directory once downloaded. Program pairs only include the source code without any dependencies, so they cannot be compiled.

For example, the following example shows the directory structure for the `cat` program.

```
.
└── programs/
    └── cat/
        ├── c-program/
        │   └── cat.c
        └── rust-program/
            └── cat.rs
```

The `metadata/` directory contains many metadata files that contain information about our program pairs which is used by our script.

## Terminology

- **Program**: Refers to code that compiles to a single executable.
- **Project**: A repository that has been translated from C to Rust.  A project contains one or more programs.
- **Pair**: A pair of C and Rust programs, where the Rust program was translated from or inspired by the C program.
- **Metadata**: Gives information about one or more pairs.


## Usage

To run and download all avalaible program pairs simply use:

```bash
cargo run
```

## Metadata

Metadata files contain information about our C-Rust program pairs and can be validated with our [JSON schema](./metadata/metadata.schema.json).
In our CLI tool, we validate metadata files with this schema using the `jsonschema` crate, but you could also do so with any schema validation tool.

### Schema

We have two metadata schema types - an *individual* or *project* schema found in the `metadata/individual/` and `metadata/project/` directories respectively . In an individual metadata file, we group together unrelated C-Rust projects that each only contain one program.

```json
{
  "pairs": [
    {
      "program_name": "simple-grep",
      "program_description": "A basic text search utility",
      "translation_method": "manual",
      "translation_tool": "hand-written",
      "feature_relationship": "subset",
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

In metadata files in the `project/` directory, we have a project containing many C-Rust programs.

```json
{
  "project_information": {
    "program_name": "coreutils",
    "translation_method": "semi-automatic",
    "translation_tool": "c2rust with manual cleanup",
    "feature_relationship": "equivalent",
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

#### Schema fields

| Field | Type | Description | Valid Values/Examples |
|-------|------|-------------|----------------------|
| `program_name` | string | Name of the program | `"grep"`, `"ls"` |
| `program_description` | string | Brief description of program functionality | `"Text search utility"` |
| `documentation_url` | URL | Documentation or project homepage URL | `"https://docs.rs/crate"` |
| `repository_url` | URL | Source code repository URL | `"https://github.com/user/repo"` |
| `translation_method` | string | Translation process type | `"manual"`, `"semi-automatic"`, `"automatic"` |
| `translation_tool` | string | Tool used for translation | `"c2rust"`, `"manual-rewrite"` |
| `feature_relationship` | string | Feature comparison with C version | `"superset"`, `"subset"`, `"equivalent"`, `"overlapping"` |
| `source_paths` | array of paths | Paths to source files/directories | `["src/main.rs", "src/lib.rs"]` |

**Translation Method Values:**

- `manual` - Hand-written from scratch
- `semi-automatic` - Mix of automated tools and manual work
- `automatic` - Primarily tool-generated

**Feature Relationship Values:**

- `superset` - Rust has all C features plus more
- `subset` - Rust implements only some C features
- `equivalent` - Same feature set as C version
- `overlapping` - Some matching, some different features

#### Program Configuration

Each C or Rust program have different configuration options, specified under the `c_program` or `rust_program` fields in `metadata.schema.json`. Note that metadata files in `/project` have two program configurations. The first is the *global program configuration*, specified as the `project_global_program` field in our schema, which specifies fields that apply to every program pair in the project. This includes fields like `repository_url` and `documentation_url`. The next *program configuration* is listed as `project_program` in our schema and only applies to individual program pairs, containing the `source_paths` field which are unique to each program.

## Resources

- [Awesome Rust](https://github.com/rust-unofficial/awesome-rust): Contains a list of awesome Rust projects - many of which might be translated from C projects.
- [JSON Schema Validator](https://www.jsonschemavalidator.net/): Used to validate JSON schemas.
