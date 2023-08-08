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
    feature: FeatureKind,
    data: Vec<u8>,
}
