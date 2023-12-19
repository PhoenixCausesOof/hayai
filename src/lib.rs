#![deny(clippy::missing_safety_doc)]

/// The type used for encoding addresses in the VM.
/// 
/// It is not a `usize` so that operands are more compact.
type AddressType = u32;

pub mod vm;