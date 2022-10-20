//! Module to run Mythril

use super::solc;
use super::Summary;
use regex::Regex;
use std::env;
use std::fs::OpenOptions;
use std::io::Write;
// use rutil::report;
use std::path::PathBuf;
use std::{ffi::OsStr, fs, fs::File, path::Path, process::Command};
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

/// Get contract names from an input file
fn get_contract_names(input_file: &PathBuf) -> Vec<String> {
    //
    let contents = fs::read_to_string(input_file).expect(
        "Should have been able
    to read the file",
    );
    let regex = Regex::new(r"contract ([A-Z]([0-9A-Za-z])*)").unwrap();
    regex
        .find_iter(contents.as_str())
        .map(|contract| {
            contract
                .as_str()
                .strip_prefix("contract ")
                .unwrap()
                .to_string()
        })
        .collect()
}

/// Generate mythril command for each file
fn generate_command(input_file_path: PathBuf) -> String {
    debug!("input file: {}", input_file_path.display());
    // Step 1:
    let check_results = solc::check_solc_settings(&input_file_path);

    if let Err(msg) = check_results {
        panic!("{}", msg);
    }

    debug!("Create a directory for the file");
    let parent_dir = env::current_dir().unwrap().join("ilf-data");

    let file_stem_name = input_file_path
        .file_stem()
        .and_then(OsStr::to_str)
        .unwrap_or("");

    let output_dir = parent_dir.join(file_stem_name);
    let _ = fs::create_dir_all(&output_dir).unwrap();
    // contracts directory
    let output_contract_dir = output_dir.join("contracts");
    let _ = fs::create_dir_all(&output_contract_dir).unwrap();

    // migration directory
    let output_migrations_dir = output_dir.join("migrations");
    let _ = fs::create_dir_all(&output_migrations_dir).unwrap();
    // let migrate_file = output_migrations_dir.join("delop.js");
    let contract_names = get_contract_names(&input_file_path);
    println!("contracts: {:?}", contract_names);
    for contract_name in contract_names.clone() {
        let input = format!(
            "
var ContractName = artifacts.require(\"{}\");

module.exports = function(deployer) {{
  deployer.deploy(ContractName);
}};
",
            contract_name
        );

        let file_path = output_migrations_dir.join(contract_name + ".js");
        let mut file = File::create(&file_path).unwrap();
        let _ = writeln!(file, "{}", input);
        debug!("migration file: {}", file_path.display());
    }

    let cp_args =
        input_file_path.to_str().unwrap().to_string() + " " + output_contract_dir.to_str().unwrap();

    // Copy to the new directory
    let cp_output = Command::new("cp")
        .args(cp_args.split_whitespace())
        .output()
        .unwrap();

    debug!("cp {}", cp_args);

    if !cp_output.status.success() {
        let error_msg = String::from_utf8(cp_output.stderr.to_vec()).expect(
            "Slither:
        unknown error!",
        );
        println!("cp error message: {}", error_msg);
    }

    // copy truffle-config.js
    let truffle_file = Path::new("/home/thanhtoantnt/workspace/sochi/ilf/truffle-config.js");
    let truffle_args =
        truffle_file.to_str().unwrap().to_string() + " " + output_dir.to_str().unwrap();
    let cp_truffle_output = Command::new("cp")
        .args(truffle_args.split_whitespace())
        .output()
        .unwrap();

    debug!("cp {}", truffle_args);
    if !cp_truffle_output.status.success() {
        let error_msg = String::from_utf8(cp_truffle_output.stderr.to_vec()).expect("Copy error");
        println!("cp error message: {}", error_msg);
    }

    let mut fst_command = format!(
        "python3 script/extract.py --proj {} --port 8545\n",
        output_dir.to_str().unwrap()
    );

    for contract_name in contract_names {
        let contract_command = format!(
            "python3 -m ilf --proj {} --contract {} --fuzzer imitation --model ./model/ --limit 2000\n",
            output_dir.to_str().unwrap(),
            contract_name
        );

        fst_command.push_str(&contract_command);
    }

    return fst_command;
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

    let output_file_path = env::current_dir().unwrap().join("ilf_script.sh");
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
            let command = generate_command(path.path().to_path_buf());
            debug!("{}", command);
            let _ = writeln!(output_file, "{}", command);
        }
    }
}
