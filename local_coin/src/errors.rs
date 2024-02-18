use std::{fmt, io};
use thiserror::Error;

use serde::ser::StdError;

/// Errors that can occur when trying to resolve the position of an address from a file
#[derive(Error, Debug)]
pub enum AddressPositionError {
    /// Indicates that no index was found for the given address and value combination.
    ///
    /// - `String`: The address for which the index was not found.
    /// - `i64`: The value for which the index was not found.
    #[error("no matching index found for address: {0}, value: {1}")]
    NoMatchingIndexForValue(String, f64),

    /// Indicates that no address was found that matches the provided address.
    ///
    /// - `String`: The address which was not found.
    #[error("no matching address found: {0}")]
    NoMatchingAddress(String),

    /// Indicates that no indices were found for the given address with a specific value.
    #[error("no matching indices found for {address} with value: {value}")]
    NoMatchingIndices {
        /// The address for which the indices were not found.
        address: String,
        /// The specific value for which the indices were not found
        value: String,
    },
}
/// Errors that can occur while executing a command in another directory.
#[derive(Debug)]
pub enum CommandError {
    /// Represents errors encountered when attempting to set the directory for a command.
    ///
    /// Contains the underlying `std::io::Error` for detailed diagnostics.
    SetDirError(std::io::Error),

    /// Represents errors encountered during the execution of a command.
    ///
    /// Contains the underlying `std::io::Error` for detailed diagnostics.
    CommandError(std::io::Error),

    /// Represents errors encountered when attempting to reset to the original directory after a command.
    ///
    /// Contains the underlying `std::io::Error` for detailed diagnostics.
    ResetDirError(std::io::Error),
}
impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandError::SetDirError(e) => write!(f, "Set Directory Error: {}", e),
            CommandError::CommandError(e) => write!(f, "Command Error: {}", e),
            CommandError::ResetDirError(e) => write!(f, "Reset Directory Error: {}", e),
        }
    }
}

impl StdError for CommandError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            CommandError::SetDirError(e) => Some(e),
            CommandError::CommandError(e) => Some(e),
            CommandError::ResetDirError(e) => Some(e),
        }
    }
}

impl From<io::Error> for CommandError {
    fn from(error: io::Error) -> Self {
        CommandError::CommandError(error)
    }
}
