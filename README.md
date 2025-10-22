# Manchester Small-Scale Experimental Machine "Baby" Emulator Library 

[![crates.io](https://img.shields.io/crates/v/baby-emulator)](https://crates.io/crates/baby-emulator)
[![Released API docs](https://docs.rs/baby-emulator/badge.svg)](https://docs.rs/baby-emulator)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENCE)

This library provides a collections of types and methods for emulating & assembling code for 
the [Manchester Baby](https://www.scienceandindustrymuseum.org.uk/objects-and-stories/baby-and-modern-computing), the first program stored 
computer. 

## Explaination 

The Manchester "Baby" was the first computer to store both its program 
code and data in a common randomly-accessible memory, it is for this 
reason the Baby is considered the first machine to run "true" software, 
providing a familiar (albeit primitive) programming environment to anyone 
familiar with assembly, this library can be included in a variety of 
software and platforms allowing emulation functionality of this historic machine. 

This library provides an interface for emulating the Baby as a bytecode 
interpreter (`baby_emulator::core`), and also a library for assembling 
asm using both modern and original asm notations into a format that 
can be ran by the emulator (`baby_emulator::assembler`). 

Please log any questions or issues to the [GitHub repo](https://github.com/jasonalexander-ja/SSEMBabyEmulator).

## Installation 

Command line: 
```bash 
cargo add baby-emulator 
```

Cargo.toml: 
```toml 
baby-emulator = "0.2.1" 
``` 

### Features 

As of version 2.1, variable bit lengths are supported on both the simulator and assembler, 
either `i32` as per the original Baby, `i16` and `i8`, available by importing the library 
the correspondingly named feature. 

```toml
baby-emulator = { version = "0.2.1", features = ["i32"] }
baby-emulator = { version = "0.2.1", features = ["i16"] }
baby-emulator = { version = "0.2.1", features = ["i8"] }
```

## Example 

This shows a few short examples of what this library is capable of, designed to be a 
starting point allowing further experimentation by the "user". See
the [docs](https://docs.rs/baby-emulator) for further examples and info. 

### Bytecode Interpreter Emulation

The core of this library is `baby_emulator::core::BabyModel`, 
this struct has fields representing all of the Baby's internal 
registers and 32 word memory, you can initialise this struct with 
an array of `[i32; 32]`, this array can contain the program code 
instructions starting at position 0. 

This example runs an example program that adds 5 to 5 and stores 
the result in the accumulator. Running here is done with the `run_loop`
method, this method will simply execute sucessive instructions until 
either an error is thrown (like a stop instruction), or the number
os iterations exceeds the specified limmit. 

```rust
use baby_emulator::core::BabyModel;
use baby_emulator::core::errors::BabyErrors;
use baby_emulator::core::errors::BabyError;

let model = BabyModel::new_example_program();
match model.run_loop(100) {
    (model, BabyErrors::Stop(_)) => println!("{}", model.core_dump()),
    (_, err) => println!("{}", err.get_descriptor())
}
```

You can also single step through an emulation, executing a single 
instruction at a time using the `execute` method and seeing the 
direct result. 

```rust
use baby_emulator::core::BabyModel;
use baby_emulator::core::errors::BabyError;

let model = BabyModel::new_example_program();
match model.execute() {
    Ok(m) => println!("{}", m.core_dump()),
    Err(e) => println!("Error {}", e.get_descriptor())
}
```

### Assembly

Here is an example of assembling a Baby asm string using 
modern notation, then running the resultant program;
see the `baby_emulator::assembler` docs for more information:

```rust
use baby_emulator::assembler::{assemble, linker::LinkerData}; 
use baby_emulator::core::{BabyModel, instructions::BabyInstruction};
 
 
const ASM: &str = 
"
ldn $start_value  ; Loads 10 into the accumulator 

:loop_start_value ; The memory address the loop should return to 
sub $subtract_val ; Subtract 1 from the accumulator 
cmp               ; Skip the next jump instruction if the accumulator is negative 
jmp $loop_start   ; Jump to the start of the loop 
stp               ; Program stops when the accumulator is negative 

:loop_start       ; Pointer to the memory address the loop should return to 
abs $loop_start_value

:subtract_val     ; Value to be subtracted
abs 0d1

:start_value      ; Value to start in the accumulator 
abs 0d-10
";

fn main() {
    let instructions = match assemble(&String::from(ASM), false) {
        Ok(LinkerData(v, _)) => v,
        Err(e) => { println!("{}", e.describe(true)); return; }
    };
    let main_store = BabyInstruction::to_numbers(instructions);
 
    let mut model = BabyModel::new_with_program(main_store);
    loop {
        model = match model.execute() {
            Ok(m) => m,
            Err(_) => break
        };
    }
    println!("{}", model.core_dump());
}
```
