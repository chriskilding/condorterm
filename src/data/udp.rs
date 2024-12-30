use core::str;
use std::net::{IpAddr, SocketAddr, UdpSocket};

use super::client::{Backend, Datagram};

/// Condor UDP backend implementation
#[derive(Default, Copy, Clone, Debug)]
pub struct UdpBackend {
    port: u16,
}

impl UdpBackend {
    pub fn create(_host: IpAddr, port: u16) -> Self {
        Self { port }
    }
}

impl Backend for UdpBackend {
    /// Read the latest Condor UDP input
    fn receive(&self) -> Datagram {
        let address = SocketAddr::from(([127, 0, 0, 1], self.port));

        if let Ok(socket) = UdpSocket::bind(address) {
            let mut buffer = [0; 1024];

            let _ = socket.recv(&mut buffer);

            // Convert the binary buffer to string
            match str::from_utf8(&buffer) {
                Ok(s) => {
                    // Parse the message
                    return parse(s);
                }
                Err(_) => {}
            }
        }

        // Ignore the message
        return Datagram::default();
    }
}

fn update(datagram: &mut Datagram, key: &str, value: f32) {
    match key {
        "airspeed" => {
            datagram.airspeed = value;
        }
        "altitude" => {
            datagram.altitude = value;
        }
        "compass" => {
            datagram.compass = value;
        }
        "gforce" => {
            datagram.gforce = value;
        }
        "slipball" => {
            datagram.slipball = value;
        }
        "time" => {
            datagram.time = value;
        }
        "vario" => {
            datagram.vario = value;
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

fn create_socket(host: &IpAddr, port: u16) -> core::result::Result<UdpSocket, String> {
    let address = SocketAddr::new(*host, port);

    match UdpSocket::bind(address) {
        Ok(socket) => Ok(socket),
        Err(_) => Err("unable to listen on UDP socket".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_airspeed() {
        let msg = "airspeed=58.0";
        let result = parse(msg);
        assert_eq!(result.airspeed as u32, 58);
    }

    #[test]
    fn should_parse_time() {
        let msg = "time=12.001";
        let result = parse(msg);
        assert_eq!(result.time as u32, 12);
    }

    #[test]
    fn should_parse_altitude() {
        let msg = "altitude=810.002";
        let result = parse(msg);
        assert_eq!(result.altitude as u32, 810);
    }

    #[test]
    fn should_parse_vario() {
        let msg = "vario=2.01";
        let result = parse(msg);
        assert_eq!(result.vario as u32, 2);
    }

    #[test]
    fn should_parse_compass() {
        let msg = "compass=79.0";
        let result = parse(msg);
        assert_eq!(result.compass as u32, 79);
    }

    #[test]
    fn should_parse_slipball() {
        let msg = "slipball=0.001";
        let result = parse(msg);
        assert_eq!(result.slipball as u32, 0);
    }

    #[test]
    fn should_parse_gforce() {
        let msg = "gforce=1.0002";
        let result = parse(msg);
        assert_eq!(result.gforce as u32, 1);
    }

    #[test]
    fn should_parse_duplicates_and_use_latest_data() {
        let msg = "
airspeed=58.0
airspeed=65.0
        ";
        let result = parse(msg);
        assert_eq!(result.airspeed as u32, 65);
    }

    #[test]
    fn should_parse_multiple_fields() {
        let msg = "
airspeed=58.0
time=12.001
altitude=810.002
        ";

        let result = parse(msg);

        assert_eq!(result.airspeed as u32, 58);
        assert_eq!(result.time as u32, 12);
        assert_eq!(result.altitude as u32, 810);
    }
}
