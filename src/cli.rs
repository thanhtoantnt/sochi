//! Command line arguments for Insight

use clap::{Arg, ArgMatches, Command};

use rutil::arg::ArgExt;

use std::cmp::Ordering;

/// Module defining print command-line arguments for `Insight`.
pub mod print_args {
    /// Debug mode
    pub const DEBUG_MODE: &str = "debug";

    /// Deep debug mode
    pub const DEEP_DEBUG_MODE: &str = "deep-debug";

    /// Argument to print contract summary
    pub const PRINT_CONTRACT_SUMMARY: &str = "print-contract-summary";

    /// Argument to print function summary
    pub const PRINT_FUNCTION_SUMMARY: &str = "print-function-summary";

    /// Argument to print call graph
    pub const PRINT_CALL_GRAPH: &str = "print-call-graph";

    /// Argument to print call graph in a dot file
    pub const PRINT_DOT: &str = "print-dot";

    /// Argument to print call graph in a .png file
    pub const PRINT_PNG: &str = "print-png";

    /// Argument to print call graph in a pdf file
    pub const PRINT_PDF: &str = "print-pdf";

    /// Argument to print call graph in JSON format
    pub const PRINT_JSON: &str = "print-json";

    /// Directory to store output files
    pub const OUTPUT_DIR: &str = "output-dir";
}

/// Command line arguments for printing source code insights.
pub mod args {
    /// Keyword to parse input file.
    pub const INPUT_FILE: &str = "input-file";

    /// Argument to pass specific options to Solc
    pub const SOLC_OPTIONS: &str = "solc-options";
}

/// Data structure for options in printing source code insights.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PrinterOptions<'a> {
    /// print contract summary
    pub print_contract_summary: bool,

    /// print debugging information
    pub debug_mode: bool,

    /// print call graph
    pub print_call_graph: bool,

    /// print call graph in dot files
    pub print_dot: bool,

    /// print call graph in png files
    pub print_png: bool,

    /// print call graph in pdf files
    pub print_pdf: bool,

    /// print call graph in pdf files
    pub print_json: bool,

    /// output directory
    pub output_dir: Vec<&'a str>,
}
/// Command line options.
#[derive(Debug)]
pub struct Options<'a> {
    /// Input file.
    pub input_file_name: &'a str,

    /// Print options for Insight.
    pub printer_options: PrinterOptions<'a>,

    /// Option to pass specific options to Solc.
    pub solc_options: Vec<&'a str>,
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
            Arg::new_argument(PRINT_CONTRACT_SUMMARY)
                .help("Print contract summary")
                .visible_alias("pcs"),
        )
        .arg(
            Arg::new_argument(DEBUG_MODE)
                .help("Print debugging information")
                .short('d'),
        )
        .arg(
            Arg::new_argument(PRINT_CALL_GRAPH)
                .help("Print call graph")
                .visible_alias("pcg"),
        )
        .arg(Arg::new_argument(PRINT_DOT).help("Print call graph into a .dot file"))
        .arg(Arg::new_argument(PRINT_PNG).help("Print call graph into a .png file"))
        .arg(Arg::new_argument(PRINT_PDF).help("Print call graph into a pdf file"))
        .arg(Arg::new_argument(PRINT_JSON).help("Print call graph into JSON format"))
        .arg(
            Arg::new_argument(OUTPUT_DIR)
                .help("Path of a directory to store output files")
                .takes_value(true)
                .multiple_occurrences(true)
                .allow_invalid_utf8(true)
                .display_order(2)
                .short('o'),
        )
    }
}

/// Configure command line argument matches.
pub fn configure_arguments() -> ArgMatches {
    let matches = clap::Command::new("visualizer")
        .version(&*format!("version {}", env!("GIT_VERSION")))
        // .help_template("{bin} ({version}) - {usage}")
        .about("Visualization tool for smart contracts.")
        // .author(&*format!("By {}", env!("CARGO_PKG_AUTHORS")))
        .configure_print_arguments()
        .arg(
            Arg::new(args::INPUT_FILE)
                .help("Input file")
                .required(true)
                .allow_invalid_utf8(true)
                .multiple_values(false),
        )
        .arg(
            Arg::new_argument(args::SOLC_OPTIONS)
                .help("Options passed specifically to Solc")
                .allow_invalid_utf8(true)
                .takes_value(true)
                .display_order(2),
        )
        .get_matches();

    matches
}

/// Parse print options.
pub fn parse_printer_argument_matches(argms: &ArgMatches) -> PrinterOptions {
    use self::print_args::*;
    let print_contract_summary = argms.is_present(PRINT_CONTRACT_SUMMARY);
    let print_dot = argms.is_present(PRINT_DOT);
    let print_png = argms.is_present(PRINT_PNG);
    let print_pdf = argms.is_present(PRINT_PDF);
    let print_json = argms.is_present(PRINT_JSON);

    let output_dir = match argms.values_of_os(OUTPUT_DIR) {
        None => vec![],
        Some(ss) => ss.into_iter().filter_map(|v| v.to_str()).collect(),
    };

    let print_call_graph = argms.is_present(PRINT_CALL_GRAPH);

    PrinterOptions {
        debug_mode: argms.is_present(DEBUG_MODE),
        print_contract_summary,
        print_call_graph,
        print_dot,
        print_png,
        print_json,
        print_pdf,
        output_dir,
    }
}

/// Parse command line arguments.
pub fn parse_argument_matches(argms: &ArgMatches) -> Options {
    let mut input_files = argms.values_of_os(args::INPUT_FILE).unwrap();

    match input_files.len().cmp(&1) {
        Ordering::Less => panic!("No input file is given!"),
        Ordering::Greater => panic!("Expect only 1 input file!"),
        Ordering::Equal => (),
    }

    let solc_user_options = match argms.values_of_os(args::SOLC_OPTIONS) {
        None => vec![],
        Some(ss) => ss.into_iter().filter_map(|v| v.to_str()).collect(),
    };

    // Configure `Insight` flags
    let vopts = parse_printer_argument_matches(argms);
    Options {
        input_file_name: input_files.next().unwrap().to_str().unwrap_or_default(),
        printer_options: vopts,
        solc_options: solc_user_options,
    }
}
