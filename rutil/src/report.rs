//! Module containing printing and reporting features.

use std::cmp;
use std::fmt::Write;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use super::string::StringExt;

/// Length of a long ruler.
const LONG_SEPARATOR_LENGTH: usize = 75;

/// Length of a short ruler.
const SHORT_SEPARATOR_LENGTH: usize = 55;

/// Printing a reporting message in the format header-separator-body.
pub fn print_message(header: &str, body: &str) {
    let separator = "-".repeat(cmp::max(header.len(), LONG_SEPARATOR_LENGTH));
    println!("{}\n{}\n{}", header, separator, body);
}

/// Printing a long separator (a line of the `=` charater) to stdout.
pub fn print_long_separator() {
    let ruler = "=".repeat(LONG_SEPARATOR_LENGTH);
    println!("{}\n", ruler);
}

/// Printing a short separator (a line of the `=` charater) to stdout.
pub fn print_short_separator() {
    let ruler = "=".repeat(SHORT_SEPARATOR_LENGTH);
    println!("{}\n", ruler);
}

/// Print a code fragment and line, column position.
pub fn print_code_fragment_and_position(
    filename: &str,
    line: u32,
    column: u32,
) {
    let begin_line = if line > 4 { line - 3 } else { 0 };
    let begin_col = if column > 2 { column - 2 } else { 0 };
    let file = match File::open(filename) {
        Ok(res) => res,
        Err(_) => {
            panic!("File {} not found!", filename);
        }
    };
    let reader = BufReader::new(file);
    let mut lines = reader.lines().skip(begin_line.try_into().unwrap());
    let mut res = String::new();
    // lines.next returns: Option<Result<String, std::io::Error>>`
    for i in 1..line + 3 - begin_line {
        let curr_line = lines
            .next()
            .unwrap_or_else(|| Ok("".to_string()))
            .unwrap_or_else(|_| "".to_string());
        let _ = write!(res, "  {}|", (i + begin_line));

        if i + begin_line == line {
            let _ = writeln!(res, ">{}", curr_line);
            let _ = writeln!(
                res,
                "  {}|>{}^^^",
                " ".repeat((i + begin_line).to_string().len()),
                " ".repeat(begin_col.try_into().unwrap())
            );
        } else {
            let _ = writeln!(res, "{}", curr_line);
        }
    }
    println!(
        "Code segment:\n{}Location: {}:{}\nFile: {}",
        res, line, column, filename
    );
}

/// Format an output message for printing to console.
pub fn indent_lines(msg: String, indent: usize) -> String {
    msg.indent(indent)
}

/// Get line width of the console terminal.
pub fn get_terminal_width() -> usize {
    match termsize::terminal_size() {
        None => 80,
        Some((termsize::Width(w), _)) => w as usize - 3,
    }
}

/// Beautify a string to be printed to the terminal.
pub fn beautify_string(
    marker: &str,
    marker_continuation: bool,
    indent: usize,
    prefix: &str,
    output: &str,
    line_width: usize,
) -> String {
    let marker_len = marker.len();
    let prefix_len = prefix.len();
    let text_width = line_width - marker_len - prefix_len - indent;

    // Wrap text by a max line width
    let wrap_options = textwrap::Options::new(text_width)
        .word_separator(textwrap::WordSeparator::AsciiSpace)
        .word_splitter(textwrap::WordSplitter::NoHyphenation);
    let res = textwrap::wrap(output, &wrap_options).join("\n");

    // Indent tail lines by 1 space if the string is bracket enclosed
    let res = match res.is_bracket_enclosed() {
        true => res.indent_tail_lines(1),
        false => res,
    };

    // Prepend marker, indentation, and prefix for the first line
    let res = marker.to_owned() + &" ".repeat(indent) + prefix + &res;

    // Finally, prepend marker continuation and indentation for other lines
    let tail_line_indent = marker_len + prefix_len + indent;
    match marker_continuation {
        true => res.indent_and_prefix_tail_lines(tail_line_indent - 1, "|"),
        false => res.indent_tail_lines(tail_line_indent),
    }
}

