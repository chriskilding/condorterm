use chrono::NaiveTime;
use rand::Rng;

/// Condor's raw input
#[derive(Debug, Copy, Clone)]
pub struct Datagram {
    pub airspeed: f32,
    /// Altimeter reading (m or ft)
    pub altitude: f32,
    /// Compass reading (degrees)
    pub compass: f32,
    /// g force factor
    pub gforce: f32,
    /// Slip ball deflection angle (radians)
    pub slipball: f32,
    /// In-game display time (decimal-hours)
    pub time: f32,
    /// Pneumatic vario reading (m/s)
    pub vario: f32,
}

impl Default for Datagram {
    fn default() -> Self {
        Self {
            airspeed: 0.0,
            altitude: 0.0,
            compass: 0.0,
            gforce: 1.0,
            slipball: 0.0,
            time: 0.0,
            vario: 0.0,
        }
    }
}

impl Datagram {
    /// Generate a Datagram with random values.
    /// Useful for testing.
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();

        Self {
            airspeed: rng.gen_range(0.0..160.0),
            altitude: rng.gen_range(0.0..30000.0),
            compass: rng.gen_range(0.0..359.0),
            gforce: rng.gen_range(-5.0..8.0),
            slipball: rng.gen_range(-1.0..1.0),
            time: rng.gen_range(0.0..23.99),
            vario: rng.gen_range(-12.0..12.0),
        }
    }
}

/// Convert decimal-hours to a Time object
pub fn to_time(decimal_hour: f32) -> NaiveTime {
    let (hour, decimal) = to_fraction(decimal_hour);
    let min_sec = 60.0 * decimal;
    let min = min_sec as u32;
    let sec = (60.0 * (min_sec - (min as f32))) as u32;
    NaiveTime::from_hms_opt(hour, min, sec).unwrap_or(NaiveTime::MIN)
}

/// Split a float into its components before and after the decimal point
fn to_fraction(x: f32) -> (u32, f32) {
    let integer = x.floor();
    let decimal = x - integer;
    (integer as u32, decimal)
}
