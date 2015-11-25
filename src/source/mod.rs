//! Sources of points.
//!
//! These don't necessarily have to be file format readers, but they usually are.

pub mod las;
pub mod sdc;
#[cfg(feature = "sdf-source")]
pub mod sdf;
#[cfg(feature = "rxp-source")]
pub mod rxp;

use std::ffi::OsStr;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use las::Stream as LasStream;
#[cfg(feature = "rxp-source")]
use rivlib::Stream as RxpStream;
use toml;

use Result;
use error::Error;
use point::Point;

/// Opens a file source with the given options.
///
/// # Examples
///
/// ```
/// use pabst::source::open_file_source;
/// let source = open_file_source("data/1.0_0.las", None).unwrap();
/// ```
pub fn open_file_source<P>(path: P, options: Option<&toml::Table>) -> Result<Box<Source>>
    where P: AsRef<Path> + AsRef<OsStr>
{
    match Path::new(&path).extension().and_then(|e| e.to_str()) {
        Some("las") => LasStream::<BufReader<File>>::open_file_source(path, options),
        #[cfg(feature = "rxp-source")]
        Some("rxp") => RxpStream::open_file_source(path, options),
        _ => Err(Error::UndefinedSource),
    }
}

/// A point source.
pub trait Source {
    /// Sources some points from the `Source`.
    ///
    /// Use `want` to request a certain number of points, but the source is not compelled to return
    /// exactly that number. This method returns an optional vector of points, or `None` if the
    /// source is out of points.
    fn source(&mut self, want: usize) -> Result<Option<Vec<Point>>>;

    /// Sources all points in this `Source`.
    ///
    /// If the source is an infinite stream, it should re-define this method to return an error.
    fn source_to_end(&mut self, want: usize) -> Result<Vec<Point>> {
        let mut points = Vec::new();
        loop {
            match try!(self.source(want)) {
                Some(ref mut p) => points.append(p),
                None => return Ok(points),
            }
        }
    }
}

/// A point source that can be opened from a path.
pub trait FileSource {
    /// Open this file source for the given path with the given options.
    fn open_file_source<P>(path: P, options: Option<&toml::Table>) -> Result<Box<Source>>
        where Self: Sized,
              P: AsRef<Path> + AsRef<OsStr>;
}
