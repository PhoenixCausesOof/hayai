use std::fmt::Debug;
use num_enum::UnsafeFromPrimitive;

#[derive(Clone, Copy, Debug, PartialEq, Eq, UnsafeFromPrimitive)]
#[repr(u8)]
pub enum Pod {
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
}


#[derive(Clone, Copy, Debug, PartialEq, Eq, UnsafeFromPrimitive)]
#[repr(u8)]
pub enum AddressingMode {
    Immediate,
    Absolute,
}


/// Compactly stores operand data for their respective instructions.
#[derive(Clone, Copy)]
pub struct Metadata(u8);

impl Metadata {
    #[inline]
    pub fn new(pod: Pod, opkind: Option<AddressingMode>) -> Self {
        match opkind {
            Some(opkind) => Self((pod as u8) << 5 | (opkind as u8) << 4),
            None => Self((pod as u8) << 5),
        }
    }

    #[inline]
    pub fn pod(&self) -> Pod {
        unsafe { Pod::unchecked_transmute_from(self.0 >> 5) }
    }

    #[inline]
    pub fn addressing_mode(&self) -> AddressingMode {
        const MASK: u8 = 1 << 4;

        unsafe { AddressingMode::unchecked_transmute_from((self.0 & MASK) >> 4) }
    }
}

impl Debug for Metadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Metadata")
            .field("pod", &self.pod())
            .field("addressing_mode", &self.addressing_mode())
            .finish()
    }
}
