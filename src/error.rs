//! Our error enum.

use las;
#[cfg(feature = "rxp")]
use rivlib;
use sdc;
#[cfg(feature = "sdf-convert")]
use sdf;

/// Our custom error handling type.
#[derive(Debug)]
pub enum Error {
    /// A point is missing a dimension that is required by someone else, usually a `Sink`.
    MissingDimension(String),
    /// A wrapper around a las error.
    Las(las::Error),
    #[cfg(feature = "rxp")]
    /// A wrapper around an rxp error.
    Rxp(rivlib::Error),
    /// A wrapper around an sdc error.
    Sdc(sdc::Error),
    #[cfg(feature = "sdf-convert")]
    /// A wrapper around an sdf error.
    Sdf(sdf::Error),
}

impl From<las::Error> for Error {
    fn from(err: las::Error) -> Error {
        Error::Las(err)
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
