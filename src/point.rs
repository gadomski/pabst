//! A catchall LiDAR point.

use std::u16;

/// A point.
#[derive(Clone, Copy, Debug, Default)]
pub struct Point {
    /// The x dimension of a point. All points must have these.
    pub x: f64,
    /// The y dimension of a point. All points must have these.
    pub y: f64,
    /// The z dimension of a point. All points must have these.
    pub z: f64,
    /// This is the intensity of the backscattered energy, as detected by the LiDAR sensor. Since
    /// each sensor has its own dynamic range, an intensity value is actually three values — the
    /// intensity value itself, the minimum possible value, and the maximum possible value.
    pub intensity: Intensity,
    /// The return number of this point.
    pub return_number: Option<usize>,
    /// The total number of returns in the pulse of this point.
    pub number_of_returns: Option<usize>,
    /// The direction that the scanner mirror was travelling when the point was collected. The only
    /// reason we include this field is that it is in the las standard. Anyone that doesn't collect
    /// scan direction information should just keep this at the default of
    /// `ScanDirection::Unknown`, and let the las sink deal with it.
    pub scan_direction: ScanDirection,
    /// Is this point at the edge of a flight line? Again, a crappy field that was included just
    /// becuase las.
    pub edge_of_flight_line: bool,
    /// The classification of this point.
    ///
    /// I'm not sure if I want to really define out the ASPRS classification mappings in this
    /// library, or leave that to the las library. I'm leaning towards keeping that in las just
    /// because I don't like classifications as a general concept, and so I try to stay away from
    /// them whenever I can.
    ///
    /// The value of `0` is "Created, never classified" according to at least one of the las
    /// specifications, so this is a sensible default value.
    pub classification: u8,
    /// Is this a synthetic point? Urgh. las again. Just keep this false unless someone out there
    /// is actually using the synthetic field.
    pub synthetic: bool,
    /// Okay, las, model key point? Really? You're taking up a perfectly good bit in *every single
    /// las point* just for this field? Really?
    pub key_point: bool,
    /// It's not even worth complaining anymore. Just look at a las standard to see what all these
    /// crappy fields mean. And while you're at it, try clicking on some of the URLs *in the las
    /// standards*. I'll bet some of them are broken. That's how good ASPRS are (is?).
    pub withheld: bool,
    /// Whew, back into the normal realm of fields. This is the angle of the mirror when the point
    /// was captured. Angle is usually defined with zero being straight out of the scanner.
    ///
    /// Las, in all of its wisdom, only stores these as integers, but more sensible formats allow
    /// for finer resolution, so we dial this up to `f64` and let the las sink throw away perfectly
    /// good data.
    pub scan_angle: Option<f64>,
    /// Why, las, why. What is this, even.
    pub point_source_id: Option<u16>,
    /// Okay, I'm done complaining now. In all seriousness, I thought about throwing this field
    /// out, in particular, but I figure since las is so used I should just mirror it. But that
    /// doesn't mean I can't whine about it.
    pub user_data: Option<u8>,
    /// GNSS synchronized time.
    pub gps_time: Option<f64>,
    /// The range of the point from the scanner. Note that for some formats, this explicity *does
    /// not* equal the cartesian distance to the x, y, z location of the point. This is a bit of an
    /// esoteric field but we include it because Riegl uses it a decent amount.
    pub range: Option<f64>,
    /// The width of the received pulse. Again, a pretty esoteric field, this time from the Riegl
    /// side of things. I get thinking more and more how to deal with these strange fields like
    /// this.
    pub width: Option<f64>,
    /// Man, this is the range index, and I have *no* idea what it means.
    pub rg_index: Option<f64>,
    /// The number of the mirror facet that was used to send this pulse.
    pub facet_number: Option<u8>,
    /// The target type, as an integer. Riegl uses some coded values for target type (Gaussian,
    /// Center of Gravity, etc), but at this point we don't choose to propagate those values up to
    /// this level.
    pub target_type: Option<u8>,
    /// Is this point from a high channel?
    ///
    /// TODO this field should be turned into a newtype or enum
    pub high_channel: Option<bool>,
    /// When doing error propagation, it can be useful to carry along a point's partial derivatives
    /// with respect to each of the fourteen degrees of freedom in the LiDAR georeferencing
    /// equation. If we have the partials, we can combine them with compoment errors to get final
    /// propagated error.
    pub partials: Option<Partials>,
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

impl Default for ScanDirection {
    fn default() -> ScanDirection {
        ScanDirection::Unknown
    }
}

/// An intensity value, with information about its minimum and maximum possible values.
#[derive(Clone, Copy, Debug, Default)]
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

/// A 3x14 collection of partial derivates for the x, y, and z components of a LiDAR point.
#[derive(Clone, Copy, Debug)]
pub struct Partials {
    range: Xyz,
    scan_angle: Xyz,
    boresight_roll: Xyz,
    boresight_pitch: Xyz,
    boresight_yaw: Xyz,
    lever_arm_x: Xyz,
    lever_arm_y: Xyz,
    lever_arm_z: Xyz,
    roll: Xyz,
    pitch: Xyz,
    yaw: Xyz,
    gnss_x: Xyz,
    gnss_y: Xyz,
    gnss_z: Xyz,
}

/// A dumb structure of xyz f64s.
#[derive(Clone, Copy, Debug)]
pub struct Xyz {
    x: f64,
    y: f64,
    z: f64,
}
