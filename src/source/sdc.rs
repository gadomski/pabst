//! Implement source for .sdc, Riegl's simple file-based discrete return format.

use std::io::Read;

use sdc;

use Result;
use point::{Intensity, Point};
use source::Source;

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
            ..Default::default()
        }
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
