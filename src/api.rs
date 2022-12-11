//! API module for the `morph-rs` crate.

use std::fmt;

/// Enum of different variants of the Morph Earbuds.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum EarbudsKind {
    /// V1 is the first iteration of the Morph BLE earbuds.
    V1,
    /// Undefined earbuds - default type.
    Undefined,
}

impl EarbudsKind {
    fn full_name(&self) -> &'static str {
        match *self {
            EarbudsKind::V1 => "Morph InfiniConnect v1",
            EarbudsKind::Undefined => "Undefined model (default variant).",
        }
    }
}

impl Default for EarbudsKind {
    fn default() -> Self {
        Self::Undefined
    }
}

impl fmt::Display for EarbudsKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.full_name())
    }
}

pub mod features {

    //! Exports an enum of Morph Earbuds commands.

    /// This is an enum of commands for the earbuds.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum FeatureKind {
        /// Feature that can be sent to the earbuds, to check that the command handler is
        /// running.
        /// Payload received should be 0x01.
        GetMorphFeatureHandlerVersion = 0x00,
        /// Feature that can be sent to the earbuds, to check the volume.
        /// Volume received from this command is payload 0x30, or 48 in decimal.
        /// (Possible range of volume is 0-0x7f, or 0-127 in decimal)
        GetVolume = 0x01,
        /// Default value.
        Undefined,
    }

    impl Default for FeatureKind {
        fn default() -> Self {
            Self::Undefined
        }
    }
}

/// Enum of BLE Protocol Data Unit types.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PduKind {
    /// Command PDU.
    Command = 0x0,
    /// Notification PDU.
    Notification = 0x1,
    /// Response PDU.
    Response = 0x10,
    /// Error PDU.
    Error = 0x11,
    /// Undefined PDU type.
    Undefined,
}

impl Default for PduKind {
    fn default() -> Self {
        Self::Undefined
    }
}
