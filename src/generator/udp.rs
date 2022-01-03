use crate::generator::Generator;
use socket2::Socket;
use std::net::{IpAddr, SocketAddr};

/// Struct to hold generator properties
pub struct UdpGenerator {
    dest_address: SocketAddr,
    local_address: Option<SocketAddr>,
    interface: Option<String>,
}

/// Implement generate trait
impl Generator for UdpGenerator {
    /// Start the UdpGenerator
    fn start(&self, data: Vec<u8>, packet_count: i32) {
        println!("[UDP] Start was called");
        //TODO
    }
}

impl UdpGenerator {
    /// Create new UdpGenerator
    pub fn new(
        destination_address: String,
        destination_port: u16,
        local_address: Option<String>,
        local_port: Option<u16>,
        interface: Option<String>,
    ) -> UdpGenerator {
        // Fill new generator struct
        UdpGenerator {
            dest_address: SocketAddr::new(
                destination_address.parse::<IpAddr>().unwrap(),
                destination_port,
            ),
            local_address: match local_address {
                Some(address) => match local_port {
                    Some(port) => Some(SocketAddr::new(address.parse::<IpAddr>().unwrap(), port)),
                    None => Some(SocketAddr::new(address.parse::<IpAddr>().unwrap(), 0)),
                },
                None => None,
            },
            interface: interface,
        }
    }
}
