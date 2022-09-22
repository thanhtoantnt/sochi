//! Macros used by the Verazt project

/// Macro to get the current function name.
#[allow(unused_imports)]
#[macro_export]
macro_rules! function {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);

        // // Extract function name and the parent module/trait.
        // match &name[..name.len() - 3].rfind("::") {
        //     Some(pos1) => match &name[..*pos1].rfind("::") {
        //         Some(pos2) => &name[pos2 + 2..name.len() - 3],
        //         None => &name[pos1 + 2..name.len() - 3],
        //     },

        //     None => &name[..name.len() - 3],
        // }

        &name[..name.len() - 3]
    }};
}

/// Macro to format a string with indentation for each line.
#[allow(unused_imports, unused_unsafe)]
// #[allow_internal_unstable(format_args_nl)]
#[macro_export]
macro_rules! formati {
    ($indent:expr, $($arg:tt)*) => {
        unsafe {
            use std::fmt::Write as FmtWrite;
            use $crate::report;
            let tw = report::get_terminal_width() -
                $crate::debug::DEBUG_MARKER_LEN - 3;
            // let msg = std::fmt::format(std::format_args_nl!($($arg)*));
            let mut msg = String::new();
            let _ = writeln!(msg, $($arg)*);
            report::beautify_string("", false, $indent, "", &msg, tw)
        }
    }
}

/// Macro to format a string with a prefix message for console printing.
#[allow(unused_imports, unused_unsafe)]
#[macro_export]
macro_rules! formatp {
    ($lindent:expr, $rindent:expr, $prefix:expr, $($arg:tt)*) => {
        unsafe {
            use std::fmt::Write as FmtWrite;
            use $crate::report;
            let tw = report::get_terminal_width() -
                $crate::debug::DEBUG_MARKER_LEN - 3 - $rindent;
            // let msg = std::fmt::format(std::format_args_nl!($($arg)*));
            let mut msg = String::new();
            let _ = writeln!(msg, $($arg)*);
            report::beautify_string("", false, $lindent, $prefix, &msg, tw)
        }
    }
}

/// Override the default print! macro to disable printing when needed.
#[allow(unused_imports, unused_unsafe)]
#[macro_export]
macro_rules! print {
    () => {
        unsafe {
            if !$crate::debug::DISABLE_PRINTING {
                std::print!("")
            }
        }
    };
    ($($arg:tt)*) => {
        unsafe {
            use $crate::report;
            if !$crate::debug::DISABLE_PRINTING {
                if $crate::debug::DEBUG_MODE {
                    let marker = "[inf] ";
                    $crate::debug::DEBUG_MARKER_LEN = marker.len();
                    let msg = std::fmt::format(std::format_args!($($arg)*));
                    let tw = report::get_terminal_width();
                    let msg = report::beautify_string(marker, false, 0, "", &msg, tw);
                    std::print!("{}", msg);
                    $crate::debug::DEBUG_MARKER_LEN = 0
                }
                else {
                    std::io::_print($crate::format_args_nl!($($arg)*));
                }
            }
        }
    }
}

/// Override the default println! macro to disable printing when needed.
#[allow(unused_imports, unused_unsafe)]
// #[allow_internal_unstable(format_args_nl)]
#[macro_export]
macro_rules! println {
    () => {
        unsafe {
            if !$crate::debug::DISABLE_PRINTING {
                std::println!("")
            }
        }
    };
    ($($arg:tt)*) => {
        unsafe {
            use std::fmt::Write as FmtWrite;
            use $crate::report;
            if !$crate::debug::DISABLE_PRINTING {
                if $crate::debug::DEBUG_MODE {
                    let marker = "[inf] ";
                    $crate::debug::DEBUG_MARKER_LEN = marker.len();
                    let msg = std::fmt::format(std::format_args!($($arg)*));
                    let tw = report::get_terminal_width();
                    let msg = report::beautify_string(marker, false, 0, "", &msg, tw);
                    std::println!("{}", msg);
                    $crate::debug::DEBUG_MARKER_LEN = 0
                }
                else {
                    // std::io::_print($crate::format_args_nl!($($arg)*));
                    let mut msg = String::new();
                    let _ = writeln!(msg, $($arg)*);
                    std::print!("{}", msg);
                }
            }
        }
    }
}

/// Macro to print messages with the H0 separator level (=====).
#[allow(unused_imports)]
#[macro_export]
macro_rules! print_header_0 {
    ($($arg:tt)*) => {
        unsafe {
            if !$crate::debug::DISABLE_PRINTING {
                let mut ruler = "=".repeat(55);
                if $crate::debug::DEBUG_MODE {
                    ruler = "[inf] ".to_owned() + &ruler;
                };
                std::println!("{}", ruler);
                $crate::println!($($arg)*);
            }
        }
    }
}

