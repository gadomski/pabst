//! Point ABSTraction library in Rust.
//!
//! This library is inspired by [PDAL](http://www.pdal.io/), which provides both point abstraction
//! and a wide range of other abilitiles. **Pabst** hopes to keep its focus more narrow. We provide
//! source and sink abstractions for LiDAR point cloud file and stream formats. Other libraries
//! should be used to provide intermediate functionality.

extern crate las;
#[cfg(feature = "rxp")]
extern crate rivlib;

pub mod error;
pub mod point;
pub mod source;
pub mod sink;

use std::result;

pub type Result<T> = result::Result<T, error::Error>;
