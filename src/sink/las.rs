//! Sink points into a las file.

use std::io::{Write, Seek};
use std::path::Path;

use las;
use toml;

use Result;
use error::Error;
use point::{Point, ScanDirection};
use sink::{FileSink, Sink};

impl<W: Write + Seek> Sink for las::writer::OpenWriter<W> {
    fn sink(&mut self, point: Point) -> Result<()> {
        try!(self.write_point(&try!(from_point(point))));
        Ok(())
    }

    fn close_sink(self: Box<Self>) -> Result<()> {
        self.close().map_err(|e| Error::from(e)).map(|_| ())
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

impl<W: Write + Seek> FileSink for las::Writer<W> {
    fn open_file_sink<P: AsRef<Path>>(path: P, options: Option<&toml::Table>) -> Result<Box<Sink>> {
        let mut writer = try!(las::Writer::from_path(path));
        if let Some(options) = options {
            for (key, val) in options.iter() {
                match key.to_lowercase().as_ref() {
                    "scale-factors" | "scaling" => {
                        if let &toml::Value::Array(ref array) = val {
                            if array.len() != 3 {
                                return Err(Error::InvalidOption("Incorrect number of scale \
                                                                 factors"
                                                                    .to_string()));
                            }
                            let mut factors = Vec::with_capacity(3);
                            for value in array {
                                factors.push(try!(value.as_float()
                                                       .ok_or(Error::InvalidOption("Unable to \
                                                                                    parse scale \
                                                                                    factor as \
                                                                                    float"
                                                                                       .to_string()))));
                            }
                            writer = writer.scale_factors(factors[0], factors[1], factors[2]);
                        } else {
                            return Err(Error::InvalidOption("Invalid value for scale factor"
                                                                .to_string()));
                        }
                    }
                    "auto-offset" | "auto-offsets" => writer = writer.auto_offsets(try!(val.as_bool()
                                                             .ok_or(Error::InvalidOption("Unable to \
                                                                                    parse auto \
                                                                                    offset as \
                                                                                    boolean".to_string())))),
                    _ => {
                        return Err(Error::InvalidOption(format!("The las sink does not know \
                                                                 how to handle this option: {}",
                                                                key)));
                    }
                }
            }
        }

        Ok(Box::new(try!(writer.open())))
    }
}

#[cfg(test)]
mod tests {
    use std::fs::remove_file;

    use las;

    use sink::{open_file_sink, Sink};
    use source::{open_file_source, Source};

    #[test]
    fn read_write_las() {
        let mut source = las::Reader::from_path("data/1.0_0.las").unwrap();
        let mut sink = las::Writer::from_path("read_write_las.las").unwrap().open().unwrap();
        for point in source.source_to_end(100).unwrap() {
            sink.sink(point).unwrap()
        }
        let _ = sink.close().unwrap();

        let mut source = las::Reader::from_path("read_write_las.las").unwrap();
        let points = source.source_to_end(100).unwrap();
        assert_eq!(1, points.len());
        remove_file("read_write_las.las").unwrap();
    }

    #[test]
    fn source_and_sink() {
        let mut source = open_file_source("data/1.0_0.las", None).unwrap();
        let mut sink = open_file_sink("source_and_sink.las", None).unwrap();
        for point in source.source_to_end(100).unwrap() {
            sink.sink(point).unwrap();
        }
        sink.close_sink().unwrap();

        let mut source = las::Reader::from_path("source_and_sink.las").unwrap();
        let points = source.source_to_end(100).unwrap();
        assert_eq!(1, points.len());
        remove_file("source_and_sink.las").unwrap();
    }
}
