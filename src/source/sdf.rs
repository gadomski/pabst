//! Read points from .sdf full waveform files.
//!
//! Our sdf library contains some logic to do discretization of waveforms, so we just use that
//! rather than processing the waveforms ourselves.

use std::ffi::OsStr;
use std::path::Path;
use std::u32;

use sdf;

use Result;
use error::Error;
use point::{Intensity, Point};
use source::{FileSource, Source};

impl Source for sdf::File {
    fn source(&mut self, want: usize) -> Result<Option<Vec<Point>>> {
        let mut points = Vec::with_capacity(want);
        let ref file_info = try!(self.info());
        try!(self.reindex());
        loop {
            match self.read() {
                Ok(ref record) => {
                    points.append(&mut try!(sdf::convert::discretize(record, file_info))
                                           .into_iter()
                                           .map(|p| Point::from(p))
                                           .collect());
                    if points.len() > want {
                        break;
                    }
                }
                Err(sdf::Error::EndOfFile(_)) => {
                    break;
                }
                Err(err) => return Err(Error::from(err)),
            }
        }
        if points.is_empty() {
            Ok(None)
        } else {
            Ok(Some(points))
        }
    }

    fn source_len(&mut self) -> Option<usize> {
        if self.reindex().is_err() {
            return None;
        }
        if self.seek(0).is_err() {
            return None;
        }
        let start = match self.tell() {
            Ok(n) => n,
            Err(_) => return None,
        };
        if self.seek(u32::MAX).is_err() {
            return None;
        }
        let end = match self.tell() {
            Ok(n) => n,
            Err(_) => return None,
        };
        Some((end - start) as usize)
    }
}

impl From<sdf::convert::Point> for Point {
    fn from(point: sdf::convert::Point) -> Point {
        Point {
            x: point.x as f64,
            y: point.y as f64,
            z: point.z as f64,
            intensity: Intensity::from_u16(point.peak.amplitude),
            return_number: Some(point.target as usize),
            number_of_returns: Some(point.num_target as usize),
            facet_number: Some(point.facet as u8),
            high_channel: Some(point.high_channel),
            // TODO this probably isn't always gps time
            gps_time: Some(point.time),
            ..Default::default()
        }
    }
}

impl FileSource for sdf::File {
    type Config = SdfConfig;
    fn open_file_source<P>(path: P, _: Self::Config) -> Result<Box<Source>>
        where P: AsRef<Path> + AsRef<OsStr>
    {
        let path = OsStr::new(&path);
        Ok(Box::new(try!(sdf::File::open(path.to_str().unwrap()))))
    }
}

/// Our decodable configuration.
#[derive(Clone, Copy, Debug, Default, RustcDecodable)]
pub struct SdfConfig;

#[cfg(test)]
mod tests {

    use sdf;

    use source::{Source, open_file_source};

    #[test]
    fn source_them_all() {
        let mut source = sdf::File::open("data/110630_174316.sdf").unwrap();
        let points = source.source(1000).unwrap().unwrap();
        assert!(points.len() >= 1000);
        source.remove_index().unwrap();
    }

    #[test]
    fn open_file_source_test() {
        let _ = open_file_source("data/110630_174316.sdf", None).unwrap();
    }
}
