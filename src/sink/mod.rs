//! Point sinks.

pub mod las;
pub mod sdc;

use std::ffi::OsStr;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use las::Writer as LasWriter;
use rustc_serialize::Decodable;
use toml;

use Result;
use error::Error;
use point::Point;

enum SinkType {
    Las,
}

impl SinkType {
    fn from_osstr_ref<S: AsRef<OsStr>>(s: S) -> Result<SinkType> {
        match Path::new(&s).extension().and_then(|e| e.to_str()) {
            Some("las") => Ok(SinkType::Las),
            Some(_) | None => Err(Error::UnregisteredFileExtension(OsStr::new(&s).to_os_string())),
        }
    }
}

/// Opens a file sink with the given options
///
/// # Examples
///
/// ```
/// # use std::fs::remove_file;
/// use pabst::sink::open_file_sink;
/// let sink = open_file_sink("temp.las", None).unwrap();
/// # remove_file("temp.las").unwrap();
/// ```
pub fn open_file_sink<P>(path: P, config: Option<toml::Value>) -> Result<Box<Sink>>
where P: AsRef<Path> + AsRef<OsStr>
{
    let decoder = config.map(|c| toml::Decoder::new(c));
    match try!(SinkType::from_osstr_ref(&path)) {
        SinkType::Las =>  {
            let config = match decoder {
                Some(mut decoder) => Some(try!(<LasWriter<BufWriter<File>> as FileSink>::Config::decode(&mut decoder))),
                None => None,
            };
            LasWriter::<BufWriter<File>>::open_file_sink(path, config)
        }
    }
}

/// A point sink.
///
/// A sink is a place where points go. Mabye they're written to disk. Maybe not.
pub trait Sink {
    /// Sink a single point into this sink.
    fn sink(&mut self, point: &Point) -> Result<()>;

    /// Close a sink, probably writing its points out or something.
    fn close_sink(self: Box<Self>) -> Result<()>;
}

/// A sink that puts points into a path.
pub trait FileSink {
    /// Decodable configuration.
    type Config: Decodable;

    /// Open a new file sink.
    fn open_file_sink<P: AsRef<Path>>(path: P, options: Option<Self::Config>) -> Result<Box<Sink>> where Self: Sized;
}
