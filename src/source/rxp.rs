//! Get points from an .rxp file or an rxp stream.
//!
//! `rxp` is a data layout format from Riegl.

use std::ffi::OsStr;
use std::path::Path;

use rivlib;

use error::Error;
use point::{Intensity, Point};
use Result;
use source::{FileSource, Source};

impl Source for rivlib::Stream {
    fn source(&mut self, want: usize) -> Result<Option<Vec<Point>>> {
        self.read(want as u32)
            .map_err(|e| Error::from(e))
            .map(|o| o.map(|v| v.into_iter().map(|p| Point::from(p)).collect()))
    }

    fn source_len(&mut self) -> Option<usize> {
        None
    }
}

impl From<rivlib::Point> for Point {
    fn from(point: rivlib::Point) -> Point {
        Point {
            x: point.x as f64,
            y: point.y as f64,
            z: point.z as f64,
            intensity: Intensity::new(point.reflectance as f64, -50.0, 50.0),
            return_number: match point.echo_type {
                rivlib::stream::EchoType::Single | rivlib::stream::EchoType::First => Some(1),
                _ => None,
            },
            number_of_returns: match point.echo_type {
                rivlib::stream::EchoType::Single => Some(1),
                _ => None,
            },
            gps_time: if point.pps {
                Some(point.time as f64 * 1e-9)
            } else {
                None
            },
            ..Default::default()
        }
    }
}

/// Rxp's decodable configuration object.
#[derive(Copy, Clone, Debug, RustcDecodable)]
pub struct RxpConfig {
    sync_to_pps: Option<bool>,
}

impl FileSource for rivlib::Stream {
    type Config = RxpConfig;

    fn open_file_source<P>(path: P, config: Option<Self::Config>) -> Result<Box<Source>>
        where P: AsRef<Path> + AsRef<OsStr>
    {
        let path = OsStr::new(&path).to_str().unwrap();
        let sync_to_pps = config.map(|c| c.sync_to_pps.unwrap_or(true)).unwrap_or(true);
        Ok(Box::new(try!(rivlib::Stream::open(path, sync_to_pps))))
    }
}

#[cfg(test)]
mod tests {
    use rivlib;
    use toml;

    use source::{open_file_source, Source};

    fn xyz_from_first_point<S: Source>(source: &mut S) {
        let points = source.source(1).unwrap().unwrap();
        let ref point = points[0];
        assert_eq!(2.2630674839019775, point.x);
        assert_eq!(-0.03840770199894905, point.y);
        assert_eq!(-1.3249952793121338, point.z);
    }

    #[test]
    fn source_all() {
        let mut source = rivlib::Stream::open("data/130501_232206_cut.rxp", true).unwrap();
        let points = source.source_to_end(100).unwrap();
        assert_eq!(177208, points.len());
    }

    #[test]
    fn xyz() {
        let ref mut source = rivlib::Stream::open("data/130501_232206_cut.rxp", true).unwrap();
        xyz_from_first_point(source);
    }

    #[test]
    fn file_source() {
        let config = toml::Parser::new(r#"
        sync_to_pps = true
        "#).parse().unwrap();
        let mut source = open_file_source("data/130501_232206_cut.rxp", Some(toml::Value::Table(config))).unwrap();
        let points = source.source_to_end(200000).unwrap();
        assert_eq!(177208, points.len());
    }
}
