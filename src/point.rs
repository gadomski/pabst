//! A trait for a LiDAR point
//!
//! This is is a catchall trait that has a bunch of optional fields. This lets sources choose what
//! values to fill in, and sinks choose what values to pay attention to.

use std::u16;

use Result;

/// A point.
pub trait Point {
    /// The x dimension of a point. All points must have these.
    fn x(&self) -> f64;

    /// The y dimension of a point. All points must have these.
    fn y(&self) -> f64;

    /// The z dimension of a point. All points must have these.
    fn z(&self) -> f64;

    /// This is the intensity of the backscattered energy, as detected by the LiDAR sensor. Since
    /// each sensor has its own dynamic range, an intensity value is actually three values — the
    /// intensity value itself, the minimum possible value, and the maximum possible value.
    fn intensity(&self) -> Intensity;

    /// The return number of this point.
    fn return_number(&self) -> Option<usize>;

    /// The total number of returns in the pulse of this point.
    fn number_of_returns(&self) -> Option<usize>;

    /// The direction that the scanner mirror was travelling when the point was collected. The only
    /// reason we include this field is that it is in the las standard. Anyone that doesn't collect
    /// scan direction information should just keep this at the default of
    /// `ScanDirection::Unknown`, and let the las sink deal with it.
    fn scan_direction(&self) -> ScanDirection {
        ScanDirection::Unknown
    }

    /// Is this point at the edge of a flight line? Again, a crappy field that was included just
    /// becuase las.
    fn edge_of_flight_line(&self) -> bool {
        false
    }

    /// The classification of this point.
    ///
    /// I'm not sure if I want to really define out the ASPRS classification mappings in this
    /// library, or leave that to the las library. I'm leaning towards keeping that in las just
    /// because I don't like classifications as a general concept, and so I try to stay away from
    /// them whenever I can.
    ///
    /// The value of `0` is "Created, never classified" according to at least one of the las
    /// specifications, so this is a sensible default value.
    fn classification(&self) -> u8 {
        0
    }

    /// Is this a synthetic point? Urgh. las again. Just keep this false unless someone out there
    /// is actually using the synthetic field.
    fn synthetic(&self) -> bool {
        false
    }

    /// Okay, las, model key point? Really? You're taking up a perfectly good bit in *every single
    /// las point* just for this field? Really?
    fn key_point(&self) -> bool {
        false
    }

    /// It's not even worth complaining anymore. Just look at a las standard to see what all these
    /// crappy fields mean. And while you're at it, try clicking on some of the URLs *in the las
    /// standards*. I'll bet some of them are broken. That's how good ASPRS are (is?).
    fn withheld(&self) -> bool {
        false
    }

    /// Whew, back into the normal realm of fields. This is the angle of the mirror when the point
    /// was captured. Angle is usually defined with zero being straight out of the scanner.
    ///
    /// Las, in all of its wisdom, only stores these as integers, but more sensible formats allow
    /// for finer resolution, so we dial this up to `f64` and let the las sink throw away perfectly
    /// good data.
    fn scan_angle(&self) -> Option<f64> {
        None
    }

    /// Why, las, why. What is this, even.
    fn point_source_id(&self) -> Option<u16> {
        None
    }

    /// Okay, I'm done complaining now. In all seriousness, I thought about throwing this field
    /// out, in particular, but I figure since las is so used I should just mirror it. But that
    /// doesn't mean I can't whine about it.
    fn user_data(&self) -> Option<u8> {
        None
    }

    /// GNSS synchronized time.
    fn gps_time(&self) -> Option<f64> {
        None
    }

    /// The range of the point from the scanner. Note that for some formats, this explicity *does
    /// not* equal the cartesian distance to the x, y, z location of the point. This is a bit of an
    /// esoteric field but we include it because Riegl uses it a decent amount.
    fn range(&self) -> Option<f64> {
        None
    }

    /// The width of the received pulse. Again, a pretty esoteric field, this time from the Riegl
    /// side of things. I get thinking more and more how to deal with these strange fields like
    /// this.
    fn width(&self) -> Option<f64> {
        None
    }

    /// Man, this is the range index, and I have *no* idea what it means.
    fn rg_index(&self) -> Option<f64> {
        None
    }

    /// The number of the mirror facet that was used to send this pulse.
    fn facet_number(&self) -> Option<u8> {
        None
    }

    /// The target type, as an integer. Riegl uses some coded values for target type (Gaussian,
    /// Center of Gravity, etc), but at this point we don't choose to propagate those values up to
    /// this level.
    fn target_type(&self) -> Option<u8> {
        None
    }

    /// Is this point from a high channel?
    ///
    /// TODO this field should be turned into a newtype or enum
    fn high_channel(&self) -> Option<bool> {
        None
    }
}

/// Implemented by structs that can be created from a `Point`.
pub trait FromPoint {
    /// Creates a new point from an object that implements `Point`.
    fn from_point<P: Point>(point: &P) -> Result<Self> where Self: Sized;
}

/// The direction that the scanner mirror was moving when the pulse was emitted.
#[derive(Clone, Copy, Debug)]
pub enum ScanDirection {
    /// Forward.
    Forward,
    /// Backwards.
    Backward,
    /// The scan direction is undefined.
    Unknown,
}

/// An intensity value, with information about its minimum and maximum possible values.
#[derive(Clone, Copy, Debug)]
pub struct Intensity {
    value: f64,
    min: f64,
    max: f64,
}

impl Intensity {
    /// Creates a new intensity with the min and the max set to the u16 bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use pabst::point::Intensity;
    /// let intensity = Intensity::from_u16(10);
    /// ```
    pub fn from_u16(value: u16) -> Intensity {
        Intensity {
            value: value as f64,
            min: u16::MIN as f64,
            max: u16::MAX as f64,
        }
    }


    /// Creates a new intensity with the value, minimu, and maximum.
    ///
    /// # Examples
    ///
    /// ```
    /// use pabst::point::Intensity;
    /// let intensity = Intensity::new(1.0, 0.0, 2.0);
    /// assert_eq!(32767, intensity.as_u16());
    /// ```
    pub fn new(value: f64, min: f64, max: f64) -> Intensity {
        Intensity {
            value: value,
            min: min,
            max: max,
        }
    }

    /// Returns this intensity value as a u16.
    ///
    /// # Examples
    ///
    /// ```
    /// use pabst::point::Intensity;
    /// let intensity = Intensity::from_u16(10);
    /// assert_eq!(10, intensity.as_u16());
    /// ```
    pub fn as_u16(&self) -> u16 {
        (u16::MAX as f64 * (self.value - self.min) / (self.max - self.min)) as u16
    }
}
