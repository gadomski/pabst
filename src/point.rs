//! A trait for a LiDAR point
//!
//! This is is a catchall trait that has a bunch of optional fields. This lets sources choose what
//! values to fill in, and sinks choose what values to pay attention to.

/// A point.
pub trait Point {
    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn z(&self) -> f64;
    fn intensity(&self) -> u16;
    fn return_number(&self) -> Option<usize>;
    fn number_of_returns(&self) -> Option<usize>;
    fn scan_direction(&self) -> ScanDirection {
        ScanDirection::Unknown
    }
    fn edge_of_flight_line(&self) -> bool {
        false
    }
    fn classification(&self) -> u8 {
        0
    }
    fn synthetic(&self) -> bool {
        false
    }
    fn key_point(&self) -> bool {
        false
    }
    fn withheld(&self) -> bool {
        false
    }
    fn scan_angle(&self) -> f64 {
        0.0
    }
    fn user_data(&self) -> u8 {
        0
    }
    fn point_source_id(&self) -> u16 {
        0
    }
    fn gps_time(&self) -> Option<f64> {
        None
    }

    fn set_x(&mut self, x: f64);
    fn set_y(&mut self, y: f64);
    fn set_z(&mut self, z: f64);
}

pub enum ScanDirection {
    Forward,
    Backward,
    Unknown,
}
