//! Read points from .sdf full waveform files.
//!
//! Our sdf library contains some logic to do discretization of waveforms, so we just use that
//! rather than processing the waveforms ourselves.
//!
//! Because of the way sourcing works, we have to reindex the file every time source is called.
//! Therefore, try to keep your wants big.

use sdf;

use Result;
use error::Error;
use point::{Intensity, Point};
use source::Source;

impl Source for sdf::File {
    type Point = sdf::convert::Point;
    fn source(&mut self, want: usize) -> Result<Option<Vec<Self::Point>>> {
        let mut points = Vec::with_capacity(want);
        let ref file_info = try!(self.info());
        try!(self.reindex());
        loop {
            match self.read() {
                Ok(ref record) => {
                    points.append(&mut try!(sdf::convert::discretize(record, file_info)));
                    if points.len() > want {
                        break;
                    }
                }
                Err(sdf::Error::EndOfFile(_)) => {
                    break;
                },
                Err(err) => return Err(Error::from(err)),
            }
        }
        if points.is_empty() {
            Ok(None)
        } else {
            Ok(Some(points))
        }
    }
}

impl Point for sdf::convert::Point {
    fn x(&self) -> f64 { self.x as f64 }
    fn y(&self) -> f64 { self.y as f64 }
    fn z(&self) -> f64 { self.z as f64 }
    fn intensity(&self) -> Intensity {
        Intensity::from_u16(self.peak.amplitude)
    }
    fn return_number(&self) -> Option<usize> {
        Some(self.target as usize)
    }
    fn number_of_returns(&self) -> Option<usize> {
        Some(self.num_target as usize)
    }
    fn facet_number(&self) -> Option<u8> {
        Some(self.facet as u8)
    }
    fn high_channel(&self) -> Option<bool> {
        Some(self.high_channel)
    }
}

impl From<sdf::Error> for Error {
    fn from(err: sdf::Error) -> Error {
        Error::Upstream(Box::new(err))
    }
}

#[cfg(test)]
mod tests {

    use sdf;

    use source::Source;

    #[test]
    fn source_them_all() {
        let mut source = sdf::File::open("data/110630_174316.sdf").unwrap();
        let points = source.source(1000).unwrap().unwrap();
        assert!(points.len() >= 1000);
        source.remove_index().unwrap();
    }
}
