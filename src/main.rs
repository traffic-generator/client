mod log;
mod proto;

use std::path::Path;

use log::Logger;
use proto::{udp::UdpPacket, Packet};

const DEFAULT_PAYLOAD: &str = "hello world";

fn main() {
    // Setup logger
    let mut log = Logger::new();
    log.log_to_file(Path::new("./logs/log.log"));
    log.time();
    log.info("Start traffic generator client".to_string());

    // Define default payload for debugging
    let payload = DEFAULT_PAYLOAD.as_bytes().to_vec();
    println!("Packet payload: {:x?}", payload);

    // Create packet
    let mut pkt = UdpPacket::new(0x7842, 0x8070);
    pkt.set_payload(payload);
    pkt.update_length();
    let pkt_vec = pkt.to_vec();
    println!("Entire packet: {:x?}", pkt_vec);
}
