# Hayai

## Description

Hayai is the fastest implementaton of an interpreted language that I could make.

It is currently in its first stage and only works as a standalone VM.

## Features

* Statically-typed
* Register machine
* Aligned stack

## Examples

See the `/examples` folder.

## Implementation

I tried to create a VM that worked as baremetal as possible. As such, some implementation details:

* Compact bytecode (no enums)
* Stack
* * Unaligned reading/writing
* * Support for all types with `Pod` trait
* Lots of `unsafe` (though the implementation isn't necessarily *unsafe*)

## Possible optimizations

I say this is the most efficient implementation I could've made, but that is a lie.

There is potential for speed-ups. It's just that they doesn't seem worthy enough for me to 
sacrifice my code's readability.

But here are some I could've potentially made (and might make):

* Combine `Opcode` and `Metadata` into a single enum
* * Requires proc-macros, which are a pain to me.