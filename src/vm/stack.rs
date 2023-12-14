use std::mem;

use super::pod::Pod;

#[derive(Debug)]
pub(super) struct Stack<const STACK_SIZE: usize>([u8; STACK_SIZE]);

impl<const STACK_SIZE: usize> Default for Stack<STACK_SIZE> {
    fn default() -> Self {
        Self([0; STACK_SIZE])
    }
}

impl<const STACK_SIZE: usize> Stack<STACK_SIZE> {
    #[inline]
    pub(super) unsafe fn read_at<T: Pod>(&self, offset: usize) -> T {
        debug_assert!(
            offset + mem::size_of::<T>() <= STACK_SIZE,
            "Attempt to read from stack at positions {}..{} with STACK_SIZE = {}",
            offset, offset + mem::size_of::<T>(), STACK_SIZE
        );
        self.0.as_ptr().add(offset).cast::<T>().read_unaligned()
    }

    #[inline]
    pub(super) unsafe fn write_at<T: Pod>(&mut self, offset: usize, val: T) {
        debug_assert!(
            offset + mem::size_of::<T>() <= STACK_SIZE,
            "Attempt to write at stack at positions {}..{} with STACK_SIZE = {}",
            offset, offset + mem::size_of::<T>(), STACK_SIZE
        );
        self.0.as_mut_ptr().add(offset).cast::<T>().write_unaligned(val)
    }
}
