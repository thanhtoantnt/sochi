//! Module to check and change `solc` settings.

use regex::Regex;
use rutil::system;
use semver::Version;
use std::io::prelude::*;
use std::{fs, fs::File, path::Path, process::Command};

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
fn install_solc_version(version: Version) -> Result<String, String> {
    match check_solc_select_path() {
        Ok(_) => {
            // Install solc
            let args = " install ".to_owned() + &version.to_string();
            Command::new(super::SOLC_SELECT)
                .args(args.split_whitespace())
                .output()
                .unwrap();

            debug!("Running command: {} {}", super::SOLC_SELECT, args);

            // Use solc version
            let args = " use ".to_owned() + &version.to_string();
            Command::new(super::SOLC_SELECT)
                .args(args.split_whitespace())
                .output()
                .unwrap();

            debug!("Running command: {} {}", super::SOLC_SELECT, args);

            Ok(version.to_string())
        }
        Err(msg) => Err(msg),
    }
}

/// Check version of the Solc compiler
fn check_solc_version(required_version: Version) -> Result<String, String> {
    match Command::new(super::SOLC).args(&["--version"]).output() {
        Ok(output) => {
            let output_str = String::from_utf8(output.stdout).unwrap();
            let regex = Regex::new(r"Version: (\d+\.\d+\.\d+)").unwrap();
            let solc_ver = match regex.captures(output_str.as_str()) {
                Some(capture) => capture.get(1).map_or("", |c| c.as_str()),
                None => "",
            };

            // debug!("solc_ver: {}", solc_ver);

            match Version::parse(solc_ver) {
                Ok(ver) => {
                    if required_version != ver {
                        return install_solc_version(required_version);
                    }

                    Ok(required_version.to_string())
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
pub fn check_solc_settings(input_file: &Path) -> Result<String, String> {
    use super::solidity_versions::*;

    let org_contents = fs::read_to_string(input_file.to_str().unwrap())
        .expect("Should have been able to read the file");

    let contents = str::replace(org_contents.as_str(), "0.5.11", "0.5.12");

    if contents != org_contents {
        fs::remove_file(input_file.to_str().unwrap()).unwrap_or(());
        let mut new_file = File::create(input_file.to_str().unwrap()).unwrap();
        new_file.write_all(contents.as_bytes()).unwrap();
        println!("new file is written to: {}", input_file.to_str().unwrap());
    }

    let regex = Regex::new(r"pragma solidity \^?(\d+\.\d+\.\d+)").unwrap();
    let regex_gt = Regex::new(r"pragma solidity >=\x20?(\d+\.\d+\.\d+)").unwrap();
    let regex_less =
        Regex::new(r"pragma solidity >=\x20?(\d+\.\d+\.\d+) <\x20?(\d+\.\d+\.\d+)").unwrap();
    let solc_ver = match regex.captures(contents.as_str()) {
        Some(capture) => capture.get(1).map_or("", |c| c.as_str()),
        None => match regex_less.captures(contents.as_str()) {
            Some(capture) => {
                let version = capture.get(1).map_or("", |c| c.as_str());
                let version2 = capture.get(2).map_or("", |c| c.as_str());
                if version2 == ZERO_EIGHT_FIRST {
                    ZERO_SEVEN_LAST
                } else if version2 == ZERO_SEVEN_FIRST {
                    ZERO_SIX_LAST
                } else if version2 == ZERO_SIX_FIRST {
                    ZERO_FIVE_LAST
                } else if version2 == ZERO_FIVE_FIRST {
                    ZERO_FOUR_LAST
                } else {
                    version
                }
            }
            None => match regex_gt.captures(contents.as_str()) {
                Some(capture) => capture.get(1).map_or("", |c| c.as_str()),
                None => "",
            },
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

    // debug!("solc-ver: {}", solc_ver);

    match check_solc_path() {
        Ok(_) => check_solc_version(solc_ver),
        Err(msg) => Err(msg),
    }
}
