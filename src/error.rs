//! Our error enum.

use std::error;

#[derive(Debug)]
pub enum Error {
    MissingDimension(String),
    Upstream(Box<error::Error>),
}
