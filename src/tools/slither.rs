//! Module to run Slither

use std::io::prelude::*;
use std::path::PathBuf;
use std::{ffi::OsStr, fs, fs::File, path::Path, process::Command};

/// Run slither for each file
fn run_file(input_file_path: PathBuf) {
    let slither_args = input_file_path.to_str().unwrap().to_string() + " --exclude-low";

    let slither_output = Command::new(super::SLITHER)
        .args(slither_args.split_whitespace())
        .output()
        .unwrap();

    debug!("Running command: {} {}", super::SLITHER, slither_args);

    if !slither_output.status.success() {
        panic!("Slither running error");
    }

    let file_stem_name = input_file_path
        .file_stem()
        .and_then(OsStr::to_str)
        .unwrap_or("");

    let parent_dir = input_file_path.parent().unwrap_or_else(|| Path::new(""));
    let output_file_path = parent_dir.join(file_stem_name.to_owned() + "_slither.txt");

    let mut output_file = File::create(output_file_path).unwrap();
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
