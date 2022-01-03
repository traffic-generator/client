use crate::generator::{Generator, GeneratorError, Protocol};
use socket2::{Domain, Socket, Type};
use std::net::{IpAddr, SocketAddr};

/// Struct to hold generator properties
pub struct TcpGenerator {
    dest_address: SocketAddr,
    local_address: Option<SocketAddr>,
    interface: Option<String>,
}

/// Implement generate trait
impl Generator for TcpGenerator {
    /// Start the TcpGenerator
    fn start(&self, data: Vec<u8>, packet_count: i32) -> Result<(), GeneratorError> {
        // Create socket
        let sock: Socket =
            match Socket::new(Domain::for_address(self.dest_address), Type::STREAM, None) {
                Ok(s) => s,
                Err(_e) => return Err(GeneratorError::SocketCreationError),
            };
        // Connect to destination
        match sock.connect(&self.dest_address.into()) {
            Ok(__) => {}
            Err(_e) => return Err(GeneratorError::ConnectionError),
        }
        // Disable Nagle's algorithm
        // SOURCE: https://doi.org/10.1145/382176.382177
        match sock.set_nodelay(true) {
            Ok(__) => {}
            Err(_e) => return Err(GeneratorError::SetSocketOptionError("nodelay".to_string())),
        }
        // Send data
        for _x in 0..packet_count {
            let bytes_send = match sock.send(&data) {
                Ok(b) => b,
                Err(_e) => return Err(GeneratorError::SendError),
            };
            println!("[TCP] send {} bytes", bytes_send);
        }
        Ok(())
    }
    fn get_destination_addr(&self) -> SocketAddr {
        self.dest_address
    }
    fn get_local_addr(&self) -> SocketAddr {
        self.local_address
            .unwrap_or(SocketAddr::new("172.0.0.1".parse().unwrap(), 0))
    }
    fn get_protocol(&self) -> Protocol {
        Protocol::Tcp
    }
    fn get_interface(&self) -> Option<String> {
        self.interface.clone()
    }
}

impl TcpGenerator {
    /// Create new TcpGenerator
    pub fn new(
        destination_address: String,
        destination_port: u16,
        local_address: Option<String>,
        local_port: Option<u16>,
        interface: Option<String>,
    ) -> TcpGenerator {
        // Fill new generator struct
        TcpGenerator {
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
