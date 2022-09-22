//! Module to store mutable flags, used by all packages

/// Global variable which enables the printing of debugging message.
pub static mut DEBUG_MODE: bool = true;

/// Global variable which enables the printing of deep debugging message.
pub static mut DEEP_DEBUG_MODE: bool = true;

/// Global variable which disables all printing functions.
pub static mut DISABLE_PRINTING: bool = false;

/// Global variable storing the debugging marker length in debug mode.
pub static mut DEBUG_MARKER_LEN: usize = 0;
