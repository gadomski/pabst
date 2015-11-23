//! Source las points.

use std::io::{Read, Seek};

use las;

use Result;
use error::Error;
use point::Point;
use source::Source;

impl<R: Read + Seek> Source for las::Stream<R> {
    type Point = las::Point;

    fn source(&mut self, want: usize) -> Result<Option<Vec<las::Point>>> {
        let mut points = Vec::with_capacity(want);
        for _ in 0..want {
            match try!(self.next_point()) {
                Some(point) => points.push(point),
                None => {
                    if points.is_empty() {
                        return Ok(None);
                    } else {
                        return Ok(Some(points));
                    }
                }
            }
        }
        Ok(Some(points))
    }
}

impl From<las::Error> for Error {
    fn from(err: las::Error) -> Error {
        Error::Upstream(Box::new(err))
    }
}

impl Point for las::Point {
    fn x(&self) -> f64 {
        self.x
    }
    fn y(&self) -> f64 {
        self.y
    }
    fn z(&self) -> f64 {
        self.z
    }
    fn intensity(&self) -> u16 {
        self.intensity
    }
    fn return_number(&self) -> Option<usize> {
        Some(self.return_number.as_u8() as usize)
    }
    fn number_of_returns(&self) -> Option<usize> {
        Some(self.number_of_returns.as_u8() as usize)
    }
    fn gps_time(&self) -> Option<f64> {
        self.gps_time
    }

    fn set_x(&mut self, x: f64) {
        self.x = x;
    }
    fn set_y(&mut self, y: f64) {
        self.y = y;
    }
    fn set_z(&mut self, z: f64) {
        self.z = z;
    }
}
