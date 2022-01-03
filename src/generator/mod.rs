use std::net::SocketAddr;
use strum_macros::{Display, EnumString};

// Protocol specific modules
mod raw;
mod tcp;
mod udp;

/// Enum to define the protocol
#[derive(EnumString, Display)]
#[strum(ascii_case_insensitive)]
pub enum Protocol {
    Tcp,
    Udp,
    Raw,
}

/// Generate trait
pub trait Generator {
    /// Start the generator
    fn start(&self, data: Vec<u8>, packet_count: i32);
    /// Get the destination address
    fn get_destination_addr(&self) -> SocketAddr;
    /// Get the local address if specified
    fn get_local_addr(&self) -> SocketAddr;
    /// Get protocol of generator
    fn get_protocol(&self) -> Protocol;
    /// Get interface if Specified
    fn get_interface(&self) -> Option<String>;
}

pub fn create_generator(
    destination_address: String,
    destination_port: u16,
    local_address: Option<String>,
    local_port: Option<u16>,
    protocol: Protocol,
    interface: Option<String>,
) -> Box<dyn Generator> {
    let gen = match protocol {
        Protocol::Raw => {
            // return RawGenerator
            return Box::new(raw::RawGenerator::new(
                destination_address,
                destination_port,
                local_address,
                local_port,
                interface,
            ));
        }
        Protocol::Tcp => {
            // return TcpGenerator
            return Box::new(tcp::TcpGenerator::new(
                destination_address,
                destination_port,
                local_address,
                local_port,
                interface,
            ));
        }
        Protocol::Udp => {
            // return UdpGenerator
            return Box::new(udp::UdpGenerator::new(
                destination_address,
                destination_port,
                local_address,
                local_port,
                interface,
            ));
        }
    };
}

/*
/// Struct to hold generator properties
pub struct Generator {
    dest_address: SocketAddr,
    protocol: Protocol,
    local_address: Option<SocketAddr>,
    interface: Option<String>,
}

impl Generator {
    /// Constructor: create new generator
    pub fn new(
        destination_address: String,
        destination_port: u16,
        protocol: Protocol,
        local_address: Option<String>,
        local_port: Option<u16>,
        interface: Option<String>,
    ) -> Generator {
        // Fill new generator struct
        Generator {
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
            protocol: protocol,
            interface: interface,
        }
    }

    pub fn start(&self, data: Vec<u8>, packet_count: i32) {
        match self.protocol {
            Protocol::Raw => raw::start(), // TODO: implement raw generator
            Protocol::Tcp => tcp::start(), // TODO: implement tcp generator
            Protocol::Udp => udp::start(), // TODO: implement udp generator
        }
    }
}
*/
