//! Sources of points.
//!
//! These don't necessarily have to be file format readers, but they usually are.

pub mod las;
pub mod sdc;
#[cfg(feature = "rxp")]
pub mod rxp;

use point;
use Result;

/// A point source.
pub trait Source {
    /// The type of point that this source produces.
    type Point: point::Point;

    /// Sources some points from the `Source`.
    ///
    /// Use `want` to request a certain number of points, but the source is not compelled to return
    /// exactly that number. This method returns an optional vector of points, or `None` if the
    /// source is out of points.
    fn source(&mut self, want: usize) -> Result<Option<Vec<Self::Point>>>;

    /// Sources all points in this `Source`.
    ///
    /// If the source is an infinite stream, it should re-define this method to return an error.
    fn source_to_end(&mut self, want: usize) -> Result<Vec<Self::Point>> {
        let mut points = Vec::new();
        loop {
            match try!(self.source(want)) {
                Some(ref mut p) => points.append(p),
                None => return Ok(points),
            }
        }
    }
}
