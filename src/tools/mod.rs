//! Module for tools used by `sochi`

pub mod slither;
use std::fmt::{Display, Formatter, Result};

/// Executable file name of `Slither`
pub const SLITHER: &str = "slither";

/// Executable file name of the Solc compiler
pub const SOLC: &str = "solc";

/// Executable file name of `solc-select`
pub const SOLC_SELECT: &str = "solc-select";

/// Data structure for a list of bugs
#[derive(Debug)]
pub struct Summary {
    /// Re-entrancy
    pub re_entrancy: usize,

    /// Timestamp dependency
    pub timestamp: usize,

    /// Unhandled exceptions
    pub unhandled_exceptions: usize,

    /// Tx_origin
    pub tx_origin: usize,
}

/// Implement functions for `Summary`
impl Summary {
    /// Constructor
    pub fn new(
        re_entrancy: usize,
        timestamp: usize,
        unhandled_exceptions: usize,
        tx_origin: usize,
    ) -> Summary {
        Summary {
            re_entrancy,
            timestamp,
            unhandled_exceptions,
            tx_origin,
        }
    }
}

/// Implement the `Display` trait for `Summary`.
impl Display for Summary {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let _ = write!(f, "\nReentrancy: {}", self.re_entrancy);
        let _ = write!(f, "\nTimestamp Dependency: {}", self.timestamp);
        write!(f, "\nTx_origin: {}\n", self.tx_origin)
    }
}
