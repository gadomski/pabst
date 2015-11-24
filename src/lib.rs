//! Point ABSTraction library in Rust.
//!
//! This library is inspired by [PDAL](http://www.pdal.io/), which provides both point abstraction
//! and a wide range of other abilitiles. **Pabst** hopes to keep its focus more narrow. We provide
//! source and sink abstractions for LiDAR point cloud file and stream formats. Other libraries
//! should be used to provide intermediate functionality.

#![deny(fat_ptr_transmutes, missing_copy_implementations, missing_debug_implementations, missing_docs, trivial_casts, trivial_numeric_casts, unsafe_code, unused_extern_crates, unused_import_braces, unused_qualifications, unused_results, variant_size_differences)]

extern crate las;
#[cfg(feature = "rxp")]
extern crate rivlib;
extern crate sdc;
#[cfg(feature = "sdf-convert")]
extern crate sdf;

pub mod error;
pub mod point;
pub mod source;
pub mod sink;

pub use error::Error;
pub use point::Point;
pub use source::Source;
pub use sink::Sink;

use std::result;

/// Our custom result type.
pub type Result<T> = result::Result<T, Error>;
