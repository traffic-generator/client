// Load protocol definitions
pub mod udp;

/// # Packet trait
///
/// Methods common to all packets regardless the protocol.
pub trait Packet {
    /// Convert packet to byte vector
    fn to_vec(&self) -> Vec<u8>;

    /// Set a byte vector as a packet's payload
    fn set_payload(&mut self, payload: Vec<u8>);

    /// Get a packet's payload
    ///
    /// Returns `Some(payload)` if available and `None` otherwise
    fn get_payload(&self) -> Option<Vec<u8>>;
}
