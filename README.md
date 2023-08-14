# Manchester Small-Scale Experimental Machine "Baby" Emulator Library

This provides a collections of types and methods for emulating 
the [Machester Baby](https://www.scienceandindustrymuseum.org.uk/objects-and-stories/baby-and-modern-computing) the first program stored 
computer; this was the first machine to have RAM and thus the 
first to execute software, providing a familiar (abeit, primitive) 
programming environment to anyone familiar with modern assembly 
programming, this library can be included in a variety of software
and platforms allowing emulation functionality of this historic machine. 

The core of this library is `core::BabyModel`, this struct has 
fields representing all of the Baby's internal registers and 
32 word memory, you can initialise this struct with an array of 
`i32` (the Baby performed 2's compliment arithmetic) and a size 
of 32, this array can contain the program instructions starting 
at position 0 (see `core::instructions::BabyInstruction` for 
documentation on operands). 

## Example  
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