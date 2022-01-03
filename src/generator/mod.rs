use std::fmt;
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
    fn start(&self, data: Vec<u8>, packet_count: i32) -> Result<(), GeneratorError>;
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
    match protocol {
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GeneratorError {
    SocketCreationError,
    ConnectionError,
    SetSocketOptionError(String),
    SendError,
}

impl fmt::Display for GeneratorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GeneratorError::SocketCreationError => "could not create socket".fmt(f),
            GeneratorError::ConnectionError => "could not connect to destination".fmt(f),
            GeneratorError::SetSocketOptionError(option) => {
                format!("could not set socket option: {}", option).fmt(f)
            }
            GeneratorError::SendError => "could not send data to socket".fmt(f),
        }
    }
}

impl std::error::Error for GeneratorError {}
