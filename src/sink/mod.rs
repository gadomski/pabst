//! Point sinks.

pub mod las;
pub mod sdc;

use point::Point;
use Result;

/// A point sink.
///
/// A sink is a place where points go. Mabye they're written to disk. Maybe not.
pub trait Sink {
    /// Sink a single point into this sink.
    fn sink<P: Point>(&mut self, point: &P) -> Result<()>;
}
