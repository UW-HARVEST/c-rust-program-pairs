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

## Resources

- [Awesome Rust](https://github.com/rust-unofficial/awesome-rust): Contains a list of awesome Rust projects - many of which might be translated from C projects.
- [JSON Schema Validator](https://www.jsonschemavalidator.net/): Used to validate JSON schemas.
