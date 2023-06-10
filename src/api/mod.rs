//! This module exports the main `morph-rs` types.

mod enums;
pub use enums::*;

#[derive(Debug, Clone)]
pub struct Command {
    feature: FeatureKind,
    data: Vec<u8>,
}
