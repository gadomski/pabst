//! Source las points.

use std::io::{Read, Seek};
use std::path::Path;

use las;
use toml;

use Result;
use error::Error;
use point::{Intensity, Point, ScanDirection};
use source::{FileSource, Source};

impl<R: Read + Seek> Source for las::Reader<R> {
    fn source(&mut self, want: usize) -> Result<Option<Vec<Point>>> {
        let mut points = Vec::with_capacity(want);
        for _ in 0..want {
            match try!(self.read_point()) {
                Some(point) => points.push(Point::from(point)),
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

    fn source_len(&mut self) -> Option<usize> {
        Some(self.npoints() as usize)
    }
}

impl<R: Read + Seek> FileSource for las::Reader<R> {
    fn open_file_source<P: AsRef<Path>>(path: P,
                                        options: Option<&toml::Table>)
                                        -> Result<Box<Source>> {
        if options.is_some() {
            return Err(Error::InvalidOption("las source does not support any options at this \
                                             time"
                                                .to_string()));
        }
        let source = try!(las::Reader::from_path(path));
        Ok(Box::new(source))
    }
}

impl From<las::Point> for Point {
    fn from(point: las::Point) -> Point {
        Point {
            x: point.x,
            y: point.y,
            z: point.z,
            intensity: Intensity::from_u16(point.intensity),
            return_number: Some(point.return_number.as_u8() as usize),
            number_of_returns: Some(point.number_of_returns.as_u8() as usize),
            gps_time: point.gps_time,
            scan_direction: match point.scan_direction {
                las::point::ScanDirection::Forward => ScanDirection::Forward,
                las::point::ScanDirection::Backward => ScanDirection::Backward,
            },
            edge_of_flight_line: point.edge_of_flight_line,
            classification: point.classification.as_u8(),
            synthetic: point.synthetic,
            key_point: point.key_point,
            withheld: point.withheld,
            scan_angle: Some(point.scan_angle_rank as f64),
            point_source_id: Some(point.point_source_id),
            user_data: Some(point.user_data),
            ..Default::default()
        }
    }
}
