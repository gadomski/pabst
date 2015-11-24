//! Our error enum.

use std::num::ParseFloatError;
use std::str::ParseBoolError;

use las;
#[cfg(feature = "rxp")]
use rivlib;
use sdc;
#[cfg(feature = "sdf-convert")]
use sdf;

/// Our custom error handling type.
#[derive(Debug)]
pub enum Error {
    /// Invalid options to a source or sink.
    InvalidOption(String),
    /// A point is missing a dimension that is required by someone else, usually a `Sink`.
    MissingDimension(String),
    /// A wrapper around a las error.
    Las(las::Error),
    /// A wrapper around `std::str::ParseBoolError`.
    ParseBool(ParseBoolError),
    /// A wrapper around `std::num::ParseFloatError`.
    ParseFloat(ParseFloatError),
    #[cfg(feature = "rxp")]
    /// A wrapper around an rxp error.
    Rxp(rivlib::Error),
    /// A wrapper around an sdc error.
    Sdc(sdc::Error),
    #[cfg(feature = "sdf-convert")]
    /// A wrapper around an sdf error.
    Sdf(sdf::Error),
    /// The type of sink could not be determined.
    UndefinedSink,
    /// The type of source could not be determined.
    UndefinedSource,
}

impl From<las::Error> for Error {
    fn from(err: las::Error) -> Error {
        Error::Las(err)
    }
}

impl From<ParseBoolError> for Error {
    fn from(err: ParseBoolError) -> Error {
        Error::ParseBool(err)
    }
}

impl From<ParseFloatError> for Error {
    fn from(err: ParseFloatError) -> Error {
        Error::ParseFloat(err)
    }
}

#[cfg(feature = "rxp")]
impl From<rivlib::Error> for Error {
    fn from(err: rivlib::Error) -> Error {
        Error::Rxp(err)
    }
}

impl From<sdc::Error> for Error {
    fn from(err: sdc::Error) -> Error {
        Error::Sdc(err)
    }
}

#[cfg(feature = "sdf-convert")]
impl From<sdf::Error> for Error {
    fn from(err: sdf::Error) -> Error {
        Error::Sdf(err)
    }
}
