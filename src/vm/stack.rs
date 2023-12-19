use aligned::{Aligned, A16};
use std::mem;

use super::pod::Pod;

/// A stack for the VM. Allows for reading / writing [`Pod`]s to any valid positions.
/// 
/// The stack is aligned so that `std::mem::read_unaligned` and `std::mem::write_unaligned` can
/// be optimized to aligned reads / writes.
#[derive(Debug)]
pub(super) struct Stack<const STACK_SIZE: usize>([Aligned<A16, u8>; STACK_SIZE]);

impl<const STACK_SIZE: usize> Stack<STACK_SIZE> {
    pub(super) fn new() -> Self {
        Self([Aligned(0); STACK_SIZE])
    }

    #[inline(always)]
    pub(super) unsafe fn read_at<T: Pod>(&self, offset: usize) -> T {
        debug_assert!(
            offset + mem::size_of::<T>() <= STACK_SIZE,
            "Attempt to read from stack at positions {}..{} with STACK_SIZE = {}",
            offset, offset + mem::size_of::<T>(), STACK_SIZE
        );
        self.0.as_ptr().add(offset).cast::<T>().read_unaligned()
    }

    #[inline(always)]
    pub(super) unsafe fn write_at<T: Pod>(&mut self, offset: usize, val: T) {
        debug_assert!(
            offset + mem::size_of::<T>() <= STACK_SIZE,
            "Attempt to write at stack at positions {}..{} with STACK_SIZE = {}",
            offset, offset + mem::size_of::<T>(), STACK_SIZE
        );
        self.0.as_mut_ptr().add(offset).cast::<T>().write_unaligned(val)
    }
}

impl<const STACK_SIZE: usize> Default for Stack<STACK_SIZE> {
    fn default() -> Self {
        Self::new()
    }
}
