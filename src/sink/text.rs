//! A text writer.

use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use {Error, Result};
use point::Point;
use sink::{FileSink, Sink};

/// A very dumb text writer.
#[derive(Debug)]
pub struct Writer<W: Write> {
    dimensions: Vec<String>,
    writer: W,
}

impl Writer<BufWriter<File>> {
    /// Opens a writer for a path.
    ///
    /// # Examples
    ///
    /// ```
    /// use pabst::sink::text::Writer;
    /// let writer = Writer::from_path("/dev/null", vec!["x".to_string(), "y".to_string(), "z".to_string()]).unwrap();
    /// ```
    pub fn from_path<P: AsRef<Path>>(path: P,
                                     dimensions: Vec<String>)
                                     -> Result<Writer<BufWriter<File>>> {
        let mut writer = BufWriter::new(try!(File::create(path)));
        for (i, dimension) in dimensions.iter().enumerate() {
            try!(write!(writer, "{}", dimension));
            if i < dimensions.len() - 1 {
                try!(write!(writer, " "));
            }
        }
        try!(write!(writer, "\n"));
        Ok(Writer {
            dimensions: dimensions,
            writer: writer,
        })
    }
}

impl<W: Write> Sink for Writer<W> {
    fn sink(&mut self, point: &Point) -> Result<()> {
        for (i, name) in self.dimensions.iter().enumerate() {
            match name.as_ref() {
                "x" => try!(write!(self.writer, "{}", point.x)),
                "y" => try!(write!(self.writer, "{}", point.y)),
                "z" => try!(write!(self.writer, "{}", point.z)),
                "intensity" => try!(write!(self.writer, "{}", point.intensity.as_u16())),
                "range" => {
                    if let Some(range) = point.range {
                        try!(write!(self.writer, "{}", range));
                    } else {
                        return Err(Error::MissingDimension("Point does not have range".to_string()));
                    }
                }
                "scan_angle" => {
                    if let Some(scan_angle) = point.scan_angle {
                        try!(write!(self.writer, "{}", scan_angle));
                    } else {
                        return Err(Error::MissingDimension("Point does not have scan angle".to_string()));
                    }
                }
                "gps_time" => {
                    if let Some(time) = point.gps_time {
                        try!(write!(self.writer, "{}", time));
                    } else {
                        return Err(Error::MissingDimension("Point does not have gps time".to_string()));
                    }
                }
                _ => {
                    return Err(Error::MissingDimension(format!("Text writer doesn't know how to \
                                                                write dimension '{}'",
                                                               name)))
                }
            };
            if i < self.dimensions.len() - 1 {
                try!(write!(self.writer, " "));
            }
        }
        try!(write!(self.writer, "\n"));
        Ok(())
    }

    fn close_sink(self: Box<Self>) -> Result<()> {
        Ok(())
    }
}

impl<W: Write> FileSink for Writer<W> {
    type Config = TextConfig;
    fn open_file_sink<P>(path: P, config: TextConfig) -> Result<Box<Sink>>
        where P: AsRef<Path>
    {
        Ok(Box::new(try!(Writer::from_path(path, config.dimensions))))
    }
}

/// A simple configuration object.
#[derive(Debug, RustcDecodable)]
pub struct TextConfig {
    dimensions: Vec<String>,
}

impl Default for TextConfig {
    fn default() -> TextConfig {
        TextConfig { dimensions: vec!["x".to_string(), "y".to_string(), "z".to_string()] }
    }
}

#[cfg(test)]
mod tests {
    use sink::open_file_sink;
    use source::open_file_source;

    #[test]
    fn open_sink() {
        let mut source = open_file_source("data/1.0_0.las", None).unwrap();
        let mut sink = open_file_sink("target/debug/open_sink.txt", None).unwrap();
        for ref point in source.source_to_end(1000).unwrap() {
            sink.sink(point).unwrap();
        }
        sink.close_sink().unwrap();
    }
}
