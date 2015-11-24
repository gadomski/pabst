//! Source las points.

use std::io::{Read, Seek};

use las;

use Result;
use error::Error;
use point::{Intensity, Point, ScanDirection};
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
    fn intensity(&self) -> Intensity {
        Intensity::from_u16(self.intensity)
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
    fn scan_direction(&self) -> ScanDirection {
        match self.scan_direction {
            las::point::ScanDirection::Forward => ScanDirection::Forward,
            las::point::ScanDirection::Backward => ScanDirection::Backward,
        }
    }
    fn edge_of_flight_line(&self) -> bool {
        self.edge_of_flight_line
    }
    fn classification(&self) -> u8 {
        self.classification.as_u8()
    }
    fn synthetic(&self) -> bool {
        self.synthetic
    }
    fn key_point(&self) -> bool {
        self.key_point
    }
    fn withheld(&self) -> bool {
        self.withheld
    }
    fn scan_angle(&self) -> Option<f64> {
        Some(self.scan_angle_rank as f64)
    }
    fn point_source_id(&self) -> Option<u16> {
        Some(self.point_source_id)
    }
    fn user_data(&self) -> Option<u8> {
        Some(self.user_data)
    }
}
