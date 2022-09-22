//! Module to run Slither

use std::{fs, path::Path};

/// Run slither using options
pub fn run(dir: &str) {
    // List all files in the repository
    let path = Path::new(&dir);
    let files = fs::read_dir(path).unwrap();
    for file in files {
        println!("filename: {}", file.unwrap().path().display());
    }
}
