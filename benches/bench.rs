use criterion::{criterion_group, criterion_main, Criterion};
use hayai::vm::{instruction::{Opcode, metadata::*}, VM};
use std::{hint::black_box, time::Duration};

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("my-benchmark");
    group
        .significance_level(0.1)
        .sample_size(1000)
        .warm_up_time(Duration::from_secs(7));

    // Helper function and macro
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


    #[inline]
    fn fibonacci(floor: u32) -> u32 {
        let mut x = 0;
        let mut y = 1;

        // Equivalent to a do-while loop
        while {
            {
                let z = x + y;
                x = y;
                y = z;
            };
            x < floor
        } {}

        x
    }

    group.bench_function("hayai", |b| {
        b.iter(|| {
            let mut vm: VM<12> = VM::new(black_box(&vec));

            unsafe { vm.interpret() };
        })
    });

    group.bench_function("native", |b| b.iter(|| fibonacci(black_box(255))));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
