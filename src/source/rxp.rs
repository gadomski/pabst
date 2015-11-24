//! Get points from an .rxp file or an rxp stream.
//!
//! `rxp` is a data layout format from Riegl.

use rivlib;

use error::Error;
use point::{Intensity, Point};
use Result;
use source::Source;

impl Source for rivlib::Stream {
    type Point = rivlib::Point;
    fn source(&mut self, want: usize) -> Result<Option<Vec<rivlib::Point>>> {
        self.read(want as u32).map_err(|e| Error::from(e))
    }
}

impl Point for rivlib::Point {
    fn x(&self) -> f64 {
        self.x as f64
    }
    fn y(&self) -> f64 {
        self.y as f64
    }
    fn z(&self) -> f64 {
        self.z as f64
    }
    fn intensity(&self) -> Intensity {
        Intensity::new(self.reflectance as f64, -50.0, 50.0)
    }
    fn return_number(&self) -> Option<usize> {
        match self.echo_type {
            rivlib::stream::EchoType::Single | rivlib::stream::EchoType::First => Some(1),
            _ => None,
        }
    }
    fn number_of_returns(&self) -> Option<usize> {
        match self.echo_type {
            rivlib::stream::EchoType::Single => Some(1),
            _ => None,
        }
    }
    fn gps_time(&self) -> Option<f64> {
        if self.pps {
            Some(self.time as f64 * 1e-9)
        } else {
            None
        }
    }
}

impl From<rivlib::Error> for Error {
    fn from(err: rivlib::Error) -> Error {
        Error::Upstream(Box::new(err))
    }
}

#[cfg(test)]
mod tests {
    use rivlib;

    use point::Point;
    use source::Source;

    fn xyz_from_first_point<P: Point, S: Source<Point = P>>(source: &mut S) {
        let points = source.source(1).unwrap().unwrap();
        let ref point = points[0];
        assert_eq!(2.2630674839019775, point.x());
        assert_eq!(-0.03840770199894905, point.y());
        assert_eq!(-1.3249952793121338, point.z());
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
}
