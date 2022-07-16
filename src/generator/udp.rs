//! # UDP
//!
//! Packet definitions for the UDP protocol ([`RFC 768`](https://www.rfc-editor.org/rfc/rfc768.html))
//!
//! ## Header
//!
//! ```
//! 0      7 8     15 16    23 24    31
//! +--------+--------+--------+--------+
//! |     Source      |   Destination   |
//! |      Port       |      Port       |
//! +--------+--------+--------+--------+
//! |                 |                 |
//! |     Length      |    Checksum     |
//! +--------+--------+--------+--------+
//! ```
//!

use super::Packet;

const UDP_HEADER_LEN: u16 = 8;
const UDP_PROTO_NR: u16 = 0x11;

/// Holds an UDP packet
pub struct UdpPacket {
    src_port: u16,
    dst_port: u16,
    len: u16,
    checksum: u16,
    payload: Option<Vec<u8>>,
}

// Implement Packet trait for UdpPacket so it can be used as a Packet by higher modules
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

// Implement a handful helpful functions for UDP packets
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

    /// Calculate and update checksum
    pub fn update_checksum(&mut self, source_address: u32, destination_address: u32) {
        // Source: https://www.rfc-editor.org/rfc/rfc768.html
        // Source: https://www.rfc-editor.org/rfc/rfc1071.html

        // Set current checksum to 0 so it won't influence the calculation
        self.checksum = 0x0;

        // Needed for pseudo header
        let src_b = source_address.to_be_bytes();
        let dst_b = destination_address.to_be_bytes();
        let proto: u8 = UDP_PROTO_NR.try_into().unwrap();
        let len_b = self.len.to_be_bytes();

        // Create pseudo pkt vector
        let mut pseudo_pkt = Vec::new();
        pseudo_pkt.extend_from_slice(&src_b);
        pseudo_pkt.extend_from_slice(&dst_b);
        pseudo_pkt.push(0x0);
        pseudo_pkt.push(proto);
        pseudo_pkt.extend_from_slice(&len_b);

        // Add packet to pseudo packet
        let mut pkt = self.to_vec();
        pseudo_pkt.append(&mut pkt);

        // Add padding if pseudo packet size is not a multiple of 2 bytes
        let ps_pkt_len = pseudo_pkt.len();
        if ps_pkt_len % 2 != 0 {
            pseudo_pkt.push(0);
        }

        // Sum all words of pseudo packet together
        let mut sum = 0; // Note: u32 is large enough since max value is 0xFFF60009 = 0xffff * 65527 (UDP length is max 65 527 for IPv6 and 65 507 for IPv4)
        for i in (0..ps_pkt_len).step_by(2) {
            let word = ((pseudo_pkt[i] as u32) << 8) + (pseudo_pkt[i + 1] as u32);
            sum += word;
        }

        // Add upper and lower word together as long as the result is bigger than 2 bytes
        while sum > 0xffff {
            let sum_b = sum.to_be_bytes();
            sum = ((sum_b[0] as u32) << 8)
                + (sum_b[1] as u32)
                + ((sum_b[2] as u32) << 8)
                + (sum_b[3] as u32);
        }

        //
        let checksum = !sum as u16;

        // Update checksum
        self.checksum = checksum
    }

    /// Update length field
    pub fn update_length(&mut self) {
        self.len = self.get_length();
    }

    /// Get length of udp packet
    fn get_length(&self) -> u16 {
        let payload_len = match &self.payload {
            Some(payload) => payload.len().try_into().unwrap(),
            None => 0,
        };
        UDP_HEADER_LEN + payload_len
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn update_checksum_even_len() {
        // Source: https://people.engr.ncsu.edu/mlsichit/Teaching/407/Resources/udpChecksum.html
        let payload: Vec<u8> = vec![0x62, 0x62];
        let src_addr: u32 = 0x9801331b;
        let dst_addr: u32 = 0x980e5e4b;
        let src_port: u16 = 0xa08f;
        let dst_port: u16 = 0x2694;

        let mut pkt = UdpPacket::new(src_port, dst_port);
        pkt.set_payload(payload);
        pkt.update_length();
        pkt.update_checksum(src_addr, dst_addr);

        assert_eq!(pkt.checksum, 0x14de);
    }

    #[test]
    fn update_checksum_odd_len() {
        // Source: https://people.engr.ncsu.edu/mlsichit/Teaching/407/Resources/udpChecksum.html
        let payload: Vec<u8> = vec![0x62, 0x62, 0x62];
        let src_addr: u32 = 0x0a19876c;
        let dst_addr: u32 = 0xac409252;
        let src_port: u16 = 0xa08f;
        let dst_port: u16 = 0x2694;

        let mut pkt = UdpPacket::new(src_port, dst_port);
        pkt.set_payload(payload);
        pkt.update_length();
        pkt.update_checksum(src_addr, dst_addr);

        assert_eq!(pkt.checksum, 0xa439);
    }

    #[test]
    fn update_checksum_empty_payload() {
        // Source: https://people.engr.ncsu.edu/mlsichit/Teaching/407/Resources/udpChecksum.html
        let src_addr: u32 = 0x0a19876c;
        let dst_addr: u32 = 0xac409252;
        let src_port: u16 = 0xa08f;
        let dst_port: u16 = 0x2694;

        let mut pkt = UdpPacket::new(src_port, dst_port);
        pkt.update_length();
        pkt.update_checksum(src_addr, dst_addr);

        assert_eq!(pkt.checksum, 0x68a2);
    }
}
