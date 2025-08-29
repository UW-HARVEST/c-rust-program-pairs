//! # C-Rust program pair downloader
//!
/// Entry point for the metadata downloader.
///
/// # Usage
///
/// Download full metadata corpus:
/// ```bash
/// cargo run
/// ```
///
/// Download demo/sample metadata only:
/// ```bash
/// cargo run -- demo
/// ```
use c_rust_program_pairs;

fn main() {
    c_rust_program_pairs::run();
}
