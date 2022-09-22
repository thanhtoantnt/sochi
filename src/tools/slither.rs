//! Module to run Slither

use std::{ffi::OsStr, fs, path::Path};

/// Run slither using options
pub fn run(dir: &str) {
    // List all files in the repository
    let path = Path::new(&dir);
    let files = fs::read_dir(path).unwrap();
    for file in files {
        let file = file.unwrap().path();
        let extension = file.extension().and_then(OsStr::to_str);

        if extension.unwrap() == "sol" {
            println!("filename: {}", file.display());
        }
    }
}
