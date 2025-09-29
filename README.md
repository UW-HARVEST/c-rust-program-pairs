# C-Rust Program Pairs

This repository contains a list of C-Rust program pairs.  Each pair consists of
a C program and a Rust program with similar functionality.  The `metadata/`
directory lists the program pairs.  Often the Rust program was translated from
or inspired by the C program.  Program pairs only include the source code
without any dependencies, so they cannot be compiled.

Here is the directory structure for the downloaded `cat` program pair:

<<<<<<< HEAD
```tree
=======
```text
>>>>>>> b388e7282bce221aa4a18a714d4f294e7f3f2658
.
└── program_pairs/
    └── cat/
        ├── c-program/
        │   └── cat.c
        └── rust-program/
            └── cat.rs
```

## Prerequisites

You need the SSL development library.  On Ubuntu, run:

```sh
sudo apt-get install -y pkg-config libssl-dev
```

## Usage

To download all available program pairs into the `program_pairs/` directory:

```sh
cargo run
```

To download a small subset of available program pairs:

```sh
cargo run demo
```

To delete `program_pairs/` and `repository_clones/`:

```sh
cargo run delete
```

## Terminology

- **Program**: Code that compiles to a single executable.
- **Pair**: A pair of C and Rust programs with similar functionality.
- **Project**: A repository with C or Rust code.
  A project contains one or more programs.
- **Metadata**: Information about one or more pairs.

## Resources

- [Awesome Rust](https://github.com/rust-unofficial/awesome-rust): Contains a
  list of awesome Rust projects - many of which might be translated from C
  projects.
- [JSON Schema Validator](https://www.jsonschemavalidator.net/): Used to
  validate JSON schemas.

## Possible TODOs

- `git pull` repositories in the cache to update them.
- Use date or git SHA so repositories downloaded are consistent from run to run.
