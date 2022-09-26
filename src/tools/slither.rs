//! Module to run Slither

use super::solc;
use super::Summary;
use regex::Regex;
use rutil::report;
use std::io::prelude::*;
use std::path::PathBuf;
use std::{ffi::OsStr, fs, fs::File, path::Path, process::Command};

/// Run slither for each file
fn run_file(input_file_path: PathBuf) -> Result<PathBuf, String> {
    let check_results = solc::check_solc_settings(&input_file_path);

    if let Err(msg) = check_results {
        return Err(msg);
    }

    let slither_args = input_file_path.to_str().unwrap().to_string();

    let slither_output = Command::new(super::SLITHER)
        .args(slither_args.split_whitespace())
        .output()
        .unwrap();

    debug!("Running command: {} {}", super::SLITHER, slither_args);

    if !slither_output.status.success() {
        let error_msg =
            String::from_utf8(slither_output.stderr.to_vec()).expect("Slither: unknown error!");
        report::print_message("Slither error message:", error_msg.as_str());
        panic!("Failed to run: {}", input_file_path.display());
    }

    let file_stem_name = input_file_path
        .file_stem()
        .and_then(OsStr::to_str)
        .unwrap_or("");

    let parent_dir = input_file_path.parent().unwrap_or_else(|| Path::new(""));
    let output_file_path = parent_dir.join(file_stem_name.to_owned() + "." + super::SLITHER);

    let mut output_file = File::create(&output_file_path).unwrap();
    output_file.write_all(&slither_output.stderr).unwrap();

    Ok(output_file_path)
}

/// Interpret Slither results
fn interpret_results(file: &Path) -> Summary {
    // Note: Slither can find bugs in the following types:
    // Re-entrancy
    // Timestamp dependency
    // Unhandled exceptions
    // Use of tx.origin
    let contents =
        fs::read_to_string(file.to_str().unwrap()).expect("Should have been able to read the file");

    let reentrancy_regex = Regex::new(r"Reentrancy in ").unwrap();
    let timestamp_dep_regex = Regex::new(r" uses timestamp ").unwrap();
    let unhandled_exceptions_regex = Regex::new(r"Failure condition of ").unwrap();
    let tx_origin_regex = Regex::new(r" uses tx.origin for authorization").unwrap();
    let reentrancy = reentrancy_regex.captures_iter(contents.as_str()).count();
    let timestamp_dep = timestamp_dep_regex.captures_iter(contents.as_str()).count();
    let unhandled_exceptions = unhandled_exceptions_regex
        .captures_iter(contents.as_str())
        .count();
    let tx_origin = tx_origin_regex.captures_iter(contents.as_str()).count();

    Summary::new(
        reentrancy,
        timestamp_dep,
        0,
        unhandled_exceptions,
        0,
        0,
        tx_origin,
    )
}

/// Run slither using options
pub fn run_directory(dir: &str) -> Summary {
    // List all files in the repository
    let path = Path::new(&dir);
    let files = fs::read_dir(path).unwrap();

    let mut reentrancy = 0;
    let mut timestamp = 0;
    let mut tx_origin = 0;
    let mut unhanled_exceptions = 0;
    for file in files {
        let file = file.unwrap().path();
        let extension = file.extension().and_then(OsStr::to_str);

        if extension.unwrap() == "sol" {
            println!("Input file: {}", file.display());
            let output = run_file(file);
            match output {
                Ok(result) => {
                    debug!("The output is written to: {}", result.display());
                    let result = interpret_results(&result);
                    reentrancy += result.re_entrancy;
                    timestamp += result.timestamp;
                    unhanled_exceptions += result.unhandled_exceptions;
                    tx_origin += result.tx_origin;
                    debug!("bugs: {}", result);
                }
                Err(msg) => {
                    println!("err: {}", msg);
                }
            }
        }
    }

    Summary::new(
        reentrancy,
        timestamp,
        0,
        unhanled_exceptions,
        0,
        0,
        tx_origin,
    )
}
