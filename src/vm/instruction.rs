use std::fmt::Debug;

use num_enum::UnsafeFromPrimitive;

#[derive(Clone, Copy, Debug, UnsafeFromPrimitive)]
#[repr(u8)]
pub enum Opcode {
    Move,

    Add,
    Subtract,
    Multiply,
    Divide,

    Compare,
    Jump,
    JumpIfEqual,
    JumpIfLessThan,

    // Debugging
    Print
}
