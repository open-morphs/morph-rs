//! API module for the `morph-rs` crate.
#![allow(dead_code)] // temporary, avoid warnings

use std::fmt;

/// Enum of different variants of the Morph Earbuds.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub enum EarbudsKind {
    /// V1 is the first iteration of the Morph BLE earbuds.
    MorphEarbudsV1,
    /// Undefined earbuds model.
    Undefined,
}

impl EarbudsKind {
    fn full_name(&self) -> &'static str {
        match *self {
            EarbudsKind::MorphEarbudsV1 => "Morph InfiniConnect v1",
            EarbudsKind::Undefined => "Undefined model.", // default variant.
        }
    }
}

impl Default for EarbudsKind {
    fn default() -> Self {
        Self::Undefined
    }
}

impl fmt::Display for EarbudsKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.full_name())
    }
}

/// Enum of BLE Protocol Data Unit types.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub enum PduKind {
    /// Command PDU.
    Command = 0x0,
    /// Notification PDU.
    Notification = 0x1,
    /// Response PDU.
    Response = 0x10,
    /// Error PDU.
    Error = 0x11,
    /// Default variant.
    Undefined,
}

impl Default for PduKind {
    fn default() -> Self {
        Self::Undefined // Default variant.
    }
}
