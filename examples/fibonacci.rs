use hayai::vm::{VM, instruction::Opcode, metadata::*};

fn main() {
    unsafe fn bytes_of<T>(val: &T) -> &[u8] {
        std::slice::from_raw_parts(val as *const T as *const u8, std::mem::size_of::<T>())
    }

    macro_rules! byte_vec {
        ($($val:expr),*) => {
            {
                let mut vec = Vec::new();
                $(vec.extend_from_slice(unsafe {bytes_of(&$val)});)*
                vec
            }
        };
    }

    /* Equivalent pseudo-code
    x = 0
    y = 1
    do {
        print(x)

        z = x + y;
        x = y;
        y = z;
    } while (x < 255);
    */

    // Fibonacci in bytecode
    let vec = byte_vec!(
        /* mov imm u32 0x0, 0x0 */      Opcode::Move, Metadata::new(Pod::U32, Some(AddressingMode::Immediate)), 0u32, 0u32,
        /* mov imm u32 0x4, 0x1 */      Opcode::Move, Metadata::new(Pod::U32, Some(AddressingMode::Immediate)), 4u32, 1u32,
        /* print u32 0x0 */             Opcode::Print, Metadata::new(Pod::U32, None), 0u32,
        /* mov abs u32 0x8, 0x0 */      Opcode::Move, Metadata::new(Pod::U32, Some(AddressingMode::Absolute)), 8u32, 0u32,
        /* add abs u32 0x8, 0x4 */      Opcode::Add, Metadata::new(Pod::U32, Some(AddressingMode::Absolute)), 8u32, 4u32,
        /* mov abs u32 0x0, 0x4 */      Opcode::Move, Metadata::new(Pod::U32, Some(AddressingMode::Absolute)), 0u32, 4u32,
        /* mov abs u32 0x4, 0x8 */      Opcode::Move, Metadata::new(Pod::U32, Some(AddressingMode::Absolute)), 4u32, 8u32,
        /* cmp imm u32 0x0, 0xff */     Opcode::Compare, Metadata::new(Pod::U32, Some(AddressingMode::Immediate)), 0u32, 255u32,
        /* jl 0x14 */                   Opcode::JumpIfLessThan, 20usize
    );


    let mut vm: VM<12> = VM::new(&vec);

    unsafe { vm.interpret() };
}
