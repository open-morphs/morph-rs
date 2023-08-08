//! This module exports the main `morph-rs` types.
#![allow(dead_code)]

mod enums;
pub use enums::*;

mod consts {
    use uuid::{uuid, Uuid};

    const MORPH_SERVICE_UUID: Uuid = uuid!("00001100-D102-11E1-9B23-00025B00A5A5");
    const MORPH_COMMAND_UUID: Uuid = uuid!("00001101-D102-11E1-9B23-00025B00A5A5");
    const MORPH_RESPONSE_UUID: Uuid = uuid!("00001102-D102-11E1-9B23-00025B00A5A5");
}

/// `Message` acts as the final representation of a BLE packet to the earbuds.
#[derive(Debug, Clone)]
pub struct Message {
    buds: EarbudsKind,
    vendor: VendorKind,
    pdu: PduKind,
    feature: FeatureKind,
    data: Vec<u8>,
}

/// `Payload` is a trait to be impl'd for the `Message` type.
pub trait Payload {
    /// Respond with the `Feature` id (hex).
    fn get_id(&self) -> u8;

    /// Return encoded data to send to earbuds.
    fn get_data(&self) -> Vec<u8> {
        vec![]
    }

    /// Return the `EarbudsKind` field.
    fn get_buds(&self) -> EarbudsKind {
        Default::default()
    }

    /// Return the `PduKind` field.
    fn get_vendor(&self) -> VendorKind {
        Default::default()
    }

    /// Return the `PduKind` field.
    fn get_pdu(&self) -> PduKind {
        Default::default()
    }

    /// Return the `FeatureKind` field.
    fn get_feature(&self) -> FeatureKind {
        Default::default()
    }

    /// Return `true` if the PDU is a Command.
    fn is_command(&self) -> bool {
        false
    }

    /// Return `true` if the PDU is a Notification.
    fn is_notification(&self) -> bool {
        false
    }

    /// Return `true` if the PDU is a Response.
    fn is_response(&self) -> bool {
        false
    }

    /// Return `true` if the PDU is an Error.
    fn is_error(&self) -> bool {
        false
    }
}
