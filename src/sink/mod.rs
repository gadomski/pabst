//! Point sinks.

pub mod las;
pub mod sdc;

use std::ffi::OsStr;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use las::Writer as LasWriter;
use toml;

use Result;
use error::Error;
use point::Point;

/// Opens a file sink with the given options
///
/// # Examples
///
/// ```
/// use pabst::sink::open_file_sink;
/// let sink = open_file_sink("temp.las", None).unwrap();
/// ```
pub fn open_file_sink<P>(path: P, options: Option<&toml::Table>) -> Result<Box<Sink>>
where P: 'static + AsRef<Path> + AsRef<OsStr>
{
    match Path::new(&path).extension().and_then(|e| e.to_str()) {
        Some("las") => LasWriter::<BufWriter<File>>::open_file_sink(path, options),
        _ => Err(Error::UndefinedSink),
    }
}

/// A point sink.
///
/// A sink is a place where points go. Mabye they're written to disk. Maybe not.
pub trait Sink {
    /// Sink a single point into this sink.
    ///
    /// TODO make point a reference
    fn sink(&mut self, point: Point) -> Result<()>;

    /// Close a sink, probably writing its points out or something.
    fn close_sink(self: Box<Self>) -> Result<()>;
}

/// A sink that puts points into a path.
pub trait FileSink {
    /// Open a new file sink.
    fn open_file_sink<P: AsRef<Path>>(path: P, options: Option<&toml::Table>) -> Result<Box<Sink>> where Self: Sized;
}
