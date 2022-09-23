//! Module to run Slither

use super::Summary;
use regex::Regex;
use rutil::system;
use semver::Version;
use std::io::prelude::*;
use std::path::PathBuf;
use std::{ffi::OsStr, fs, fs::File, path::Path, process::Command};

/// Check path of the Solc compiler
fn check_solc_path() -> Result<(), String> {
    match system::path_of_command_from_env(super::SOLC) {
        Ok(path) => {
            debug!("Solc path: {}", path);
            Ok(())
        }
        Err(_) => Err("Solc path not found!".to_string()),
    }
}

/// Check path of `solc-select`
fn check_solc_select_path() -> Result<(), String> {
    match system::path_of_command_from_env(super::SOLC_SELECT) {
        Ok(path) => {
            debug!("Solc-select path: {}", path);
            Ok(())
        }
        Err(_) => Err("Solc-select path not found!".to_string()),
    }
}

/// Install required solc version
fn install_solc_version(version: Version) -> Result<(), String> {
    match check_solc_select_path() {
        Ok(_) => {
            // Install solc
            let args = " install".to_owned() + &version.to_string();
            Command::new(super::SOLC_SELECT)
                .args(args.split_whitespace())
                .output()
                .unwrap();

            // Use solc version
            let args = " use".to_owned() + &version.to_string();
            Command::new(super::SOLC_SELECT)
                .args(args.split_whitespace())
                .output()
                .unwrap();

            Ok(())
        }
        Err(msg) => Err(msg),
    }
}

/// Check version of the Solc compiler
fn check_solc_version(required_version: Version) -> Result<(), String> {
    match Command::new(super::SOLC).args(&["--version"]).output() {
        Ok(output) => {
            let output_str = String::from_utf8(output.stdout).unwrap();
            let regex = Regex::new(r"Version: (\d+\.\d+\.\d+)").unwrap();
            let solc_ver = match regex.captures(output_str.as_str()) {
                Some(capture) => capture.get(1).map_or("", |c| c.as_str()),
                None => "",
            };

            match Version::parse(solc_ver) {
                Ok(ver) => {
                    if required_version != ver {
                        return install_solc_version(required_version);
                    }

                    Ok(())
                }
                Err(msg) => {
                    let err_msg = format!("Solc version not found: {}", msg);
                    Err(err_msg)
                }
            }
        }

        Err(_) => Err("Check Solc version: command not found".to_string()),
    }
}

/// Check settings of the Solc compiler
fn check_solc_settings(input_file: &Path) -> Result<(), String> {
    let contents = fs::read_to_string(input_file.to_str().unwrap())
        .expect("Should have been able to read the file");

    let regex = Regex::new(r"pragma solidity \^(\d+\.\d+\.\d+)").unwrap();
    let regex_gt = Regex::new(r"pragma solidity >=(\d+\.\d+\.\d+)").unwrap();
    let solc_ver = match regex.captures(contents.as_str()) {
        Some(capture) => capture.get(1).map_or("", |c| c.as_str()),
        None => match regex_gt.captures(contents.as_str()) {
            Some(capture) => capture.get(1).map_or("", |c| c.as_str()),
            None => "",
        },
    };

    let solc_ver = match Version::parse(solc_ver) {
        Ok(ver) => ver,
        Err(msg) => {
            let err_msg = format!(
                "Solc version not found: {}.\n Please add pragma solidity ^solc_version to the input contract",
                msg
            );

            return Err(err_msg);
        }
    };

    match check_solc_path() {
        Ok(_) => check_solc_version(solc_ver),
        Err(msg) => Err(msg),
    }
}

/// Run slither for each file
fn run_file(input_file_path: PathBuf) -> Result<PathBuf, String> {
    let check_results = check_solc_settings(&input_file_path);

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
        panic!("Slither running error");
    }

    let file_stem_name = input_file_path
        .file_stem()
        .and_then(OsStr::to_str)
        .unwrap_or("");

    let parent_dir = input_file_path.parent().unwrap_or_else(|| Path::new(""));
    let output_file_path = parent_dir.join(file_stem_name.to_owned() + "_slither.txt");

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
    let timestamp_regex = Regex::new(r" uses timestamp ").unwrap();
    let reentrancy = reentrancy_regex.captures_iter(contents.as_str()).count();
    let timestamp = timestamp_regex.captures_iter(contents.as_str()).count();

    Summary::new(reentrancy, timestamp, 0)
}

/// Run slither using options
pub fn run_directory(dir: &str) -> Summary {
    // List all files in the repository
    let path = Path::new(&dir);
    let files = fs::read_dir(path).unwrap();

    let mut reentrancy = 0;
    let mut timestamp = 0;
    let mut tx_origin = 0;
    for file in files {
        let file = file.unwrap().path();
        let extension = file.extension().and_then(OsStr::to_str);

        if extension.unwrap() == "sol" {
            println!("Input file: {}", file.display());
            let output = run_file(file);
            match output {
                Ok(result) => {
                    // TODO: Interpret results
                    debug!("The output is written to: {}", result.display());
                    let result = interpret_results(&result);
                    reentrancy += result.re_entrancy;
                    timestamp += result.timestamp;
                    tx_origin += result.tx_origin;
                    debug!("bugs: {}", result);
                }
                Err(msg) => {
                    println!("err: {}", msg);
                }
            }
        }
    }

    Summary::new(reentrancy, timestamp, tx_origin)
}