/// Macro to print messages with the H1 separator level (-----).
#[allow(unused_imports, unused_unsafe)]
#[macro_export]
macro_rules! print_header_1 {
    ($($arg:tt)*) => {
        unsafe {
            if !$crate::debug::DISABLE_PRINTING {
                let mut ruler = "-".repeat(36);
                if $crate::debug::DEBUG_MODE {
                    ruler = "[inf] ".to_owned() + &ruler;
                };
                std::println!("{}", ruler);
                $crate::println!($($arg)*);
            }
        }
    }
}

/// Macro to print messages with the H2 separator level (-----).
#[allow(unused_imports, unused_unsafe)]
#[macro_export]
macro_rules! print_header_2 {
    ($($arg:tt)*) => {
        unsafe {
            if !$crate::debug::DISABLE_PRINTING {
                let mut ruler = "-".repeat(23);
                if $crate::debug::DEBUG_MODE {
                    ruler = "[inf] ".to_owned() + &ruler;
                };
                std::println!("{}", ruler);
                $crate::println!($($arg)*);
            }
        }
    }
}

/// Override the default todo! macro to print todo message and logging location.
#[allow(unused_imports, unused_unsafe)]
// #[allow_internal_unstable(format_args_nl)]
#[macro_export]
macro_rules! todo {
    () => {
        unsafe {
            use std::fmt::Write as FmtWrite;
            use $crate::report;
            use $crate::debug;
            if !debug::DISABLE_PRINTING {
                let marker = if debug::DEBUG_MODE { "[!!!]" } else { "" };
                debug::DEBUG_MARKER_LEN = marker.len();
                let msg = "TODO: Not yet implemented!\n";
                let tw = report::get_terminal_width();
                let func = std::format!("{}", $crate::function!());
                let file = std::format!("{}:{}:{}",
                                        std::file!(), std::line!(),
                                        std::column!());
                let msg = report::beautify_string(marker, true, 0, "", &msg, tw) +
                    "\n" +
                    &report::log_function_name("", 6, &func, tw) +
                    "\n" +
                    &report::log_file_name("", 6, &file, tw);
                std::println!("{}", msg);
                debug::DEBUG_MARKER_LEN = 0
            }
        }
    };
    ($($arg:tt)*) => {
        unsafe {
            use std::fmt::Write as FmtWrite;
            use $crate::report;
            use $crate::debug;
            if !$crate::debug::DISABLE_PRINTING {
                let marker = if debug::DEBUG_MODE { "[!!!]" } else { "" };
                let mkr_len = marker.len();
                debug::DEBUG_MARKER_LEN = marker.len();
                // let msg = "TODO: ".to_owned() +
                //     &std::fmt::format(bstd::format_args_nl!($($arg)*));
                let mut msg = "TODO: ".to_owned();
                let _ = writeln!(msg, $($arg)*);
                let tw = report::get_terminal_width();
                let func = format!("{}", $crate::function!());
                let file = format!("{}:{}:{}",
                                    std::file!(), std::line!(),
                                    std::column!());
                let msg = report::beautify_string(marker, false, 0, "", &msg, tw) +
                    "\n" +
                    &report::log_function_name("", mkr_len, &func, tw) +
                    "\n" +
                    &report::log_file_name("", mkr_len, &file, tw);
                std::println!("{}", msg);
                debug::DEBUG_MARKER_LEN = 0
            }
        }
    }
}

/// Core macro to print a debugging message.
#[allow(unused_imports, unused_unsafe)]
#[macro_export]
macro_rules! debug_core {
    ($marker:expr, $indent:expr, $prefix:expr, $($arg:tt)*) => {
        // unsafe {
            use $crate::report;
            use std::fmt::Write as FmtWrite;
            $crate::debug::DEBUG_MARKER_LEN = $marker.len();
            // let msg = std::fmt::format(std::format_args_nl!($($arg)*));
            let mut msg = String::new();
            let _ = writeln!(msg, $($arg)*);
            let tw = report::get_terminal_width();
            let msg = report::beautify_string($marker, false, $indent, $prefix,
                                              &msg, tw);
            std::println!("{}", msg);
            $crate::debug::DEBUG_MARKER_LEN = 0
        // }
    }
}

/// Macro to print a debugging message.
#[allow(unused_imports, unused_unsafe)]
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        unsafe {
            use $crate::debug;
            if debug::DEBUG_MODE && !debug::DISABLE_PRINTING {
                $crate::debug_core!("[dbg] ", 0, "", $($arg)*);
            }
        }
    }
}

