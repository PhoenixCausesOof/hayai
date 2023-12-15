use criterion::{criterion_group, criterion_main, Criterion};
use hayai::vm::{instruction::Opcode, metadata::*, VM};
use std::{hint::black_box, time::Duration};

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("my-benchmark");
    group
        .significance_level(0.1)
        .sample_size(1000)
        .warm_up_time(Duration::from_secs(7));

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

    // Fibonacci up until 255 in bytecode
    let vec = byte_vec!(
        Opcode::Move, Metadata::new(Pod::U32, Some(Opkind::Immediate)), 0u32, 0u32,         // x = 0
        Opcode::Move, Metadata::new(Pod::U32, Some(Opkind::Immediate)), 4u32, 1u32,         // y = 1
        Opcode::Move, Metadata::new(Pod::U32, Some(Opkind::Memory)), 8u32, 0u32,            // z = x
        Opcode::Add, Metadata::new(Pod::U32, Some(Opkind::Memory)), 8u32, 4u32,             // z += y (z = x + y)
        Opcode::Move, Metadata::new(Pod::U32, Some(Opkind::Memory)), 0u32, 4u32,            // x = y 
        Opcode::Move, Metadata::new(Pod::U32, Some(Opkind::Memory)), 4u32, 8u32,            // y = z 
        Opcode::Compare, Metadata::new(Pod::U32, Some(Opkind::Immediate)), 0u32, 255u32,    // jmp 0
        Opcode::JumpIfLessThan, 20usize
    );

    #[inline]
    fn fibonacci(floor: u32) -> u32 {
        let mut x = 0;
        let mut y = 1;
        let mut z = 0;

        while {
            {
                z = x + y;
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

    group.bench_function("rust", |b| b.iter(|| fibonacci(black_box(255))));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
