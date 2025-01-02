use super::client::{Backend, Datagram};
use async_trait::async_trait;
use rand::Rng;

/// Fake backend that generates random plausible data
#[derive(Default, Copy, Clone, Debug)]
pub struct RandomBackend {}

#[async_trait]
impl Backend for RandomBackend {
    async fn receive(&self) -> Datagram {
        let mut rng = rand::thread_rng();

        Datagram {
            airspeed: Some(rng.gen_range(0.0..160.0)),
            altitude: Some(rng.gen_range(0.0..30000.0)),
            compass: Some(rng.gen_range(0.0..359.0)),
            gforce: Some(rng.gen_range(-5.0..8.0)),
            slipball: Some(rng.gen_range(-1.0..1.0)),
            time: Some(rng.gen_range(0.0..23.99)),
            vario: Some(rng.gen_range(-12.0..12.0)),
        }
    }
}
