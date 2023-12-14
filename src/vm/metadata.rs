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
pub enum Opkind {
    Immediate,
    Memory,
}


#[derive(Clone, Copy)]
pub struct Metadata(u8);

impl Metadata {
    #[inline(always)]
    pub fn new(pod: Pod, opkind: Option<Opkind>) -> Self {
        match opkind {
            Some(opkind) => Self((pod as u8) << 5 | (opkind as u8) << 4),
            None => Self((pod as u8) << 5),
        }
    }

    #[inline(always)]
    pub fn pod(&self) -> Pod {
        unsafe { Pod::unchecked_transmute_from(self.0 >> 5) }
    }

    #[inline(always)]
    pub fn opkind(&self) -> Opkind {
        const MASK: u8 = 1 << 4;

        unsafe { Opkind::unchecked_transmute_from((self.0 & MASK) >> 4) }
    }
}

impl Debug for Metadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Metadata")
            .field("pod", &self.pod())
            .field("opkind", &self.opkind())
            .finish()
    }
}
