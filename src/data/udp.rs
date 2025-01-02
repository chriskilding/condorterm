use async_trait::async_trait;
use core::str;
use smol::net::UdpSocket;

use super::client::{Backend, Datagram};

/// Condor UDP backend implementation
#[derive(Debug, Default)]
pub struct UdpBackend {
    socket: Option<UdpSocket>,
}

impl UdpBackend {
    pub fn new(socket: UdpSocket) -> Self {
        return Self {
            socket: Some(socket),
        };
    }
}

#[async_trait]
impl Backend for UdpBackend {
    /// Read the latest Condor UDP input
    async fn receive(&self) -> Datagram {
        if let Some(socket) = &self.socket {
            let mut buffer = [0; 1024];

            if let Ok(..) = socket.recv(&mut buffer).await {
                // Convert the binary buffer to string
                match str::from_utf8(&buffer) {
                    Ok(s) => {
                        // Parse the message
                        return parse(s);
                    }
                    Err(_) => {}
                }
            }
        }

        // Ignore the message
        return Datagram::default();
    }
}

fn update(datagram: &mut Datagram, key: &str, value: f32) {
    match key {
        "airspeed" => {
            datagram.airspeed = Some(value);
        }
        "altitude" => {
            datagram.altitude = Some(value);
        }
        "compass" => {
            datagram.compass = Some(value);
        }
        "gforce" => {
            datagram.gforce = Some(value);
        }
        "slipball" => {
            datagram.slipball = Some(value);
        }
        "time" => {
            datagram.time = Some(value);
        }
        "vario" => {
            datagram.vario = Some(value);
        }
        _ => {
            // do nothing
        }
    }
}

/// Parse the Condor UDP datagram format
// key1=value1
// key2=value2
// etc
fn parse(s: &str) -> Datagram {
    let mut datagram = Datagram::default();

    for line in s.lines() {
        if let Some((key, value)) = line.split_once("=") {
            if let Ok(float_value) = value.parse::<f32>() {
                update(&mut datagram, key, float_value);
            }
        }
    }

    datagram
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_airspeed() {
        let msg = "airspeed=58.0";
        let result = parse(msg);
        assert_eq!(result.airspeed.unwrap() as u32, 58);
    }

    #[test]
    fn should_parse_time() {
        let msg = "time=12.001";
        let result = parse(msg);
        assert_eq!(result.time.unwrap() as u32, 12);
    }

    #[test]
    fn should_parse_altitude() {
        let msg = "altitude=810.002";
        let result = parse(msg);
        assert_eq!(result.altitude.unwrap() as u32, 810);
    }

    #[test]
    fn should_parse_vario() {
        let msg = "vario=2.01";
        let result = parse(msg);
        assert_eq!(result.vario.unwrap() as u32, 2);
    }

    #[test]
    fn should_parse_compass() {
        let msg = "compass=79.0";
        let result = parse(msg);
        assert_eq!(result.compass.unwrap() as u32, 79);
    }

    #[test]
    fn should_parse_slipball() {
        let msg = "slipball=0.001";
        let result = parse(msg);
        assert_eq!(result.slipball.unwrap() as u32, 0);
    }

    #[test]
    fn should_parse_gforce() {
        let msg = "gforce=1.0002";
        let result = parse(msg);
        assert_eq!(result.gforce.unwrap() as u32, 1);
    }

    #[test]
    fn should_parse_duplicates_and_use_latest_data() {
        let msg = "
airspeed=58.0
airspeed=65.0
        ";
        let result = parse(msg);
        assert_eq!(result.airspeed.unwrap() as u32, 65);
    }

    #[test]
    fn should_parse_multiple_fields() {
        let msg = "
airspeed=58.0
time=12.001
altitude=810.002
        ";

        let result = parse(msg);

        assert_eq!(result.airspeed.unwrap() as u32, 58);
        assert_eq!(result.time.unwrap() as u32, 12);
        assert_eq!(result.altitude.unwrap() as u32, 810);
    }

    #[test]
    fn should_ignore_incomplete_fields() {
        let msg = "airspeed=";

        let result = parse(msg);

        assert_eq!(result.airspeed, None);
    }

    #[test]
    fn should_ignore_field_key_without_assignment() {
        let msg = "airspeed";

        let result = parse(msg);

        assert_eq!(result.airspeed, None);
    }
}
