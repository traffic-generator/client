mod generator;
mod log;
mod sender;

use std::{net::SocketAddr, path::Path};

use generator::{udp::UdpPacket, Packet};
use log::Logger;
use sender::{udp::UdpSender, Send};

const DEFAULT_PAYLOAD: &str = "Hello World";

fn main() {
    // Setup logger
    let mut log = Logger::new();
    log.log_to_file(Path::new("./logs/log.log"));
    log.time();
    log.info("Start traffic generator client".to_string());

    // Define default payload for debugging
    let payload = DEFAULT_PAYLOAD.as_bytes().to_vec();
    log.debug(format!("Packet payload: {:02x?}", payload));

    // Create packet
    let mut pkt = UdpPacket::new(0xa08f, 0x2694);
    pkt.set_payload(payload);
    pkt.update_length();
    pkt.update_checksum(0x9801331b, 0x980e5e4b);
    let pkt_vec = pkt.to_vec();
    log.debug(format!("Entire packet: {:02x?}", pkt_vec));

    // Send packet using udp sender
    // let dest_addr = SocketAddr::from(([192, 168, 1, 78], 7848));
    // let mut udp_sender = UdpSender::from(dest_addr);
    // udp_sender.setup();
    // udp_sender.send(pkt_vec);
    sender::udp::test_flow();
}
