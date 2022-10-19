//! Main module for printing information of contract

//--------------------------------------------------------------------
// ATTRIBUTES TO GUARANTEE CODE QUALITY. DO NOT MODIFY.
// Warning on future incompatible features
#![warn(future_incompatible)]
// Linting rules, enabled when running Cargo with `--features linting`
#![cfg_attr(feature = "linting", deny(missing_docs))]
#![cfg_attr(feature = "linting", deny(clippy::missing_docs_in_private_items))]
#![cfg_attr(feature = "linting", deny(unused))]
#![cfg_attr(feature = "linting", deny(nonstandard_style))]
#![cfg_attr(feature = "linting", deny(clippy::perf))]
#![cfg_attr(feature = "linting", deny(clippy::style))]
#![cfg_attr(feature = "linting", deny(clippy::complexity))]
#![cfg_attr(feature = "linting", deny(clippy::suspicious))]
#![cfg_attr(feature = "linting", deny(clippy::doc_markdown))]
#![cfg_attr(feature = "linting", deny(rustdoc::broken_intra_doc_links))]
#![cfg_attr(feature = "linting", deny(rustdoc::bare_urls))]
//---------------------------------------------------------------------

#[allow(unused_extern_crates)]
extern crate rutil;

use rutil::{debug, report};

use sochi::cli;
use sochi::tools::confuzzius;
use sochi::tools::ilf;
use sochi::tools::mythril;
use sochi::tools::slither;

/// Global variable which enables the printing of debugging message.
pub static mut DEBUG_MODE: bool = true;

/// Apply debugging flags.
pub fn apply_debugging_flags(debug_mode: bool) {
    unsafe {
        debug::DEBUG_MODE = debug_mode;
        debug::DEEP_DEBUG_MODE = debug_mode;
    }
}

/// Main function of Visualizer.
fn main() {
    // Preconfigurations
    report::override_panic_message();

    // Parse command line arguments
    let matches = cli::configure_arguments();
    let opts = cli::parse_argument_matches(&matches);
    apply_debugging_flags(opts.printer_options.debug_mode);

    let tools = opts.printer_options.tools;
    if tools.contains(&cli::ToolName::Slither) {
        if opts.printer_options.generate_results {
            slither::generate_results(opts.input_dir);
        } else {
            let result = slither::interpret_results(opts.input_dir);
            println!("Slither results: {}", result);
        }
    }

    if tools.contains(&cli::ToolName::Confuzzius) {
        if opts.printer_options.generate_results {
            confuzzius::generate_results(opts.input_dir);
        } else {
            let result = confuzzius::interpret_results(opts.input_dir);
            println!("Confuzzius results: {}", result);
        }
    }

    if tools.contains(&cli::ToolName::ILF) {
        if opts.printer_options.generate_commands {
            ilf::generate_commands(opts.input_dir);
        }
    }

    // Note: For mythril, the running has to be done manually.
    // There is an error we cannot fix to run mythril automatically.
    if tools.contains(&cli::ToolName::Mythril) {
        if opts.printer_options.generate_commands {
            mythril::generate_commands(opts.input_dir);
        } else {
            let result = mythril::interpret_results(opts.input_dir);
            println!("Mythril results: {}", result);
        }
    }
}
