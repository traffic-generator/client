//! # Sender
//!
//! Send out dat using different protocols
//!
//! Note: interesting senders
//! * Raw --> Send any raw byte vector as can be obtained by a generator
//! * Udp
//! * Tcp
//! * QUIC

use std::net::SocketAddr;

pub mod udp;

/// Send a byte vector
pub trait Send {
    /// Setup before multiple consecutive sends
    fn setup(&mut self);

    /// Transmit a byte vector
    fn send(&self, data: Vec<u8>);
}
