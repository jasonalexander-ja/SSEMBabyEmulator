# Manchester Small-Scale Experimental Machine "Baby" Emulator Library

[![crates.io](https://img.shields.io/crates/v/baby-emulator)](https://crates.io/crates/baby-emulator)
[![Released API docs](https://docs.rs/baby-emulator/badge.svg)](https://docs.rs/baby-emulator)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

This library provides a collections of types and methods for emulating 
the [Machester Baby](https://www.scienceandindustrymuseum.org.uk/objects-and-stories/baby-and-modern-computing) the first program stored 
computer. 

## Explaination

The Manchester "Baby" was the first computer to store both its program
code and data in a common randomly-accessible memory, it is for this 
reason the Baby is considered the first machine to run "true" software, 
providing a familiar (abeit, primitive) programming environment to anyone 
familiar with assembly, this library can be included  in a variety of 
software and platforms allowing emulation functionality of this historic machine. 

## Installation 

Command line:
```
cargo add baby-emulator
```

Cargo.toml:
```
baby-emulator = "0.1.2"
```

## Example 

The core of this library is `core::BabyModel`, this struct has 
fields representing all of the Baby's internal registers and 
32 word memory, you can initialise this struct with an array of 
`[i32; 32]`, this array can contain the program code instructions 
starting at position 0. 

```rust
use baby_emulator::core::BabyModel;
use baby_emulator::errors::{BabyError, BabyErrors};


fn main() {
    let model = BabyModel::new_example_program();
    let mut last_model = BabyModel::new();
    let mut result = model.execute();
    while let Ok(new_model) = result {
        last_model = new_model.clone();
        result = new_model.execute();
    }
    match result {
        Err(BabyErrors::Stop(_)) => println!("{}", last_model.accumulator),
        _ => println!("Something went wrong. ")
    }
}
```

## Useage 

### Programming

The Baby accepts a 16 bit instructions, of which the upper 3 
bits denotes one of 7 instructions, each instruction has a helper
in `core::instructions::BabyInstructions` enum:

| Binary   | Instruction Enum   | Description                                                                                                                                                    |
|----------|--------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------|
| 000      | Jump               | Jump to the instruction at the address obtained from the specified memory address S[a] (absolute unconditional jump)                                     |
| 100      | RelativeJump       | Jump to the instruction at the program counter plus (+) the relative value obtained from the specified memory address S[a] (relative unconditional jump) |
| 010      | Negate             | Take the number from the specified memory address S, negate it, and load it into the accumulator                                                         |
| 110      | Store              | Store the number in the accumulator to the specified memory address S                                                                                    |
| 001\|101 | Subtract           | Subtract the number at the specified memory address S from the value in accumulator, and store the result in the accumulator                             |
| 011      | SkipNextIfNegative | Skip next instruction if the accumulator contains a negative value                                                                                       |
| 111      | Stop               | Stop                                                                                                                                                     |

The remaining lower bits is the the operand, the operand is 
a memory address for all but the `SkipIfNegative` and `Stop`
instructions which do not use operands. 

You can generate a program by making a vec of tuples containing 
a `BabyInstructions` enum and `u16` operand, and then passing this off
to `core::instructions::BabyInstructions::to_numbers` function,
this will return a `[i32; 32]` array which can be used to initialise 
a `BabyModel` ready for execution: 

```rust
use baby_emulator::core::{BabyModel, instructions::BabyInstruction};
use baby_emulator::errors::{BabyError, BabyErrors};

let instrs = vec![
    (BabyInstruction::Negate, 5),
    (BabyInstruction::Subtract, 5),
    (BabyInstruction::Store, 6),
    (BabyInstruction::Negate, 6),
    (BabyInstruction::Stop, 0)
];
let mut main_store = BabyInstruction::to_numbers(instrs);
main_store[5] = 5; // Initialise with data. 

let model = BabyModel::new_with_program(main_store);
```

### Running 

Once your model has been initalised, the one method you will need to
use is `BabyModel::execute`, this method will look at the current 
instruction, and will perform it, in the process modifying all the 
values held within the model.

If no issue is found then `BabyModel::execute` will return an `Ok`
with a new model containing all the updated values in the registers
and memories, and with the `BabyModel`.`instruction` set to the next
instruction ready to be executed and `BodyModel`.`instruction_address`
set to the memory address of that instruction. 

If an issue is found then `BabyModel::execute` will return an `Err`
containing an instance of `errors::BabyErrors` enum with which 
error it is, and containing an inner derivative of `errors::BabyError`
that holds the metadata on that error. 

An error can simply be that a `Stop` command has been hit, and 
there is nothing else more to execute, so the calling code should
handle that. 


To carry on from the above example: 
```rust
// We will store the state of the model when the last instruction is executed for debug purposes 
let mut last_model = BabyModel::new();

// We now just keep calling model.execute() until it's not `Ok`
let mut result = model.execute();
while let Ok(new_model) = result {
    last_model = new_model.clone();
    result = new_model.execute();
}

// Print out the result
match result {
    // If the program ran sucessfully, it would have encountered a stop 
    Err(BabyErrors::Stop(_)) => println!("{}", last_model.accumulator),
    _ => println!("Something went wrong. ")
}
```
