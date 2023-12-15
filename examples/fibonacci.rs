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

    // Fibonacci in bytecode
    let vec = byte_vec!(
        Opcode::Move, Metadata::new(Pod::U32, Some(Opkind::Immediate)), 0u32, 0u32,       // x = 0
        Opcode::Move, Metadata::new(Pod::U32, Some(Opkind::Immediate)), 4u32, 1u32,       // y = 1
        Opcode::Print, Metadata::new(Pod::U32, None), 0u32,                               // print(x)
        Opcode::Move, Metadata::new(Pod::U32, Some(Opkind::Memory)), 8u32, 0u32,          // z = x
        Opcode::Add, Metadata::new(Pod::U32, Some(Opkind::Memory)), 8u32, 4u32,           // z += y (z = x + y)
        Opcode::Move, Metadata::new(Pod::U32, Some(Opkind::Memory)), 0u32, 4u32,          // x = y
        Opcode::Move, Metadata::new(Pod::U32, Some(Opkind::Memory)), 4u32, 8u32,          // y = z
        Opcode::Compare, Metadata::new(Pod::U32, Some(Opkind::Immediate)), 0u32, 1000000000u32,  // while (x < 255)
        Opcode::JumpIfLessThan, 20usize
    );

    let mut vm: VM<12> = VM::new(&vec);

    unsafe { vm.interpret() };
}
