//! Implement sink for .sdc.

use std::io::Write;

use sdc;

use Result;
use error::Error;
use point::Point;
use sink::Sink;

impl<W: Write> Sink for sdc::Writer<W> {
    fn sink(&mut self, point: &Point) -> Result<()> {
        try!(self.write_point(&try!(from_point(point))));
        Ok(())
    }

    fn close_sink(self: Box<Self>) -> Result<()> {
        Ok(())
    }
}

fn from_point(point: &Point) -> Result<sdc::Point> {
    Ok(sdc::Point {
        time: try!(point.gps_time.ok_or(Error::MissingDimension("time".to_string()))),
        range: point.range.unwrap_or_else(|| {
            (point.x.powi(2) + point.y.powi(2) + point.z.powi(2)).sqrt()
        }) as f32,
        theta: try!(point.scan_angle
                         .ok_or(Error::MissingDimension("scan angle"
                                                            .to_string()))) as f32,
        x: point.x as f32,
        y: point.y as f32,
        z: point.z as f32,
        amplitude: point.intensity.as_u16(),
        width: point.width.unwrap_or(0.0) as u16,
        target: point.return_number.unwrap_or(0) as u8,
        num_target: point.number_of_returns.unwrap_or(0) as u8,
        rg_index: point.rg_index.unwrap_or(0.0) as u16,
        target_type: try!(sdc::TargetType::from_u8(point.target_type.unwrap_or(3))),
        facet_number: point.facet_number.unwrap_or(0),
        high_channel: point.high_channel.unwrap_or(false),
        class_id: Some(point.classification),
        rho: None,
        reflectance: None,
    })
}

#[cfg(test)]
mod tests {
    use std::fs::remove_file;

    use las;
    use sdc;

    use source::Source;
    use sink::Sink;

    #[test]
    fn sink_las() {
        let mut source = las::Reader::from_path("data/1.0_1.las").unwrap();
        {
            let mut sink = sdc::Writer::from_path("temp.sdc").unwrap();
            for point in &source.source_to_end(100).unwrap() {
                sink.sink(point).unwrap();
            }
        }
        let mut source = sdc::Reader::from_path("temp.sdc").unwrap();
        let points = source.source_to_end(100).unwrap();
        assert_eq!(1, points.len());
        remove_file("temp.sdc").unwrap();
    }
}
