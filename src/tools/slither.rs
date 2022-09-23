//! Module to run Slither

use std::io::prelude::*;
use std::path::PathBuf;
use std::{ffi::OsStr, fs, fs::File, path::Path, process::Command};

/// Run slither for each file
fn run_file(filename: PathBuf) {
    //

    let slither_args = filename.to_str().unwrap().to_string() + " --exclude-low";

    let slither_output = Command::new(super::SLITHER)
        .args(slither_args.split_whitespace())
        .output()
        .unwrap();

    debug!("Running command: {} {}", super::SLITHER, slither_args);

    if !slither_output.status.success() {
        panic!("Slither running error");
    }

    let mut output_file = File::create("output.txt").unwrap();
    let _ = output_file.write_all(&slither_output.stderr).unwrap();
}

/// Run slither using options
pub fn run_directory(dir: &str) {
    // List all files in the repository
    let path = Path::new(&dir);
    let files = fs::read_dir(path).unwrap();

    for file in files {
        let file = file.unwrap().path();
        let extension = file.extension().and_then(OsStr::to_str);

        if extension.unwrap() == "sol" {
            println!("filename: {}", file.display());
            run_file(file);
        }
    }
}
