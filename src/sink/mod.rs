//! Point sinks.

pub mod las;
pub mod sdc;
pub mod text;

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
    Text,
}

impl SinkType {
    fn from_osstr_ref<S: AsRef<OsStr>>(s: S) -> Result<SinkType> {
        match Path::new(&s).extension().and_then(|e| e.to_str()) {
            Some("las") => Ok(SinkType::Las),
            Some("txt") => Ok(SinkType::Text),
            Some(_) | None => Err(Error::UnregisteredFileExtension(OsStr::new(&s).to_os_string())),
        }
    }
}

macro_rules! decode_or_default {
    ($klass:path, $decoder:expr) => {{
        match $decoder {
            Some(ref mut decoder) => try!(<$klass as FileSink>::Config::decode(decoder)),
            None => Default::default()
        }
    }}
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
    let mut decoder = config.map(|c| toml::Decoder::new(c));
    match try!(SinkType::from_osstr_ref(&path)) {
        SinkType::Las =>  LasWriter::<BufWriter<File>>::open_file_sink(path, decode_or_default!(LasWriter<BufWriter<File>>, decoder)),
        SinkType::Text =>  text::Writer::<BufWriter<File>>::open_file_sink(path, decode_or_default!(text::Writer<BufWriter<File>>, decoder)),
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

impl Sink for Box<Sink> {
    fn sink(&mut self, point: &Point) -> Result<()> {
        (**self).sink(point)
    }
    fn close_sink(self: Box<Self>) -> Result<()> {
        (*self).close_sink()
    }
}

/// A sink that puts points into a path.
pub trait FileSink {
    /// Decodable configuration.
    type Config: Decodable;

    /// Open a new file sink.
    fn open_file_sink<P: AsRef<Path>>(path: P, options: Self::Config) -> Result<Box<Sink>> where Self: Sized;
}
