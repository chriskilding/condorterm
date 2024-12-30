use super::client::{Backend, Datagram};
use rand::Rng;

/// Fake backend that generates random plausible data
#[derive(Default, Copy, Clone, Debug)]
pub struct RandomBackend {}

impl Backend for RandomBackend {
    fn receive(&self) -> Datagram {
        let mut rng = rand::thread_rng();

        Datagram {
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
