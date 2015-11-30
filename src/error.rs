//! Our error enum.

use std::error;
use std::ffi::OsString;
use std::fmt;
use std::num::{ParseFloatError, ParseIntError};
use std::str::ParseBoolError;

use las;
#[cfg(feature = "rxp-source")]
use rivlib;
use sdc;
#[cfg(feature = "sdf-source")]
use sdf;
use toml;

/// Our custom error handling type.
#[derive(Debug)]
pub enum Error {
    /// Invalid configuration for a source or sink.
    Configuration(String),
    /// Wrapper around `toml::DecodeError`.
    Decode(toml::DecodeError),
    /// A point is missing a dimension that is required by someone else, usually a `Sink`.
    MissingDimension(String),
    /// A wrapper around a las error.
    Las(las::Error),
    /// A wrapper around `std::str::ParseBoolError`.
    ParseBool(ParseBoolError),
    /// A wrapper around `std::num::ParseIntError`.
    ParseInt(ParseIntError),
    /// A wrapper around `std::num::ParseFloatError`.
    ParseFloat(ParseFloatError),
    #[cfg(feature = "rxp-source")]
    /// A wrapper around an rxp error.
    Rxp(rivlib::Error),
    /// A wrapper around an sdc error.
    Sdc(sdc::Error),
    #[cfg(feature = "sdf-source")]
    /// A wrapper around an sdf error.
    Sdf(sdf::Error),
    /// Unregistered file extension when intuiting a source or sink type.
    UnregisteredFileExtension(OsString),
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Configuration(_) => "configuration error",
            Error::Decode(ref err) => err.description(),
            Error::MissingDimension(_) => "missing dimension",
            Error::Las(ref err) => err.description(),
            Error::ParseBool(ref err) => err.description(),
            Error::ParseInt(ref err) => err.description(),
            Error::ParseFloat(ref err) => err.description(),
            #[cfg(feature = "rxp-source")]
            Error::Rxp(ref err) => err.description(),
            Error::Sdc(ref err) => err.description(),
            #[cfg(feature = "sdf-source")]
            Error::Sdf(ref err) => err.description(),
            Error::UnregisteredFileExtension(_) => "unregistered file extension",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Decode(ref err) => Some(err),
            Error::Las(ref err) => Some(err),
            Error::ParseBool(ref err) => Some(err),
            Error::ParseInt(ref err) => Some(err),
            Error::ParseFloat(ref err) => Some(err),
            #[cfg(feature = "rxp-source")]
            Error::Rxp(ref err) => Some(err),
            Error::Sdc(ref err) => Some(err),
            #[cfg(feature = "sdf-source")]
            Error::Sdf(ref err) => Some(err),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Configuration(ref s) => write!(f, "Configuration error: {}", s),
            Error::Decode(ref err) => write!(f, "Decode error: {}", err),
            Error::MissingDimension(ref s) => write!(f, "Missing dimension: {}", s),
            Error::Las(ref err) => write!(f, "las error: {}", err),
            Error::ParseBool(ref err) => write!(f, "Parse bool error: {}", err),
            Error::ParseInt(ref err) => write!(f, "Parse int error: {}", err),
            Error::ParseFloat(ref err) => write!(f, "Parse float error: {}", err),
            #[cfg(feature = "rxp-source")]
            Error::Rxp(ref err) => write!(f, "rxp error: {}", err),
            Error::Sdc(ref err) => write!(f, "sdc error: {}", err),
            #[cfg(feature = "sdf-source")]
            Error::Sdf(ref err) => write!(f, "sdf error: {}", err),
            Error::UnregisteredFileExtension(ref s) =>
                write!(f, "Unregistered file extension: {}", s.to_string_lossy()),
        }
    }
}

impl From<toml::DecodeError> for Error {
    fn from(err: toml::DecodeError) -> Error {
        Error::Decode(err)
    }
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

impl From<ParseIntError> for Error {
    fn from(err: ParseIntError) -> Error {
        Error::ParseInt(err)
    }
}

impl From<ParseFloatError> for Error {
    fn from(err: ParseFloatError) -> Error {
        Error::ParseFloat(err)
    }
}

#[cfg(feature = "rxp-source")]
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

#[cfg(feature = "sdf-source")]
impl From<sdf::Error> for Error {
    fn from(err: sdf::Error) -> Error {
        Error::Sdf(err)
    }
}
