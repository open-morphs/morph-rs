//! API module for the `morph-rs` crate.
#![allow(dead_code)] // temporary, avoid warnings

use std::fmt;

mod features;

pub use self::features::FeatureKind;

/// Enum of different variants of the Morph Earbuds.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub enum EarbudsKind {
    /// V1 is the first iteration of the Morph BLE earbuds.
    MorphEarbudsV1,
    /// Unselected model.
    #[default]
    Unselected,
}

impl EarbudsKind {
    fn full_name(&self) -> &'static str {
        match *self {
            EarbudsKind::MorphEarbudsV1 => "Morph InfiniConnect v1",
            EarbudsKind::Unselected => "Unselected model.", // default variant.
        }
    }
}

impl fmt::Display for EarbudsKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.full_name())
    }
}

/// Enum of BLE Protocol Data Unit types.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
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
    /// Unselected variant.
    #[default]
    Unselected,
}

/// Enum of BLE vendors.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub enum VendorKind {
    /// Default vendor ID for Morphs.
    #[default]
    Morph = 0x0a66,
    /// QCC placeholder.
    Qcc = 0x001d,
}
