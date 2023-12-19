# Hayai

## Description

A fast, simple, statically-typed, register virtual machine.

## Types

* `u8`
* `u16`
* `u32`
* `u64`
* `f32`
* `f64`

## Features

* Statically-typed
* Register machine

## Examples

See the `/examples` folder.

## Implementation

* Compact instruction encoding
* * Opcodes are stored in a single byte
* * Metadata (addressing mode, type) encoded as a single byte
* Ignored strict-aliasing
* * Although, if certain conditions are met, reading from / writing to the stack can be optimized down to a pointer dereference by the compiler.

## Possible optimizations

* Encode opcode data and metadata in a single enum
* * Would avoid nested `match` expressions
* * Requires proc_macros (if striving for readability)
* Strict-aliasing (currently unaligned reads / writes)