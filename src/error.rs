//! Our error enum.

use std::error;

#[derive(Debug)]
pub enum Error {
    Upstream(Box<error::Error>),
}
