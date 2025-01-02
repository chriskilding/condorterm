use async_trait::async_trait;

/// Condor client
pub struct Client {
    backend: Box<dyn Backend>,
}

impl Client {
    pub fn new(backend: impl Backend + 'static) -> Self {
        let b = Box::new(backend);

        Self { backend: b }
    }

    pub async fn receive(&self) -> Datagram {
        self.backend.receive().await
    }
}

/// Client backend strategy
#[async_trait]
pub trait Backend: Send + Sync {
    async fn receive(&self) -> Datagram;
}

/// Condor's raw input
#[derive(Debug, Copy, Clone, Default)]
pub struct Datagram {
    pub airspeed: Option<f32>,
    /// Altimeter reading (m or ft)
    pub altitude: Option<f32>,
    /// Compass reading (degrees)
    pub compass: Option<f32>,
    /// g force factor
    pub gforce: Option<f32>,
    /// Slip ball deflection angle (radians)
    pub slipball: Option<f32>,
    /// In-game display time (decimal-hours)
    pub time: Option<f32>,
    /// Pneumatic vario reading (m/s)
    pub vario: Option<f32>,
}