/// Macro to print a deep-debugging message.
#[macro_export]
#[allow(unused_imports, unused_unsafe)]
macro_rules! ddebug {
    ($($arg:tt)*) => {
        unsafe {
            use $crate::debug;
            if debug::DEEP_DEBUG_MODE && !debug::DISABLE_PRINTING {
                $crate::debug_core!("[dbx] ", 0, "", $($arg)*);
            }
        }
    }
}

/// Macro to print a debugging message with indented lines.
#[allow(unused_imports, unused_unsafe)]
#[macro_export]
macro_rules! debugi {
    ($indent:expr, $($arg:tt)*) => {
        unsafe {
            use $crate::debug;
            if debug::DEBUG_MODE && !debug::DISABLE_PRINTING {
                $crate::debug_core!("[dbg] ", $indent, "", $($arg)*);
            }
        }
    }
}

/// Macro to print a deep-debugging message with indented lines.
#[allow(unused_imports, unused_unsafe)]
#[macro_export]
macro_rules! ddebugi {
    ($indent:expr, $($arg:tt)*) => {
        unsafe {
            use $crate::debug;
            if debug::DEEP_DEBUG_MODE && !debug::DISABLE_PRINTING {
                $crate::debug_core!("[dbx] ", $indent, "", $($arg)*);
            }
        }
    }
}

/// Macro to print a debugging message which is indented with a prefix message.
#[allow(unused_imports, unused_unsafe)]
#[macro_export]
macro_rules! debugp {
    ($indent:expr, $prefix:expr, $($arg:tt)*) => {
        unsafe {
            use $crate::debug;
            if debug::DEBUG_MODE && !debug::DISABLE_PRINTING {
                $crate::debug_core!("[dbg] ", $indent, $prefix, $($arg)*);
            }
        }
    }
}

/// Macro to print a deep-debugging message which is indented with a prefix
/// message.
#[allow(unused_imports, unused_unsafe)]
#[macro_export]
macro_rules! ddebugp {
    ($indent:expr, $prefix:expr, $($arg:tt)*) => {
        unsafe {
            use $crate::debug;
            if debug::DEEP_DEBUG_MODE && !debug::DISABLE_PRINTING {
                $crate::debug_core!("[dbx] ", $indent, $prefix, $($arg)*);
            }
        }
    }
}

/// Macro to print debugging messages with the H0 separator level (=====).
#[allow(unused_imports, unused_unsafe)]
#[macro_export]
macro_rules! debug_header_0 {
    ($($arg:tt)*) => {
        unsafe {
            use $crate::debug;
            if debug::DEBUG_MODE && !debug::DISABLE_PRINTING {
                let ruler = &"=".repeat(55);
                let ruler = "[dbg] ".to_owned() + ruler;
                std::println!("{}", ruler);
                $crate::debug!($($arg)*);
                std::print!("");
            }
        }
    }
}

/// Macro to print deep-debugging messages with the H0 separator level (=====).
#[allow(unused_imports, unused_unsafe)]
#[macro_export]
macro_rules! ddebug_header_0 {
    ($($arg:tt)*) => {
        unsafe {
            use $crate::debug;
            if debug::DEEP_DEBUG_MODE && !debug::DISABLE_PRINTING {
                let ruler = &"=".repeat(55);
                let ruler = "[dbx] ".to_owned() + ruler;
                std::println!("{}", ruler);
                $crate::ddebug!($($arg)*);
                std::print!("");
            }
        }
    }
}

/// Macro to print debugging messages with the H1 separator level (-----).
#[allow(unused_imports, unused_unsafe)]
#[macro_export]
macro_rules! debug_header_1 {
    ($($arg:tt)*) => {
        unsafe {
            use $crate::debug;
            if debug::DEBUG_MODE && !debug::DISABLE_PRINTING {
                let ruler = &"-".repeat(36);
                let ruler = "[dbg] ".to_owned() + ruler;
                std::println!("{}", ruler);
                $crate::debug!($($arg)*);
                std::print!("");
            }
        }
    }
}

/// Macro to print deep-debugging messages with the H1 separator level (-----).
#[allow(unused_imports, unused_unsafe)]
#[macro_export]
macro_rules! ddebug_header_1 {
    ($($arg:tt)*) => {
        unsafe {
            use $crate::debug;
            if debug::DEEP_DEBUG_MODE && !debug::DISABLE_PRINTING {
                let ruler = &"-".repeat(36);
                let ruler = "[dbx] ".to_owned() + ruler;
                std::println!("{}", ruler);
                $crate::ddebug!($($arg)*);
                std::print!("");
            }
        }
    }
}

