//! Sources of points.
//!
//! These don't necessarily have to be file format readers, but they usually are.

pub mod las;
pub mod sdc;
#[cfg(feature = "rxp-source")]
pub mod rxp;

use std::ffi::OsStr;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use las::Reader as LasReader;
#[cfg(feature = "rxp-source")]
use rivlib::Stream as RxpStream;
use rustc_serialize::Decodable;
use sdc::Reader as SdcReader;
use toml;

use Result;
use error::Error;
use point::Point;

enum SourceType {
    Las,
    #[cfg(feature = "rxp-source")]
    Rxp,
    Sdc,
}

impl SourceType {
    fn from_osstr_ref<S: AsRef<OsStr>>(s: S) -> Result<SourceType> {
        match Path::new(&s).extension().and_then(|e| e.to_str()) {
            Some("las") => Ok(SourceType::Las),
            #[cfg(feature = "rxp-source")]
            Some("rxp") => Ok(SourceType::Rxp),
            Some("sdc") => Ok(SourceType::Sdc),
            Some(_) | None => Err(Error::UnregisteredFileExtension(OsStr::new(&s).to_os_string())),
        }
    }
}

macro_rules! decode_or_default {
    ($klass:path, $decoder:expr) => {{
        match $decoder {
            Some(ref mut decoder) => try!(<$klass as FileSource>::Config::decode(decoder)),
            None => Default::default()
        }
    }}
}

/// Opens a file source with the given options.
///
/// # Examples
///
/// ```
/// use pabst::source::open_file_source;
/// let source = open_file_source("data/1.0_0.las", None).unwrap();
/// ```
pub fn open_file_source<P>(path: P, config: Option<toml::Value>) -> Result<Box<Source>> where P: AsRef<Path> + AsRef<OsStr>
{
    let mut decoder = config.map(|c| toml::Decoder::new(c));
    match try!(SourceType::from_osstr_ref(&path)) {
        SourceType::Las => LasReader::<BufReader<File>>::open_file_source(path, decode_or_default!(LasReader<BufReader<File>>, decoder)),
        #[cfg(feature = "rxp-source")]
        SourceType::Rxp => RxpStream::open_file_source(path, decode_or_default!(RxpStream, decoder)),
        SourceType::Sdc => SdcReader::<BufReader<File>>::open_file_source(path, decode_or_default!(SdcReader<BufReader<File>>, decoder)),
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

    /// Returns a guess at total number of points in this source.
    ///
    /// If possible, sources should prefer to report point totals from headers, etc, rather than
    /// reading all of the points. For this reason, users of this method should be aware that it
    /// might not be exactly correct.
    ///
    /// Sources that cannot know their point count should return `None`.
    fn source_len(&mut self) -> Option<usize>;
}

impl Source for Box<Source> {
    fn source(&mut self, want: usize) -> Result<Option<Vec<Point>>> {
        (**self).source(want)
    }

    fn source_len(&mut self) -> Option<usize> {
        (**self).source_len()
    }
}

/// A point source that can be opened from a path.
pub trait FileSource {
    /// Decodable configuration object.
    type Config: Decodable;

    /// Opens a file source with the given config.
    fn open_file_source<P>(path: P, config: Self::Config) -> Result<Box<Source>> where P: AsRef<Path> + AsRef<OsStr>;
}
