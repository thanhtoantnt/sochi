//! Module to run Mythril

use super::solc;
use super::Summary;
use regex::Regex;
use std::env;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::Write;
use std::path::PathBuf;
use std::{ffi::OsStr, fs, fs::File, io::BufReader, path::Path};
use walkdir::WalkDir;

/// Read and interpret mythril results for each file
fn interpret_mythril_results(input_file: PathBuf) -> Summary {
    let contents = fs::read_to_string(input_file.to_str().unwrap())
        .expect("Should have been able to read the file");

    let reentrancy_regex = Regex::new(r"State access after external call").unwrap();
    let timestamp_dep_regex = Regex::new(
        r"The block.timestamp environment variable is used to determine a control flow decision",
    )
    .unwrap();
    let unhandled_exceptions_regex =
        Regex::new(r"Unchecked return value from external call").unwrap();
    let unhandled_exception_regex2 = Regex::new(r"External Call To User-Supplied Address").unwrap();
    let tx_origin_regex = Regex::new(r"Dependence on tx.origin").unwrap();
    let integer_regex = Regex::new(r"Integer Arithmetic Bugs").unwrap();
    let unchecked_send_regex = Regex::new(r"Unprotected Ether Withdrawal").unwrap();

    let reentrancy = reentrancy_regex.captures_iter(contents.as_str()).count();
    let timestamp_dep = timestamp_dep_regex.captures_iter(contents.as_str()).count();
    let unhandled_exceptions = unhandled_exceptions_regex
        .captures_iter(contents.as_str())
        .count();
    let unhandled_exception2 = unhandled_exception_regex2
        .captures_iter(contents.as_str())
        .count();
    let tx_origin = tx_origin_regex.captures_iter(contents.as_str()).count();
    let integer_bugs = integer_regex.captures_iter(contents.as_str()).count();
    let unchecked_send = unchecked_send_regex
        .captures_iter(contents.as_str())
        .count();

    Summary::new(
        reentrancy,
        timestamp_dep,
        unchecked_send,
        unhandled_exceptions + unhandled_exception2,
        0,
        integer_bugs,
        tx_origin,
    )
}

/// Generate mythril command for each file
fn generate_command(input_file_path: PathBuf) -> String {
    debug!("input file: {}", input_file_path.display());
    let check_results = solc::check_solc_settings(&input_file_path);

    if let Err(msg) = check_results {
        panic!("{}", msg);
    }

    let solv = check_results.unwrap();

    let file_stem_name = input_file_path
        .file_stem()
        .and_then(OsStr::to_str)
        .unwrap_or("");

    let parent_dir = input_file_path.parent().unwrap_or_else(|| Path::new(""));
    let output_file_path = parent_dir.join(file_stem_name.to_owned() + "." + super::MYTHRIL);

    let mythril_args = "analyze".to_owned()
        + " --execution-timeout 10"
        + format!(" --solv {} ", solv).as_str()
        + input_file_path.to_str().unwrap()
        + " > "
        + output_file_path.to_str().unwrap();

    debug!("{} {}", super::MYTHRIL, mythril_args);

    format!("echo \"{0} {1}\"\n{0} {1}\n", super::MYTHRIL, mythril_args)
}

/// Interpret mythril results
pub fn interpret_results(dir: &str) -> Summary {
    // List all files in the repository
    let path = Path::new(&dir);
    let mut paths: Vec<_> = fs::read_dir(path).unwrap().map(|r| r.unwrap()).collect();
    paths.sort_by_key(|dir| dir.path());

    let mut reentrancy = 0;
    let mut timestamp = 0;
    let mut tx_origin = 0;
    let mut unhanled_exceptions = 0;
    let mut unchecked_send = 0;
    let mut integer_bugs = 0;

    for path in paths {
        let file = path.path();
        let extension = file.extension().and_then(OsStr::to_str);

        if extension.unwrap() == super::MYTHRIL {
            println!("Input file: {}", file.display());
            let result = interpret_mythril_results(file);
            reentrancy += result.re_entrancy;
            timestamp += result.timestamp;
            unhanled_exceptions += result.unhandled_exceptions;
            tx_origin += result.tx_origin;
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
        0,
        integer_bugs,
        tx_origin,
    )
}

/// Generate mythril commands
pub fn generate_commands(dir: &str) {
    // List all files in the repository
    let path = Path::new(&dir);
    let paths = WalkDir::new(path).into_iter().filter_map(|e| e.ok());

    let output_file_path = env::current_dir().unwrap().join("mythril_script.sh");
    File::create(&output_file_path).unwrap();
    let mut output_file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(output_file_path)
        .unwrap();

    for path in paths {
        let file = path.path();
        let extension = file.extension().and_then(OsStr::to_str);

        if let Some(super::SOL) = extension {
            let output = generate_command(path.path().to_path_buf());
            let _ = writeln!(output_file, "{}", output);
        }
    }
}

/// Interpret Slither results
fn check_mythril_results(input_file: &Path) -> u32 {
    // read the csv file
    let file_stem_name = input_file.file_stem().and_then(OsStr::to_str).unwrap_or("");
    let parent_dir = input_file.parent().unwrap_or_else(|| Path::new(""));
    let result_file = parent_dir.join(file_stem_name.to_owned() + "." + super::MYTHRIL);
    debug!("Reading file: {}", result_file.display());
    let result_file = File::open(result_file.to_str().unwrap().to_string());
    let reader = BufReader::new(result_file.unwrap());
    let mythril_regex = Regex::new(r"sol:(\d+)").unwrap();
    let mut positions = vec![];

    for line in reader.lines() {
        let line = line.unwrap();

        for found in mythril_regex.captures_iter(&line) {
            let foundx = found.get(1).map_or("", |c| c.as_str());
            let number = foundx.parse::<i32>().unwrap();
            debug!("found: {}", number);
            positions.push(number);
        }
    }

    let baseline_file = parent_dir.join(file_stem_name.to_owned() + "." + super::CSV);
    let baseline = File::open(baseline_file.to_str().unwrap().to_string());
    let baseline_reader = BufReader::new(baseline.unwrap());
    let mut baseline_locations = vec![];

    for line in baseline_reader.lines() {
        let line = line.unwrap();
        let elements: Vec<&str> = line.split(',').collect();
        let location = elements[0];
        let range = elements[1];
        let number_opt = location.parse::<i32>();
        if let Ok(number) = number_opt {
            let range = range.parse::<i32>().unwrap();
            baseline_locations.push((number, range));
        }
    }

    let mut counter = 0;
    for position in positions {
        for (number, range) in baseline_locations.clone() {
            if position >= number && position <= number + range {
                counter += 1;
            }
        }
    }

    counter
}

/// Check mythril results to be false/true positives
pub fn check_results(dir: &str) -> u32 {
    // List all files in the repository
    let path = Path::new(&dir);
    let mut paths: Vec<_> = fs::read_dir(path).unwrap().map(|r| r.unwrap()).collect();
    paths.sort_by_key(|dir| dir.path());

    // Statistics
    let mut counter = 0;

    for path in paths {
        let file = path.path();
        let extension = file.extension().and_then(OsStr::to_str);
        if extension.unwrap() == super::MYTHRIL {
            println!("Input file: {}", file.display());
            let result = check_mythril_results(&file);
            counter += result;
        }
    }

    counter
}
