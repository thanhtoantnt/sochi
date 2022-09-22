//! Core command line arguments for all tools

use clap::Arg;

/// Trait to create a new argument
pub trait ArgExt<'a> {
    /// Create an argument with long argument of the same name
    fn new_argument(name: &'a str) -> Self;
}

/// Implement trait `ArgExt` for Arg
impl<'a> ArgExt<'a> for Arg<'a> {
    fn new_argument(name: &'a str) -> Self {
        Arg::new(name).long(name)
    }
}
