//! Implement source for .sdc, Riegl's simple file-based discrete return format.

use std::ffi::OsStr;
use std::io::Read;
use std::path::Path;

use sdc;

use Result;
use point::{Intensity, Point};
use source::{FileSource, Source};

impl<R: Read> Source for sdc::Reader<R> {
    fn source(&mut self, want: usize) -> Result<Option<Vec<Point>>> {
        let mut points = Vec::with_capacity(want);
        for _ in 0..want {
            match try!(self.next_point()) {
                Some(point) => points.push(Point::from(point)),
                None => {
                    if points.is_empty() {
                        return Ok(None);
                    } else {
                        break;
                    }
                }
            }
        }
        Ok(Some(points))
    }

    fn source_len(&mut self) -> Option<usize> {
        None
    }
}

impl From<sdc::Point> for Point {
    fn from(point: sdc::Point) -> Point {
        Point {
            x: point.x as f64,
            y: point.y as f64,
            z: point.z as f64,
            intensity: Intensity::from_u16(point.amplitude),
            return_number: Some(point.target as usize),
            number_of_returns: Some(point.num_target as usize),
            gps_time: Some(point.time),
            range: Some(point.range as f64),
            scan_angle: Some(point.theta as f64),
            rg_index: Some(point.rg_index as f64),
            facet_number: Some(point.facet_number),
            high_channel: Some(point.high_channel),
            ..Default::default()
        }
    }
}

impl<R: Read> FileSource for sdc::Reader<R> {
    type Config = SdcConfig;
    fn open_file_source<P>(path: P, _: Self::Config) -> Result<Box<Source>> where P: AsRef<Path> + AsRef<OsStr> {
        Ok(Box::new(try!(sdc::Reader::from_path(path))))
    }
}

/// Configuration structure for an sdc reader.
#[derive(Clone, Copy, Debug, Default, RustcDecodable)]
pub struct SdcConfig;

#[cfg(test)]
mod tests {

    use sdc;

    use source::{open_file_source, Source};

    #[test]
    fn simple_read_format_5_0() {
        let mut source = sdc::Reader::from_path("data/4-points.sdc").unwrap();
        let points = source.source_to_end(100).unwrap();
        assert_eq!(4, points.len());
    }

    #[test]
    fn fs() {
        let mut source = open_file_source("data/4-points.sdc", None).unwrap();
        let points = source.source_to_end(100).unwrap();
        assert_eq!(4, points.len());
    }
}
