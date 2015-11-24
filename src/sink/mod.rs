//! Point sinks.

pub mod las;
pub mod sdc;

use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::Path;

use las::Writer as LasWriter;

use Result;
use error::Error;
use point::Point;

/// Opens a file sink with the given options
///
/// # Examples
///
/// ```
/// use std::collections::HashMap;
/// use pabst::sink::open_file_sink;
/// let sink = open_file_sink("temp.las", HashMap::new()).unwrap();
/// ```
pub fn open_file_sink<P>(path: P, options: HashMap<String, String>) -> Result<Box<FileSink<P>>> where P: 'static + AsRef<Path> + AsRef<OsStr> {
    match Path::new(&path).extension().and_then(|e| e.to_str()) {
        Some("las") => LasWriter::open_file_sink(path, options),
        _ => Err(Error::UndefinedSink),
    }
}

/// A point sink.
///
/// A sink is a place where points go. Mabye they're written to disk. Maybe not.
pub trait Sink {
    /// Sink a single point into this sink.
    fn sink(&mut self, point: Point) -> Result<()>;
}

/// A sink that puts points into a path.
pub trait FileSink<P: AsRef<Path>>: Sink {
    /// Open a new file sink.
    fn open_file_sink(path: P, options: HashMap<String, String>) -> Result<Box<FileSink<P>>> where Self: Sized;

    /// Returns this `FileSink` as a mutable reference to a Sink.
    fn as_mut_sink(&mut self) -> &mut Sink where Self: Sized {
        self
    }

    /// Close the file sink.
    ///
    /// This usually is where you do any finalization, like writing points out to disk.
    fn close_file_sink(&mut self) -> Result<()>;
}
