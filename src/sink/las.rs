//! Sink points into a las file.

use std::collections::HashMap;
use std::path::Path;

use las;

use Result;
use error::Error;
use point::{Point, ScanDirection};
use sink::{FileSink, Sink};

impl<P: AsRef<Path>> Sink for las::Writer<P> {
    fn sink(&mut self, point: Point) -> Result<()> {
        self.write_point(try!(from_point(point)));
        Ok(())
    }
}

fn from_point(point: Point) -> Result<las::Point> {
    Ok(las::Point {
        x: point.x,
        y: point.y,
        z: point.z,
        intensity: point.intensity.as_u16(),
        return_number: try!(las::point::ReturnNumber::from_u8(point.return_number.unwrap_or(0) as u8)),
        number_of_returns: try!(las::point::NumberOfReturns::from_u8(point.number_of_returns.unwrap_or(0) as u8)),
        scan_direction: match point.scan_direction {
            ScanDirection::Forward | ScanDirection::Unknown => las::point::ScanDirection::Forward,
            ScanDirection::Backward => las::point::ScanDirection::Backward
        },
        edge_of_flight_line: point.edge_of_flight_line,
        classification: try!(las::point::Classification::from_u8(point.classification)),
        synthetic: point.synthetic,
        key_point: point.key_point,
        withheld: point.withheld,
        scan_angle_rank: point.scan_angle.unwrap_or(0.0) as i8,
        user_data: point.user_data.unwrap_or(0),
        point_source_id: point.point_source_id.unwrap_or(0),
        gps_time: point.gps_time,
        // FIXME these should be properties too
        red: None,
        green: None,
        blue: None,
        extra_bytes: None,
    })
}

impl<P: 'static + AsRef<Path>> FileSink<P> for las::Writer<P> {
    fn open_file_sink(path: P, options: HashMap<String, String>) -> Result<Box<FileSink<P>>> {
        let mut writer = las::Writer::from_path(path);
        for (key, val) in options.iter() {
            match key.to_lowercase().as_ref() {
                "scale-factors" | "scaling" => {
                    let factors: Vec<_> = val.split(|c| c == ' ' || c == ',').collect();
                    if factors.len() != 3 {
                        return Err(Error::InvalidOption(format!("Incorrect number of scale \
                                                                 factors provided, wanted 3 \
                                                                 but got {}",
                                                                factors.len())));
                    }
                    writer = writer.scale_factors(try!(factors[0].parse()),
                                                  try!(factors[1].parse()),
                                                  try!(factors[2].parse()));
                }
                _ => {
                    return Err(Error::InvalidOption(format!("The las sink does not know how to \
                                                             handle this option: {}",
                                                            key)))
                }
            }
        }

        Ok(Box::new(writer))
    }

    fn close_file_sink(&mut self) -> Result<()> {
        self.close().map_err(|e| Error::from(e))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::fs::remove_file;

    use las;

    use sink::{open_file_sink, FileSink, Sink};
    use source::{open_file_source, Source};

    #[test]
    fn read_write_las() {
        let mut source = las::Stream::from_path("data/1.0_0.las").unwrap();
        let mut sink = las::Writer::from_path("read_write_las.las");
        for point in source.source_to_end(100).unwrap() {
            sink.sink(point).unwrap()
        }
        sink.close().unwrap();

        let mut source = las::Stream::from_path("read_write_las.las").unwrap();
        let points = source.source_to_end(100).unwrap();
        assert_eq!(1, points.len());
        remove_file("read_write_las.las").unwrap();
    }

    #[test]
    fn source_and_sink() {
        let mut source = open_file_source("data/1.0_0.las", HashMap::new()).unwrap();
        let mut sink = open_file_sink("source_and_sink.las", HashMap::new()).unwrap();
        for point in source.source_to_end(100).unwrap() {
            sink.sink(point).unwrap();
        }
        sink.close_file_sink().unwrap();

        let mut source = las::Stream::from_path("source_and_sink.las").unwrap();
        let points = source.source_to_end(100).unwrap();
        assert_eq!(1, points.len());
        remove_file("source_and_sink.las").unwrap();
    }
}
