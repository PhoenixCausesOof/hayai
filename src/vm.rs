pub mod instruction;
pub mod metadata;
mod pod;
mod stack;
use crate::AddressType;

use std::{
    cmp::{self, Ordering},
    fmt::Debug,
    mem, ops,
};

use self::{instruction::Opcode, metadata::Metadata, pod::Pod, stack::Stack};

#[derive(Debug)]
pub struct VM<'a, const STACK_SIZE: usize> {
    program: &'a [u8],
    counter: usize,
    stack: Stack<STACK_SIZE>,
    comparison: Option<Ordering>,
}

impl<'a, const STACK_SIZE: usize> VM<'a, STACK_SIZE> {
    #[inline]
    pub fn new(program: &'a [u8]) -> Self {
        Self {
            program,
            counter: 0,
            stack: Stack::new(),
            comparison: None,
        }
    }

    /// Retrieves a value of type `T` from the bytecode and advances the program counter.
    ///
    /// # Safety
    ///
    /// A valid `T` must exist in the range `self.counter`..`self.counter + size_of::<T>()` of the stored bytecode.
    #[inline]
    pub(crate) unsafe fn fetch<T>(&mut self) -> T {
        let counter = self.counter + mem::size_of::<T>();
        debug_assert!(
            counter <= self.program.len(),
            "Not enough space for Program::fetch (attempted out of bounds access) with region {}..{}",
            self.counter, counter
        );
        self.program
            .as_ptr()
            .add(mem::replace(&mut self.counter, counter))
            .cast::<T>()
            .read_unaligned()
    }

    /// Executes the next instruction in the program.
    #[inline]
    unsafe fn execute(&mut self) {
        macro_rules! match_pod {
            ($pod:expr, $code:block) => {
                match $pod {
                    metadata::Pod::U8 => {
                        type T = u8;
                        $code
                    }
                    metadata::Pod::U16 => {
                        type T = u16;
                        $code
                    }
                    metadata::Pod::U32 => {
                        type T = u32;
                        $code
                    }
                    metadata::Pod::U64 => {
                        type T = u64;
                        $code
                    }
                    metadata::Pod::F32 => {
                        type T = f32;
                        $code
                    }
                    metadata::Pod::F64 => {
                        type T = f64;
                        $code
                    }
                }
            };
        }

        /// First block is for immediate operands
        ///
        /// Second block is for memory operands
        macro_rules! match_opkind {
            ($opkind:expr, $code0:expr, $code1:expr) => {
                match $opkind {
                    metadata::AddressingMode::Immediate => $code0,
                    metadata::AddressingMode::Absolute => $code1,
                }
            };
        }

        macro_rules! fetch_many {
            ($($ty:ty),+) => {
                ($(self.fetch::<$ty>()),+)
            };
        }

        macro_rules! handle_instruction {
            (($($var0:ident:$ty0:ty),+) $code0:block, ($($var1:ident:$ty1:ty),+) $code1:block) => {{
                let (pod, opkind) = {
                    let metadata: Metadata = self.fetch();
                    (metadata.pod(), metadata.addressing_mode())
                };

                match_opkind!(opkind, match_pod!(pod, {
                    let ($($var0),+) = fetch_many!($($ty0),+);
                    $code0
                }), match_pod!(pod, {
                    let ($($var1),+) = fetch_many!($($ty1),+);
                    $code1
                }));
            }};
        }

        let opcode: Opcode = self.fetch();

        match opcode {
            Opcode::Move => handle_instruction!(
                (dst: AddressType, imm: T)
                {
                    self.move_imm(dst, imm)
                },
                (dst: AddressType, src: AddressType)
                {
                    self.r#move::<T>(dst, src);
                }
            ),
            Opcode::Add => handle_instruction!(
                (dst: AddressType, imm: T)
                {
                    self.add_imm(dst, imm)
                },
                (dst: AddressType, src: AddressType)
                {
                    self.add::<T>(dst, src);
                }
            ),
            Opcode::Subtract => handle_instruction!(
                (dst: AddressType, imm: T)
                {
                    self.sub_imm(dst, imm)
                },
                (dst: AddressType, src: AddressType)
                {
                    self.sub::<T>(dst, src);
                }
            ),
            Opcode::Multiply => handle_instruction!(
                (dst: AddressType, imm: T)
                {
                    self.mul_imm(dst, imm)
                },
                (dst: AddressType, src: AddressType)
                {
                    self.mul::<T>(dst, src);
                }
            ),
            Opcode::Divide => handle_instruction!(
                (dst: AddressType, imm: T)
                {
                    self.div_imm(dst, imm)
                },
                (dst: AddressType, src: AddressType)
                {
                    self.div::<T>(dst, src);
                }
            ),
            Opcode::Compare => {
                handle_instruction!(
                    (dst: AddressType, imm: T)
                    {
                        self.cmp_imm(dst, imm)
                    },
                    (dst: AddressType, src: AddressType)
                    {
                        self.cmp::<T>(dst, src);
                    }
                )
            }
            Opcode::Jump => {
                let offset = self.fetch::<usize>();
                self.jump(offset);
            }
            Opcode::JumpIfLessThan => {
                let offset = self.fetch::<usize>();
                self.jump_if_lt(offset);
            }

            Opcode::Print => {
                let pod = self.fetch::<Metadata>().pod();
                let dst = self.fetch::<AddressType>();

                match_pod!(pod, {
                    self.print::<T>(dst);
                })
            }
        }
    }

