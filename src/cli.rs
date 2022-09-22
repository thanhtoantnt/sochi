//! Command line arguments for Insight

use clap::{Arg, ArgMatches, Command};

use rutil::arg::ArgExt;

// use crate::tools;

/// Data structure
#[derive(Debug, PartialEq)]
pub enum ToolName {
    /// Slither
    Slither,

    /// Mythril
    Mythril,
}

impl ToolName {
    pub fn from_str(input: String) -> ToolName {
        if input == "slither" {
            return ToolName::Slither;
        } else if input == "mythril" {
            return ToolName::Mythril;
        }

        panic!("The tool is not in the list of [slither, mythril]");
    }
}

/// Module defining print command-line arguments for `Insight`.
pub mod print_args {
    /// Debug mode
    pub const DEBUG_MODE: &str = "debug";

    /// Directory to store output files
    pub const TOOL: &str = "tool";
}

/// Data structure for options in printing source code insights.
#[derive(Debug)]
pub struct PrinterOptions {
    /// Debug mod
    pub debug_mode: bool,

    /// Select one tool to run: [Slither, Mythril, etc]
    pub tools: Vec<ToolName>,
}
/// Command line options.
#[derive(Debug)]
pub struct Options {
    /// Print options for Insight.
    pub printer_options: PrinterOptions,
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
                .help("To run a tool [Slither, Mythril]")
                .takes_value(true)
                .multiple_occurrences(true)
                .allow_invalid_utf8(true)
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
        .get_matches();

    matches
}

/// Parse print options.
pub fn parse_printer_argument_matches(argms: &ArgMatches) -> PrinterOptions {
    use self::print_args::*;
    let tools = match argms.values_of_os(TOOL) {
        None => vec![],
        Some(ss) => ss.into_iter().filter_map(|v| v.to_str()).collect(),
    };

    let tools = tools
        .into_iter()
        .map(|v| ToolName::from_str(v.to_string()))
        .collect();

    PrinterOptions {
        debug_mode: argms.is_present(DEBUG_MODE),
        tools,
    }
}

/// Parse command line arguments.
pub fn parse_argument_matches(argms: &ArgMatches) -> Options {
    // Configure `Insight` flags
    let vopts = parse_printer_argument_matches(argms);
    Options {
        printer_options: vopts,
    }
}
