//! This module exports the main `morph-rs` types.
#![allow(dead_code)]

mod enums;
pub use enums::*;

/// `Message` acts as the final representation of a BLE packet to the earbuds.
#[derive(Debug, Clone)]
pub struct Message {
    feature: FeatureKind,
    data: Vec<u8>,
}
