//! Implement source for .sdc, Riegl's simple file-based discrete return format.

use std::io::Read;

use sdc;

use Result;
use error::Error;
use point::Point;
use source::Source;

impl<R: Read> Source for sdc::Reader<R> {
    type Point = sdc::Point;
    fn source(&mut self, want: usize) -> Result<Option<Vec<Self::Point>>> {
        let mut points = Vec::with_capacity(want);
        for _ in 0..want {
            match try!(self.next_point()) {
                Some(point) => points.push(point),
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
}

impl From<sdc::Error> for Error {
    fn from(err: sdc::Error) -> Error {
        Error::Upstream(Box::new(err))
    }
}

impl Point for sdc::Point {
    fn x(&self) -> f64 {
        self.x as f64
    }
    fn y(&self) -> f64 {
        self.y as f64
    }
    fn z(&self) -> f64 {
        self.z as f64
    }
    fn intensity(&self) -> u16 {
        self.amplitude
    }
    fn return_number(&self) -> Option<usize> {
        Some(self.target as usize)
    }
    fn number_of_returns(&self) -> Option<usize> {
        Some(self.num_target as usize)
    }
    fn gps_time(&self) -> Option<f64> {
        Some(self.time)
    }
}

#[cfg(test)]
mod tests {

    use sdc;

    use source::Source;

    #[test]
    fn simple_read_format_5_0() {
        let mut source = sdc::Reader::from_path("data/4-points.sdc").unwrap();
        let points = source.source_to_end(100).unwrap();
        assert_eq!(4, points.len());
    }
}