    /// Runs all the stored instructions in the bytecode
    ///
    /// # Safety
    ///
    /// Bytecode must be valid for interpreting:
    /// * No invalid instructions
    /// * No invalid operands
    pub unsafe fn interpret(&mut self) {
        while self.counter < self.program.len() {
            self.execute();
        }
    }

    #[inline(always)]
    pub fn move_imm<T: Pod>(&mut self, dst: AddressType, imm: T) {
        unsafe { self.stack.write_at(dst as usize, imm) };
    }

    #[inline(always)]
    pub fn r#move<T: Pod>(&mut self, dst: AddressType, src: AddressType) {
        self.move_imm(dst, unsafe { self.stack.read_at::<T>(src as usize) });
    }

    #[inline(always)]
    pub fn add_imm<T: Pod + ops::Add<Output = T>>(&mut self, dst: AddressType, imm: T) {
        let val: T = unsafe { self.stack.read_at(dst as usize) };
        self.move_imm(dst, val + imm);
    }

    #[inline(always)]
    pub fn add<T: Pod + ops::Add<Output = T>>(&mut self, dst: AddressType, src: AddressType) {
        let val: T = unsafe { self.stack.read_at(src as usize) };
        self.add_imm(dst, val)
    }

    #[inline(always)]
    pub fn sub_imm<T: Pod + ops::Sub<Output = T>>(&mut self, dst: AddressType, imm: T) {
        let val: T = unsafe { self.stack.read_at(dst as usize) };
        self.move_imm(dst, val - imm);
    }

    #[inline(always)]
    pub fn sub<T: Pod + ops::Sub<Output = T>>(&mut self, dst: AddressType, src: AddressType) {
        let val: T = unsafe { self.stack.read_at(src as usize) };
        self.sub_imm(dst, val)
    }

    #[inline(always)]
    pub fn mul_imm<T: Pod + ops::Mul<Output = T>>(&mut self, dst: AddressType, imm: T) {
        let val: T = unsafe { self.stack.read_at(dst as usize) };
        self.move_imm(dst, val * imm);
    }

    #[inline(always)]
    pub fn mul<T: Pod + ops::Mul<Output = T>>(&mut self, dst: AddressType, src: AddressType) {
        let val: T = unsafe { self.stack.read_at(src as usize) };
        self.mul_imm(dst, val)
    }

    #[inline(always)]
    pub fn div_imm<T: Pod + ops::Div<Output = T>>(&mut self, dst: AddressType, imm: T) {
        let val: T = unsafe { self.stack.read_at(dst as usize) };
        self.move_imm(dst, val / imm);
    }

    #[inline(always)]
    pub fn div<T: Pod + ops::Div<Output = T>>(&mut self, dst: AddressType, src: AddressType) {
        let val: T = unsafe { self.stack.read_at(src as usize) };
        self.div_imm(dst, val)
    }

    #[inline(always)]
    pub fn cmp_imm<T: Pod + cmp::PartialOrd>(&mut self, dst: AddressType, imm: T) {
        let val: T = unsafe { self.stack.read_at(dst as usize) };
        self.comparison = val.partial_cmp(&imm);
    }

    #[inline(always)]
    pub fn cmp<T: Pod + cmp::PartialOrd>(&mut self, dst: AddressType, src: AddressType) {
        let val: T = unsafe { self.stack.read_at(src as usize) };
        self.cmp_imm(dst, val)
    }

    #[inline(always)]
    pub fn jump(&mut self, dst: usize) {
        self.counter = dst;
    }

    #[inline(always)]
    pub fn jump_if_lt(&mut self, dst: usize) {
        if self.comparison == Some(Ordering::Less) {
            self.jump(dst);
        }
    }

    #[inline(always)]
    pub fn print<T: Pod + Debug>(&mut self, dst: AddressType) {
        let val: T = unsafe { self.stack.read_at(dst as usize) };
        println!("{:?}", val);
    }
}