/// Format a function name to `String` for logging purpose.
pub fn log_function_name(
    marker: &str,
    indent: usize,
    output: &str,
    line_width: usize,
) -> String {
    let marker_len = marker.len();
    let msg_prefix = "Function: ";
    let line_prefix = "| ";
    let mpref_len = msg_prefix.len();
    let lpref_len = line_prefix.len();
    let text_width = line_width - marker_len - mpref_len - lpref_len - indent;

    /// Function to split a long function name
    fn split_word(word: &str) -> Vec<usize> {
        word.match_indices("::").map(|(idx, _)| idx + 2).collect()
    }

    // Setup wrapping option
    let wrap_options = textwrap::Options::new(text_width)
        .word_separator(textwrap::WordSeparator::AsciiSpace)
        .word_splitter(textwrap::WordSplitter::Custom(split_word));

    // Wrap text by a max line width and prepend line_prefix
    let res = textwrap::wrap(output, &wrap_options)
        .iter()
        .enumerate()
        .map(|(i, line)| {
            let line = if i == 0 {
                msg_prefix.to_owned() + line
            } else {
                " ".repeat(mpref_len) + line
            };
            " ".repeat(indent) + line_prefix + &line
        })
        .collect::<Vec<String>>()
        .join("\n");

    // Return
    res
}

/// Format a file name to `String` for logging purpose.
pub fn log_file_name(
    marker: &str,
    indent: usize,
    output: &str,
    line_width: usize,
) -> String {
    let marker_len = marker.len();
    let msg_prefix = "File: ";
    let mpref_len = msg_prefix.len();
    let line_prefix = "| ";
    let lpref_len = line_prefix.len();
    let text_width = line_width - marker_len - mpref_len - lpref_len - indent;

    /// Function to split a long file name
    fn split_word(word: &str) -> Vec<usize> {
        word.match_indices('/').map(|(idx, _)| idx + 2).collect()
    }

    // Set up wrapping option
    let wrap_options = textwrap::Options::new(text_width)
        .word_separator(textwrap::WordSeparator::AsciiSpace)
        .word_splitter(textwrap::WordSplitter::Custom(split_word));

    // Wrap text by a max line width and prepend line_prefix
    let res = textwrap::wrap(output, &wrap_options)
        .iter()
        .enumerate()
        .map(|(i, line)| {
            let line = if i == 0 {
                msg_prefix.to_owned() + line
            } else {
                " ".repeat(mpref_len) + line
            };
            " ".repeat(indent) + line_prefix + &line
        })
        .collect::<Vec<String>>()
        .join("\n");

    // Return
    res
}

/// Function to override the panicking message.
pub fn override_panic_message() {
    // Replace panic message when backtrace is disabled.
    if std::env::var("RUST_BACKTRACE").is_err() {
        std::panic::set_hook(Box::new(|panic_info| {
            let reason = {
                let info = panic_info.to_string();

                // TODO: below is a hack to extract only the panic message.
                //
                // It should be changed to `panic_info.message()` when the
                // function `.message()` is available in Rust stable channel.
                let index = match info.rfind(',') {
                    Some(index) => index,
                    None => info.len(),
                };
                info[..index].to_owned()
            };

            let thread = std::thread::current();
            let thread = thread.name().unwrap_or("unknown");

            let term_width = get_terminal_width();
            // let func = std::format!("{},", panic_function!());
            let file = match panic_info.location() {
                Some(loc) => std::format!(
                    "{}:{}:{}",
                    loc.file(),
                    loc.line(),
                    loc.column()
                ),
                None => "unknown file".to_owned(),
            };

            let tw = get_terminal_width();
            let msg = format!("Thread '{}' {}", thread, reason);
            let msg = ite!(msg.ends_with('.'), msg, msg + ".");

            let msg = beautify_string("[Error] ", true, 0, "", &msg, tw);

            let msg = msg
                + "\n"
                + &log_file_name("", 0, &file, term_width)
                + "\n"
                + "| Note: run with `RUST_BACKTRACE=1` to display backtrace.";
            std::println!("{}", msg)
        }));
    }
}
