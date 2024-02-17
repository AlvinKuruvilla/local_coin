use thiserror::Error;

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
