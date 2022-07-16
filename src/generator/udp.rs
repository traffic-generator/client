//! Definitions for the UDP protocol (RFC 768)

use chrono::ParseResult;

use super::Packet;

/// Holds an UDP packet
pub struct UdpPacket {
    src_port: u16,
    dst_port: u16,
    len: u16,
    checksum: u16,
    payload: Option<Vec<u8>>,
}

impl UdpPacket {
    /// Create a new packet without payload
    pub fn new(source_port: u16, destination_port: u16) -> UdpPacket {
        let packet = UdpPacket {
            src_port: source_port,
            dst_port: destination_port,
            len: 0,
            checksum: 0,
            payload: None,
        };
        return packet;
    }

    /// Set source port
    pub fn set_source_port(&mut self, source_port: u16) {
        self.src_port = source_port;
    }

    /// Set source port
    pub fn set_destination_port(&mut self, destination_port: u16) {
        self.dst_port = destination_port;
    }

    /// Calculate and update checksum
    pub fn update_checksum(&mut self) {
        todo!()
    }

    /// Update length
    pub fn update_length(&mut self) {
        let payload_len = match &self.payload {
            Some(payload) => payload.len(),
            None => 0,
        };
        self.len = (8 + payload_len).try_into().unwrap();
    }
}

impl Packet for UdpPacket {
    fn to_vec(&self) -> Vec<u8> {
        let src = self.src_port.to_be_bytes();
        let dst = self.dst_port.to_be_bytes();
        let len = self.len.to_be_bytes();
        let chk = self.checksum.to_be_bytes();
        let mut pkt: Vec<u8> = [src, dst, chk, len].concat();
        match &self.payload {
            Some(p) => pkt.append(&mut p.clone()),
            None => {}
        };
        pkt
    }

    fn set_payload(&mut self, payload: Vec<u8>) {
        self.payload = Some(payload);
    }

    fn get_payload(&self) -> Option<Vec<u8>> {
        self.payload.clone()
    }
}