/// Macro to print debugging messages with the H2 separator level (---).
#[allow(unused_imports, unused_unsafe)]
#[macro_export]
macro_rules! debug_header_2 {
    ($($arg:tt)*) => {
        unsafe {
            use $crate::report;
            if $crate::debug::DEBUG_MODE && !$crate::debug::DISABLE_PRINTING {
                let ruler = &"-".repeat(23);
                let ruler = "[dbg] ".to_owned() + ruler;
                std::println!("{}", ruler);
                $crate::debug!($($arg)*);
                std::print!("");
            }
        }
    }
}

/// Macro to print deep-debugging messages with the H2 separator level (---).
#[allow(unused_imports, unused_unsafe)]
#[macro_export]
macro_rules! ddebug_header_2 {
    ($($arg:tt)*) => {
        unsafe {
            use $crate::debug;
            if debug::DEEP_DEBUG_MODE && !debug::DISABLE_PRINTING {
                let ruler = &"-".repeat(23);
                let ruler = "[dbx] ".to_owned() + ruler;
                std::println!("{}", ruler);
                $crate::ddebug!($($arg)*);
                std::print!("");
            }
        }
    }
}

/// Macro to print a fixme message.
#[allow(unused_imports, unused_unsafe)]
// #[allow_internal_unstable(format_args_nl)]
#[macro_export]
macro_rules! fixme {
    () => {
        unsafe {
            if $crate::debug::DEBUG_MODE && !$crate::debug::DISABLE_PRINTING {
                std::print!("\n")
            }
        }
    };
    ($($arg:tt)*) => {
        unsafe {
            use std::fmt::Write as FmtWrite;
            use $crate::report;
            use $crate::debug;
            if !debug::DISABLE_PRINTING {
                let marker = if debug::DEBUG_MODE { "[!!!]" } else { "" };
                debug::DEBUG_MARKER_LEN = marker.len();
                let mut msg = "FIXME: ".to_owned();
                let _ = writeln!(msg, $($arg)*);
                let tw = report::get_terminal_width();
                let func = std::format!("{}", $crate::function!());
                let file = std::format!("{}:{}:{}",
                                        std::file!(), std::line!(),
                                        std::column!());
                let msg = report::beautify_string(marker, true, 0, "", &msg, tw) +
                    "\n" +
                    &report::log_function_name("", 6, &func, tw) +
                    "\n" +
                    &report::log_file_name("", 6, &file, tw);
                std::println!("{}", msg);
                debug::DEBUG_MARKER_LEN = 0
            }
        }
    }
}

/// Macro to print a warning message.
#[allow(unused_imports, unused_unsafe)]
// #[allow_internal_unstable(format_args_nl)]
#[macro_export]
macro_rules! warning {
    () => {
        unsafe {
            if !$crate::debug::DISABLE_PRINTING {
                std::print!("\n")
            }
        }
    };
    ($($arg:tt)*) => {
        unsafe {
            use std::fmt::Write as FmtWrite;
            use $crate::report;
            use $crate::debug;
            if !debug::DISABLE_PRINTING {
                let marker = if debug::DEBUG_MODE { "[WRN]" } else { "" };
                debug::DEBUG_MARKER_LEN = marker.len();
                // let msg = std::fmt::format(std::format_args_nl!($($arg)*));
                let mut msg = String::new();
                let _ = writeln!(msg, $($arg)*);
                let tw = report::get_terminal_width();
                let func = std::format!("{}", $crate::function!());
                let file = std::format!("{}:{}:{}",
                                        std::file!(), std::line!(),
                                        std::column!());
                let msg = report::beautify_string(marker, true, 0, "", &msg, tw) +
                    "\n" +
                    &report::log_function_name("", 6, &func, tw) +
                    "\n" +
                    &report::log_file_name("", 6, &file, tw);
                std::println!("{}", msg);
                debug::DEBUG_MARKER_LEN = 0
            }
        }
    }
}

/// Macro to print an error and exit the program.
#[allow(unused_imports)]
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => ({
        std::println!($($arg)*);
        std::process::exit(1)
    })
}

/// Short-hand expression for it-then-else.
#[macro_export]
macro_rules! ite {
    ($test:expr, $true_expr:expr, $false_expr:expr) => {
        if $test {
            $true_expr
        } else {
            $false_expr
        }
    };
}

/// Compare output in stdout to a string
#[macro_export]
macro_rules! assert_stdout_eq {
    ($test:expr, $expected:expr) => {{
        use gag::BufferRedirect;
        use std::io::Read;

        let mut buf = BufferRedirect::stdout().unwrap();

        $test;

        let mut output = String::new();
        buf.read_to_string(&mut output).unwrap();
        drop(buf);

        assert_eq!(&output[..], $expected);
    }};
}
