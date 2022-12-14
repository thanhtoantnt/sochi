//! Command line arguments for Insight

use crate::tools;
use clap::{Arg, ArgMatches, Command};
use rutil::arg::ArgExt;
use std::cmp::Ordering;

/// Data structure for a list of tools
#[derive(Debug, PartialEq, Eq)]
pub enum ToolName {
    /// Slither
    Slither,

    /// Confuzzius
    Confuzzius,

    /// Mythril
    Mythril,

    /// ILF
    ILF,
}

impl ToolName {
    /// Constructor
    pub fn new(input: String) -> ToolName {
        if input == tools::SLITHER {
            return ToolName::Slither;
        } else if input == tools::MYTHRIL {
            return ToolName::Mythril;
        } else if input == tools::CONFUZZIUS {
            return ToolName::Confuzzius;
        } else if input == tools::ILF {
            return ToolName::ILF;
        }

        panic!("The tool is not in the list of [slither, myth, confuzzius, ilf]");
    }
}

/// Module defining print command-line arguments for `Insight`.
pub mod print_args {
    /// Debug mode
    pub const DEBUG_MODE: &str = "debug";

    /// The tool to be run and get results
    pub const TOOL: &str = "tool";

    /// Generate commands for a tool
    pub const GENERATE_COMMANDS: &str = "gen-commands";

    /// Generate results for a tool
    pub const GENERATE_RESULTS: &str = "gen-results";

    /// Check results for a tool
    pub const CHECK_RESULTS: &str = "check-results";
}

/// Command line argument.
pub mod args {
    /// Argument to parse input directory.
    pub const INPUT_DIR: &str = "input-dir";
}

/// Data structure for options.
#[derive(Debug)]
pub struct RunOptions {
    /// Debug mod
    pub debug_mode: bool,

    /// Generate commands
    pub generate_commands: bool,

    /// Generate results
    pub generate_results: bool,

    /// Check results to be false/true positive
    pub check_results: bool,

    /// Select one tool to run: [Slither, Mythril, etc]
    pub tools: Vec<ToolName>,
}
/// Command line options.
#[derive(Debug)]
pub struct Options<'a> {
    /// Print options for Insight.
    pub printer_options: RunOptions,

    /// Input directory
    pub input_dir: &'a str,
}

/// A trait to declare print arguments for `Insight`.
pub trait PrinterCli {
    /// Configure print command-line arguments.
    fn configure_print_arguments(self) -> Self;
}

/// Implement `PrinterCli` trait for `Command`.
impl<'a> PrinterCli for Command<'a> {
    fn configure_print_arguments(self) -> Self {
        use self::print_args::*;
        self.arg(
            Arg::new_argument(DEBUG_MODE)
                .help("Print debugging information")
                .short('d'),
        )
        .arg(
            Arg::new_argument(TOOL)
                .help("To run a tool in the set [slither, mythril, confuzzius]")
                .takes_value(true)
                .multiple_occurrences(true)
                .allow_invalid_utf8(true)
                .display_order(2),
        )
        .arg(
            Arg::new_argument(GENERATE_COMMANDS)
                .help("Generate commands for a tool")
                .display_order(2),
        )
        .arg(
            Arg::new_argument(GENERATE_RESULTS)
                .help("Generate running results for a tool")
                .display_order(2),
        )
        .arg(
            Arg::new_argument(CHECK_RESULTS)
                .help("Check results for a tool")
                .display_order(2),
        )
    }
}

/// Configure command line argument matches.
pub fn configure_arguments() -> ArgMatches {
    let matches = clap::Command::new("sochi")
        .version(&*format!("version {}", env!("GIT_VERSION")))
        // .help_template("{bin} ({version}) - {usage}")
        .about("Run tools to do benchmark")
        // .author(&*format!("By {}", env!("CARGO_PKG_AUTHORS")))
        .configure_print_arguments()
        .arg(
            Arg::new(args::INPUT_DIR)
                .help("Input directory")
                .required(true)
                .allow_invalid_utf8(true)
                .multiple_values(false),
        )
        .get_matches();

    matches
}

/// Parse print options.
pub fn parse_printer_argument_matches(argms: &ArgMatches) -> RunOptions {
    use self::print_args::*;
    let tools = match argms.values_of_os(TOOL) {
        None => vec![],
        Some(ss) => ss.into_iter().filter_map(|v| v.to_str()).collect(),
    };

    let tools = tools
        .into_iter()
        .map(|v| ToolName::new(v.to_string()))
        .collect();

    RunOptions {
        debug_mode: argms.is_present(DEBUG_MODE),
        generate_commands: argms.is_present(GENERATE_COMMANDS),
        generate_results: argms.is_present(GENERATE_RESULTS),
        check_results: argms.is_present(CHECK_RESULTS),
        tools,
    }
}

/// Parse command line arguments.
pub fn parse_argument_matches(argms: &ArgMatches) -> Options {
    // Configure `Insight` flags
    let vopts = parse_printer_argument_matches(argms);

    let mut input_dirs = argms.values_of_os(args::INPUT_DIR).unwrap();

    let input_dir = match input_dirs.len().cmp(&1) {
        Ordering::Less => panic!("No input directory is given!"),
        Ordering::Greater => panic!("Expect only 1 input directory!"),
        Ordering::Equal => input_dirs.next().unwrap().to_str().unwrap_or_default(),
    };

    Options {
        printer_options: vopts,
        input_dir,
    }
}
