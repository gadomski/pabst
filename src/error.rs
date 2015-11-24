//! Our error enum.

use std::error;

/// Our custom error handling type.
#[derive(Debug)]
pub enum Error {
    /// A point is missing a dimension that is required by someone else, usually a `Sink`.
    MissingDimension(String),
    /// A generic wrapper around an upstream error. We don't create an error wrapper for each type
    /// of upstream error so that we can keep our upstream definitions contained to a few files.
    Upstream(Box<error::Error>),
}
