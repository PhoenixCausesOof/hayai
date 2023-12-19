use std::fmt::Debug;

use num_enum::UnsafeFromPrimitive;

/// VM opcodes
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
