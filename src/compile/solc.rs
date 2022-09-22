//! Module to compile Solidity for `Visualizer`

use crate::tools;
use crate::tools::OUTPUT_DIR;
use regex::Regex;
use rutil::string::StringExt;
use rutil::{report, system};
use semver::Version;

use std::{ffi::OsStr, fs, path::Path, process::Command};

/// Check path of the Solc compiler
pub fn check_solc_path() -> Result<(), String> {
    match system::path_of_command_from_env(tools::SOLC) {
        Ok(path) => {
            debug!("Solc path: {}", path);
            Ok(())
        }
        Err(_) => Err("Solc path not found!".to_string()),
    }
}

/// Check version of the Solc compiler
pub fn check_solc_version(required_version: Version) -> Result<(), String> {
    match Command::new(tools::SOLC).args(&["--version"]).output() {
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
                        let err_msg = format!("Expect Solc version {0} but found: {1}. Please use Solc {0}",
                            required_version, solc_ver
                        );

                        return Err(err_msg);
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
pub fn check_solc_settings(input_file: &str) -> Result<(), String> {
    let path = Path::new(input_file);
    let extension = path.extension();
    if extension != Some(OsStr::new("sol")) {
        return Err("Input file is not a Solidity file. Please choose again."
            .to_string());
    }

    let contents = fs::read_to_string(input_file)
        .expect("Should have been able to read the file");
    let regex = Regex::new(r"pragma solidity \^(\d+\.\d+\.\d+)").unwrap();
    let solc_ver = match regex.captures(contents.as_str()) {
        Some(capture) => capture.get(1).map_or("", |c| c.as_str()),
        None => "",
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

/// Compile Solidity programs and return the output file name.
pub fn compile(
    input_file: &str,
    user_options: &[&str],
) -> Result<Vec<String>, String> {
    // Check compiler settings
    let check_results = check_solc_settings(input_file);
    if let Err(msg) = check_results {
        return Err(msg);
    }

    // Start to compile the input file
    let input_file_path = Path::new(input_file);
    let filename = input_file_path
        .file_name()
        .and_then(OsStr::to_str)
        .unwrap_or("");

    let parent_dir = input_file_path.parent().unwrap_or_else(|| Path::new(""));

    // prepare output folder
    let output_dir = parent_dir.join(OUTPUT_DIR).join(filename);
    let output_dir_path = output_dir.to_str().unwrap();
    fs::remove_dir_all(output_dir_path).unwrap_or(());
    fs::create_dir_all(output_dir_path).unwrap_or(());

    let solc_args = input_file.to_owned()
        + &user_options.join(" ").add_prefix_if_not_empty(" ")
        + " --ast-compact-json"
        + " --pretty-json"
        + format!(" -o {}", output_dir_path).as_str();

    debug!("Running command: {} {}", tools::SOLC, solc_args);

    let solc_output = Command::new(tools::SOLC)
        .args(solc_args.split_whitespace())
        .output()
        .unwrap();

    if !solc_output.status.success() {
        let error_msg = String::from_utf8(solc_output.stderr.to_vec())
            .expect("Solc: unknown error!");
        report::print_message("Solc error message:", error_msg.as_str());
        let err_msg = format!("Failed to compile: {}", input_file);
        return Err(err_msg);
    }

    // Print out output of running solc
    // io::stdout().write_all(&solc_output.stdout).unwrap();

    // Print out all the warnings
    // io::stdout().write_all(&solc_output.stderr).unwrap();

    let output_files = system::ls_dir(output_dir_path)
        .into_iter()
        .filter_map(|filename: String| -> Option<String> {
            if filename.ends_with("ast") {
                Some(filename)
            } else {
                None
            }
        })
        .collect();

    Ok(output_files)
}
