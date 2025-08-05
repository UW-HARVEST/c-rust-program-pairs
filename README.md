# C-Rust Program Pairs

This repository contains pairs of programs.
Each pair consists of a C program and a Rust program.

Each program is provided in its full source code tree.  For example, this is equivalent to a git clone, without the `.git/` subdirectory.

There is a script (name TBD) that re-creates or updates the contents of this repository, by downloading from the Internet and optionally doing some postprocessing.

## Metadata

Metadata files contain information about our C-Rust program pairs and can be validated with our [JSON schema](./metadata/metadata.schema.json).
In our CLI tool, we validate metadata files with this schema using the `jsonschema` crate, but you could also do so with any schema validation tool.

### Terminology

- **Fields**: Singular properties, attributes or fields that give us information.
- **Program**: An individual program that can be executed; each project can contain a single or multiple programs.
- **Project**: The entire repository or project that has been translated from C to Rust.
- **Pair**: A pair of C and Rust programs, where the Rust program was translated or inspired by the C program.
- **Metadata**: A single metadata file containing information about a project and its programs.

### Schema

We have two metadata schema types - an *individual* or *project* schema. In an individual metadata file, we group together unrelated C-Rust projects that each only contain one program.

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
        "executable_paths": ["./grep"]
      },
      "rust_program": {
        "documentation_url": "https://docs.rs/simple-grep",
        "repository_url": "https://github.com/user/rust-grep",
        "source_paths": ["src/main.rs", "src/lib.rs"],
        "executable_paths": ["target/release/simple-grep"]
      }
    }
  ]
}
```

In a project metadata file, we have a project containing many C-Rust programs.

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
        "executable_paths": ["src/ls"]
      },
      "rust_program": {
        "source_paths": ["src/uu/ls/src/ls.rs"],
        "executable_paths": ["target/release/ls"]
      }
    },
  ]
}
```

#### Fields

Our schema consists of many fields which specify individual properties or attributes that give information about each C-Rust program pair.

| Field | Type | Description | Valid Values/Examples |
|-------|------|-------------|----------------------|
| `program_name` | string | Name of the program | `"grep"`, `"ls"` |
| `program_description` | string | Brief description of program functionality | `"Text search utility"` |
| `repository_url` | string (URI) | Source code repository URL | `"https://github.com/user/repo"` |
| `documentation_url` | string (URI) | Documentation or project homepage URL | `"https://docs.rs/crate"` |
| `translation_method` | string | Translation process type | `"manual"`, `"semi-automatic"`, `"automatic"` |
| `translation_tool` | string | Tool used for translation | `"c2rust"`, `"manual-rewrite"` |
| `feature_relationship` | string | Feature comparison with C version | `"superset"`, `"subset"`, `"equivalent"`, `"overlapping"` |
| `source_paths` | array | Paths to source files/directories | `["src/main.rs", "src/lib.rs"]` |

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

Each C or Rust program have different configuration options, specified under the `c_program` or `rust_program` fields. Note that *project metadata* files have two program configurations. The first is the *global program configuration*, specified as the `project_global_program` field in our schema, which specifies fields that apply to every program pair in the project. This includes fields like `repository_url`, `documentation_url`, `build_commands`, `test_commands`, and `dependencies`. The next *program configuration* is listed as `project_program` in our schema and only applies to individual program pairs, containing the `source_paths` and `executable_paths` fields which are unique to each program.

## Corpus Tool

### Specification

1. Create a folder with the name of the program-pair.
2. Clone the C repository into a temporary directory, but check if this has already been done.
3. Copy the C program from the cloned repository into our folder.
4. Clone the Rust repository into a temporary directory, but check if this has already been done.
5. Copy the Rust program from the cloned repository into our folder.

## Resources

- [Awesome Rust](https://github.com/rust-unofficial/awesome-rust): Contains a list of awesome Rust projects - many of which might be translated from C projects.
- [JSON Schema Validator](https://www.jsonschemavalidator.net/): Used to validate JSON schemas.
