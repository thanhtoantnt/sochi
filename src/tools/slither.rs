//! Module to run Slither

use super::solc;
use super::Summary;
use regex::Regex;
use std::io::prelude::*;
use std::path::PathBuf;
use std::{ffi::OsStr, fs, fs::File, io::BufReader, path::Path, process::Command};

/// Run slither for each file
fn run_slither(input_file_path: PathBuf) -> Result<PathBuf, String> {
    let file_stem_name = input_file_path
        .file_stem()
        .and_then(OsStr::to_str)
        .unwrap_or("");

    let parent_dir = input_file_path.parent().unwrap_or_else(|| Path::new(""));
    let output_file_path = parent_dir.join(file_stem_name.to_owned() + "." + super::SLITHER);

    if output_file_path.exists() {
        return Ok(output_file_path);
    }

    let check_results = solc::check_solc_settings(&input_file_path);

    if let Err(msg) = check_results {
        return Err(msg);
    }

    let solc = check_results.unwrap();
    debug!("solc version: {}", solc);

    let slither_args = "--detect reentrancy-benign,reentrancy-eth,reentrancy-events,reentrancy-unlimited-gas,reentrancy-no-eth,timestamp,tx-origin"
        .to_owned()
        + " "
        + input_file_path.to_str().unwrap();

    let slither_output = Command::new(super::SLITHER)
        .args(slither_args.split_whitespace())
        .output()
        .unwrap();

    debug!("Running command: {} {}", super::SLITHER, slither_args);

    let mut output_file = File::create(&output_file_path).unwrap();
    if !slither_output.status.success() {
        let error_msg =
            String::from_utf8(slither_output.stderr.to_vec()).expect("Slither: unknown error!");

        output_file.write_all(error_msg.as_bytes()).unwrap();
    } else {
        output_file.write_all(&slither_output.stderr).unwrap();
    }

    Ok(output_file_path)
}

/// Interpret Slither results
fn interpret_slither_results(file: &Path) -> Summary {
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

/// Interpret Slither results
fn check_slither_results(file: &Path) -> i32 {
    // read the csv file
    let file_stem_name = file.file_stem().and_then(OsStr::to_str).unwrap_or("");
    let parent_dir = file.parent().unwrap_or_else(|| Path::new(""));
    let result_file = parent_dir.join(file_stem_name.to_owned() + "." + super::SLITHER);
    debug!("Reading file: {}", result_file.display());
    let file = File::open(result_file.to_str().unwrap().to_string());
    let reader = BufReader::new(file.unwrap());
    let slither_regex = Regex::new(r"sol\#(\d+)-").unwrap();
    let mut positions = vec![];

    for line in reader.lines() {
        let line = line.unwrap();

        // Reentrancy bugs
        if line.contains("Reentrancy in") {
            for found in slither_regex.captures_iter(&line) {
                let foundx = found.get(1).map_or("", |c| c.as_str());
                debug!("found: {:?}", foundx);
                let number = foundx.parse::<i32>().unwrap();
                positions.push(number);
            }
        }

        // Timestamp Dependency bugs
        if line.contains("uses timestamp ") {
            for found in slither_regex.captures_iter(&line) {
                let foundx = found.get(1).map_or("", |c| c.as_str());
                debug!("found: {:?}", foundx);
                let number = foundx.parse::<i32>().unwrap();
                positions.push(number);
            }
        }
    }

    let baseline_file = parent_dir.join(file_stem_name.to_owned() + "." + super::CSV);
    let baseline = File::open(baseline_file.to_str().unwrap().to_string());
    let baseline_reader = BufReader::new(baseline.unwrap());
    let mut baseline_locations = vec![];

    for line in baseline_reader.lines() {
        //
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

    // for positions

    // let contents = fs::read_to_string(result_file.to_str().unwrap())
    //     .expect("Should have been able to read the file");

    // let slither_regex = Regex::new(r"sol\#(\d+)\)").unwrap();

    // for found in slither_regex.captures_iter(&contents) {
    //     let foundx = found.get(1).map_or("", |c| c.as_str());
    //     println!("found: {:?}", foundx);
    // }

    counter
}

/// Check slither results to be false/true positives
pub fn check_results(dir: &str) -> i32 {
    // List all files in the repository
    let path = Path::new(&dir);
    let mut paths: Vec<_> = fs::read_dir(path).unwrap().map(|r| r.unwrap()).collect();
    paths.sort_by_key(|dir| dir.path());

    // Statistics
    let mut summary = 0;

    for path in paths {
        let file = path.path();
        let extension = file.extension().and_then(OsStr::to_str);
        if extension.unwrap() == super::SLITHER {
            println!("Input file: {}", file.display());
            let result = check_slither_results(&file);
            summary += result;
            debug!("bugs: {}", result);
        }
    }

    println!("Total: {}", summary);

    summary
}

/// Interpret slither results
pub fn interpret_results(dir: &str) -> Summary {
    // List all files in the repository
    let path = Path::new(&dir);
    let mut paths: Vec<_> = fs::read_dir(path).unwrap().map(|r| r.unwrap()).collect();
    paths.sort_by_key(|dir| dir.path());

    // Statistics
    let mut reentrancy = 0;
    let mut timestamp = 0;
    let mut tx_origin = 0;
    let mut unhanled_exceptions = 0;

    for path in paths {
        let file = path.path();
        let extension = file.extension().and_then(OsStr::to_str);
        if extension.unwrap() == super::SLITHER {
            println!("Input file: {}", file.display());
            let result = interpret_slither_results(&file);
            reentrancy += result.re_entrancy;
            timestamp += result.timestamp;
            unhanled_exceptions += result.unhandled_exceptions;
            tx_origin += result.tx_origin;
            debug!("bugs: {}", result);
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

/// Run slither and get results
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
            let output = run_slither(file);
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
