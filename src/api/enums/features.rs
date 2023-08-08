//! Exports an enum of Morph Earbuds commands.

/// This is an enum of commands for the earbuds.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum FeatureKind {
    /// Feature that can be sent to the earbuds, to check that the command handler is
    /// running.
    /// Payload received should be 0x01.
    GetMorphFeatureHandlerVersion = 0x00,
    /// Feature that can be sent to the earbuds, to check the volume.
    /// (Possible range of volume is 0-0x7f, or 0-127 in decimal)
    GetVolume = 0x01,
    /// Change volume.
    SetVolume = 0x02,
    /// Toggle microphone mute.
    ToggleMicMute = 0x03,
    /// Get microphone state.
    GetMicMuteState = 0x04,
    /// Switch earbuds into pairing mode.
    SetStartPairingMode = 0x05,
    /// Get connection status of headset.
    GetIsDeviceConnected = 0x06,
    /// Default variant.
    #[default]
    Unselected,
}
