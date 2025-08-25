# C-Rust Program Pairs

This repository contains a list of C-Rust program pairs.  The `metadata/` directory lists the program pairs.  Often the Rust program was translated from or inspired by the C program.  Program pairs only include the source code without any dependencies, so they cannot be compiled.

Here is the directory structure for the downloaded `cat` program pair:

```
.
└── programs/
    └── cat/
        ├── c-program/
        │   └── cat.c
        └── rust-program/
            └── cat.rs
```

## Usage

To download all available program pairs into the `programs/` directory:

```sh
cargo run
```

To run a demo and download a small subset of available program pairs:

```sh
cargo run demo
```

## Terminology

- **Program**: Code that compiles to a single executable.
- **Pair**: A pair of C and Rust programs with similar functionality.
- **Project**: A repository with C or Rust code.  A project contains one or more programs.
- **Metadata**: Information about one or more pairs.

## Resources

- [Awesome Rust](https://github.com/rust-unofficial/awesome-rust): Contains a list of awesome Rust projects - many of which might be translated from C projects.
- [JSON Schema Validator](https://www.jsonschemavalidator.net/): Used to validate JSON schemas.

## Possible TODOs

- `git pull` repositories in the cache to update them.
- Require date of download or repository version so we ensure that repositories downloaded are consistent from run to run.
