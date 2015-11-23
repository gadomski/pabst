//! Sink points into a las file.

use las;

use point::{Point, ScanDirection};
use Result;
use sink::Sink;

impl Sink for las::File {
    fn sink<P: Point>(&mut self, point: &P) -> Result<()> {
        self.add_point(las::Point {
            x: point.x(),
            y: point.y(),
            z: point.z(),
            intensity: point.intensity() as u16,
            return_number: try!(las::point::ReturnNumber::from_u8(point.return_number().unwrap_or(0) as u8)),
            number_of_returns: try!(las::point::NumberOfReturns::from_u8(point.number_of_returns().unwrap_or(0) as u8)),
            scan_direction: match point.scan_direction() {
                ScanDirection::Forward | ScanDirection::Unknown => las::point::ScanDirection::Forward,
                ScanDirection::Backward => las::point::ScanDirection::Backward
            },
            edge_of_flight_line: point.edge_of_flight_line(),
            classification: try!(las::point::Classification::from_u8(point.classification())),
            synthetic: point.synthetic(),
            key_point: point.key_point(),
            withheld: point.withheld(),
            scan_angle_rank: point.scan_angle() as i8,
            user_data: point.user_data(),
            point_source_id: point.point_source_id(),
            gps_time: point.gps_time(),
        // FIXME these should be properties too
            red: None,
            green: None,
            blue: None,
            extra_bytes: None,
        });
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::fs::remove_file;

    use las;

    use sink::Sink;
    use source::Source;

    #[test]
    fn read_write_las() {
        let mut source = las::Stream::from_path("data/1.0_0.las").unwrap();
        let mut sink = las::File::new();
        for ref point in source.source_to_end(100).unwrap() {
            sink.sink(point).unwrap()
        }
        sink.to_path("temp.las").unwrap();

        let mut source = las::Stream::from_path("temp.las").unwrap();
        let points = source.source_to_end(100).unwrap();
        assert_eq!(1, points.len());
        remove_file("temp.las").unwrap();
    }
}
