/// Condor client
pub struct Client {
    backend: Box<dyn Backend>,
}

impl Client {
    pub fn new(backend: impl Backend + 'static) -> Self {
        let b = Box::new(backend);

        Self { backend: b }
    }

    pub fn receive(&self) -> Datagram {
        self.backend.receive()
    }
}

/// Client backend strategy
pub trait Backend: Send + Sync {
    fn receive(&self) -> Datagram;
}

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
