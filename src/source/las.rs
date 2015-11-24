//! Source las points.

use std::io::{Read, Seek};

use las;

use Result;
use point::{Intensity, Point, ScanDirection};
use source::Source;

impl<R: Read + Seek> Source for las::Stream<R> {
    fn source(&mut self, want: usize) -> Result<Option<Vec<Point>>> {
        let mut points = Vec::with_capacity(want);
        for _ in 0..want {
            match try!(self.next_point()) {
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
