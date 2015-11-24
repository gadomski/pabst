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
            ..Default::default()
        }
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
