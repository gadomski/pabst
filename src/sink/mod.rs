//! Point sinks.

pub mod las;

use point::Point;
use Result;

pub trait Sink {
    fn sink<P: Point>(&mut self, point: &P) -> Result<()>;
}
