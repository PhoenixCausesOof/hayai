//! Module containing constructs relevant to bytecode encoding and decoding.
//! Such as [`metadata::Metadata`] and [`Opcode`]

pub mod metadata;

use std::fmt::Debug;

use num_enum::UnsafeFromPrimitive;

/// Opcodes for the VM. Basically the entire instruction set as a `u8` byte.
#[derive(Clone, Copy, Debug, UnsafeFromPrimitive)]
#[repr(u8)]
pub enum Opcode {
    // Memory
    Move,

    // Arithmetic
    Add,
    Subtract,
    Multiply,
    Divide,

    // Control flow
    Compare,
    Jump,
    JumpIfLessThan,

    // Debugging
    Print
}
