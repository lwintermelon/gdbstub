//! Handle GDB Remote Serial Protocol packets that `gdbstub` does not
//! recognize.
//!
//! This extension exists so that targets can implement stub-specific or
//! debugger-specific packets (e.g. Delve/rr's `qRRCmd`) while still letting
//! `gdbstub` own all the standard protocol machinery (framing, checksums,
//! acks, standard packet dispatch).

pub use crate::protocol::ConsoleOutput;

use crate::target::Target;

/// Target Extension - Handle unrecognized packets.
pub trait UnknownPacket: Target {
    /// Handle a packet that `gdbstub` could not parse.
    ///
    /// `body` is the raw packet body (the bytes between `$` and `#`).
    ///
    /// The response body should be written, raw (unencoded), to `out` using
    /// [`ConsoleOutput::write_raw`]. Writing nothing produces an empty
    /// response, which GDB treats as "unsupported".
    fn handle_unknown_packet(
        &mut self,
        body: &[u8],
        out: ConsoleOutput<'_>,
    ) -> Result<(), Self::Error>;
}

define_ext!(UnknownPacketOps, UnknownPacket);
