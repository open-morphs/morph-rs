//! This module exports the main `morph-rs` types.
#![allow(dead_code)]

mod enums;
pub use enums::*;

#[derive(Debug, Clone)]
pub struct Command {
    feature: FeatureKind,
    data: Vec<u8>,
}
