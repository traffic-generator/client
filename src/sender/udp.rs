//! # UDP sender
//!
//! Send data using UDP

use std::{
    io,
    net::{Ipv4Addr, Ipv6Addr, SocketAddr},
    vec,
};

use socket2::{Domain, Protocol, SockAddr, Socket, Type};

use super::Send;

pub struct UdpSender {
    dst: SockAddr,
    src: SockAddr,
    sock: Socket,
}

impl Send for UdpSender {
    fn send(&self, data: Vec<u8>) {
        self.sock.send_to(&data, &self.dst).unwrap();
    }

    fn setup(&mut self) {
        self.sock.bind(&self.src).unwrap();
        self.sock.connect(&self.dst).unwrap();
    }
}

/// Convert a destination socket address to an UdpSender
impl From<SocketAddr> for UdpSender {
    fn from(dst_sock_addr: SocketAddr) -> Self {
        // Create source address with the same IP version as destination
        let src_sock_addr = match dst_sock_addr.is_ipv4() {
            true => SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), 0),
            false => SocketAddr::new(Ipv6Addr::UNSPECIFIED.into(), 0),
        };
        println!("{:#?}", src_sock_addr.ip());
        let dst_sock = Socket::new(
            Domain::for_address(dst_sock_addr),
            Type::DGRAM,
            Some(Protocol::UDP),
        )
        .unwrap();
        UdpSender {
            dst: dst_sock_addr.into(),
            src: src_sock_addr.into(),
            sock: dst_sock,
        }
    }
}

pub fn test_flow() {
    let data = vec![62, 62, 62, 62, 2, 62, 62, 62, 2, 62, 62, 62, 2, 62, 62, 62];
    let src_address = SocketAddr::from(([192, 168, 1, 78], 7848));
    let dst_address = SocketAddr::from(([192, 168, 1, 78], 7848));
    let socket = Socket::new(Domain::IPV6, Type::DGRAM, Some(Protocol::UDP)).unwrap();
    socket.bind(&src_address.into()).unwrap();
    socket.send_to(&data, &dst_address.into()).unwrap();
}
