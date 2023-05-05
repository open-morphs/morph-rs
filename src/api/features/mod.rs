//! Exports an enum of Morph Earbuds commands.

/// This is an enum of commands for the earbuds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum FeatureVariant {
    /// Feature that can be sent to the earbuds, to check that the command handler is
    /// running.
    /// Payload received should be 0x01.
    GetMorphFeatureHandlerVersion = 0x00,
    /// Feature that can be sent to the earbuds, to check the volume.
    /// Volume received from this command is payload 0x30, or 48 in decimal.
    /// (Possible range of volume is 0-0x7f, or 0-127 in decimal)
    GetVolume = 0x01,
    /// Default variant.
    Undefined,
}

impl Default for FeatureVariant {
    fn default() -> Self {
        Self::Undefined // Default variant.
    }
}
