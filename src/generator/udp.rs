use crate::generator::{Generator, GeneratorError, Protocol};
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
    fn start(&self, data: Vec<u8>, packet_count: i32) -> Result<(), GeneratorError> {
        println!("[UDP] Start was called");
        //TODO: Start sending UDP traffic
        todo!()
    }
    fn get_destination_addr(&self) -> SocketAddr {
        self.dest_address
    }
    fn get_local_addr(&self) -> SocketAddr {
        self.local_address
            .unwrap_or(SocketAddr::new("172.0.0.1".parse().unwrap(), 0))
    }
    fn get_protocol(&self) -> Protocol {
        Protocol::Udp
    }
    fn get_interface(&self) -> Option<String> {
        self.interface.clone()
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
