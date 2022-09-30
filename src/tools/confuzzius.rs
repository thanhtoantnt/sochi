//! Module to run Confuzzius

use super::solc;
use super::Summary;
use regex::Regex;
use rutil::report;
use std::io::prelude::*;
use std::path::PathBuf;
use std::{ffi::OsStr, fs, fs::File, path::Path, process::Command};

/// Run confuzzius for each file
fn run_confuzzius(input_file_path: PathBuf) -> Result<PathBuf, String> {
    let file_stem_name = input_file_path
        .file_stem()
        .and_then(OsStr::to_str)
        .unwrap_or("");

    let parent_dir = input_file_path.parent().unwrap_or_else(|| Path::new(""));
    let output_file_path = parent_dir.join(file_stem_name.to_owned() + "." + super::CONFUZZIUS);

    if output_file_path.exists() {
        return Ok(output_file_path);
    }

    let check_results = solc::check_solc_settings(&input_file_path);

    if let Err(msg) = check_results {
        return Err(msg);
    }

    let solc = check_results.unwrap();

    let confuzzius_loc = "../ConFuzzius/fuzzer/main.py";

    let confuzzius_args = confuzzius_loc.to_owned()
        + " -s "
        + input_file_path.to_str().unwrap()
        + format!(" --solc v{}", solc).as_str()
        + " --evm byzantium"
        + " -g 20";

    debug!("Running command: {} {}", super::PYTHON3, confuzzius_args);

    let confuzzius_output = Command::new(super::PYTHON3)
        .args(confuzzius_args.split_whitespace())
        .output()
        .unwrap();

    if !confuzzius_output.status.success() {
        let error_msg = String::from_utf8(confuzzius_output.stderr.to_vec())
            .expect("Confuzzius: unknown error!");
        report::print_message("Confuzzius error message:", error_msg.as_str());
        let msg = format!("Failed to run: {}", input_file_path.display());
        return Err(msg);
    }

    let mut output_file = File::create(&output_file_path).unwrap();
    output_file.write_all(&confuzzius_output.stderr).unwrap();

    Ok(output_file_path)
}

/// Interpret Confuzzius results
fn interpret_confuzzius_results(file: &Path) -> Summary {
    let contents =
        fs::read_to_string(file.to_str().unwrap()).expect("Should have been able to read the file");

    let reentrancy_regex = Regex::new(r"Reentrancy detected").unwrap();
    let timestamp_dep_regex = Regex::new(r"Block dependency detected").unwrap();
    let unchecked_send_regex = Regex::new(r"Leaking ether detected").unwrap();
    let unhandled_exceptions_regex = Regex::new(r"Unchecked return value detected").unwrap();
    let tod_regex = Regex::new(r"Transaction order dependency detected").unwrap();
    let integer_overflow_regex = Regex::new(r"Integer overflow detected").unwrap();
    let integer_underflow_regex = Regex::new(r"Integer underflow detected").unwrap();

    let reentrancy = reentrancy_regex.captures_iter(contents.as_str()).count();
    let timestamp_dep = timestamp_dep_regex.captures_iter(contents.as_str()).count();
    let unchecked_send = unchecked_send_regex
        .captures_iter(contents.as_str())
        .count();
    let unhandled_exceptions = unhandled_exceptions_regex
        .captures_iter(contents.as_str())
        .count();
    let tod = tod_regex.captures_iter(contents.as_str()).count();
    let integer_overflow = integer_overflow_regex
        .captures_iter(contents.as_str())
        .count();
    let integer_underflow = integer_underflow_regex
        .captures_iter(contents.as_str())
        .count();

    Summary::new(
        reentrancy,
        timestamp_dep,
        unchecked_send,
        unhandled_exceptions + unchecked_send,
        tod,
        integer_overflow + integer_underflow,
        0,
    )
}

/// Run Confuzzius and get results
pub fn generate_results(dir: &str) {
    // List all files in the repository
    let path = Path::new(&dir);
    let mut paths: Vec<_> = fs::read_dir(path).unwrap().map(|r| r.unwrap()).collect();
    paths.sort_by_key(|dir| dir.path());

    for path in paths {
        let file = path.path();
        let extension = file.extension().and_then(OsStr::to_str);

        if let Some(super::SOL) = extension {
            println!("Input file: {}", file.display());
            let output = run_confuzzius(file);
            match output {
                Ok(result) => {
                    println!("The output is written to: {}", result.display());
                }
                Err(msg) => {
                    println!("err: {}", msg);
                }
            }
        }
    }
}

/// Run confuzzius using options
pub fn interpret_results(dir: &str) -> Summary {
    // List all files in the repository
    let path = Path::new(&dir);
    let mut paths: Vec<_> = fs::read_dir(path).unwrap().map(|r| r.unwrap()).collect();
    paths.sort_by_key(|dir| dir.path());

    let mut reentrancy = 0;
    let mut timestamp = 0;
    let mut tod = 0;
    let mut unhanled_exceptions = 0;
    let mut unchecked_send = 0;
    let mut integer_bugs = 0;

    for path in paths {
        let file = path.path();
        let extension = file.extension().and_then(OsStr::to_str);

        if extension.unwrap() == super::CONFUZZIUS {
            println!("Input file: {}", file.display());
            let result = interpret_confuzzius_results(&file);
            reentrancy += result.re_entrancy;
            timestamp += result.timestamp;
            unhanled_exceptions += result.unhandled_exceptions;
            tod += result.trans_ordering_dep;
            integer_bugs += result.integer_flow;
            unchecked_send += result.unchecked_send;
            debug!("bugs: {}", result);
        }
    }

    Summary::new(
        reentrancy,
        timestamp,
        unchecked_send,
        unhanled_exceptions,
        tod,
        integer_bugs,
        0,
    )
}
