//! # Baby Assembly
//! 
//! This module contains the types and functionality for assembling 
//! Baby asm. 
//! 
//! The main part of this module is the [assemble][assembler::assemble] function, that takes
//! a string of asm and attempts to assemble to a vector of [BabyInstruction][crate::core::instructions::BabyInstruction] 
//! representing individual Baby machine code instructions. 
//! 
//! This can be fed straight into [BabyInstruction::to_numbers][crate::core::instructions::BabyInstruction::to_numbers] 
//! that can then be used to with [BabyModel::new_with_program][crate::core::BabyModel::new_with_program] 
//! to make a  runnable instance of the Baby emulator loaded with the assembled program. 
//! 
//! # Example
//! 
//! This is a simple example that assembles an assembly string,
//! converts the result to a memory array using [BabyInstruction::to_numbers][crate::core::instructions::BabyInstruction::to_numbers], 
//! instantiates a baby model  and then runs it as per the example 
//! in [core][crate::core] module.  
//! 
//! ```
//! use baby_emulator::assembler::assemble; 
//! use baby_emulator::core::{BabyModel, instructions::BabyInstruction};
//! 
//! 
//! const ASM: &str = 
//! "
//! ldn $start_value  ; Loads 10 into the accumulator 
//!  
//! :loop_start
//! sub $subtract_val ; Subtract 1 from the accumulator 
//! cmp               ; Skip the next jump instruction if the accumulator is negative 
//! jmp $loop_start   ; Jump to the start of the loop 
//! stp               ; Program stops when the accumulator is negative 
//!  
//! :subtract_val     ; Value to be subtracted
//! abs 0d1
//!  
//! :start_value ; Value to start in the accumulator 
//! abs 0d-10
//! ";
//!
//! fn main() {
//!     let instructions = match assemble(&String::from(ASM), false) {
//!         Ok(v) => v,
//!         Err(e) => { println!("{}", e.describe(true)); return; }
//!     };
//!     let main_store = BabyInstruction::to_numbers(instructions);
//! 
//!     let mut model = BabyModel::new_with_program(main_store);
//!     loop {
//!         model = match model.execute() {
//!             Ok(m) => m,
//!             Err(_) => break
//!         };
//!     }
//!     println!("{}", model.core_dump());
//! }
//! ```
//! 
//! # Syntax
//! The original Manchester Baby did not have an assembly language, this is just 
//! a simple language put together to make programming a little bit easier. 
//! 
//! ## Instructions
//! Baby assembly contains the following instructions:
//! 
//! | Modern Notation | Original Notation | Description                                                                                                                                           |
//! |-----------------|-------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------|
//! | JMP #           | #, Cl             | Jump to the instruction at the address obtained from the specified memory address # (absolute unconditional jump)                                     |
//! | JRP #           | Add #, Cl         | Jump to the instruction at the program counter plus (+) the relative value obtained from the specified memory address # (relative unconditional jump) |
//! | LDN #           | -#, C             | Take the number from the specified memory address #, negate it, and load it into the accumulator                                                      |
//! | STO #           | c, #              | Store the number in the accumulator to the specified memory address #                                                                                 |
//! | SUB #           | SUB #             | Subtract the number at the specified memory address # from the value in accumulator, and store the result in the accumulator                          |
//! | CMP             | Test              | Skip next instruction if the accumulator contains a negative value                                                                                    |
//! | STP             | Stop              | Stop                                                                                                                                                  |
//! 
//! * All modern instruction are case insensitive. 
//! * `#` represents a value which can be in any format in the 
//! Values table above. 
//! * All instructions with a value operand `#` the value operand is always 
//! a memory address. 
//! 
//! ## Values 
//! The following value formats are accepted: 
//! 
//! | Format        | Example |
//! |---------------|---------|
//! | Hex           | 0xA     |
//! | Decimal       | 0d10    |
//! | Octal         | 0o12    |
//! | Binary        | 0b1010  |
//! | Tag Reference | $start  |
//! 
//! ## Tags
//! You can tag a position in the program stack and reference it's memory address 
//! as a value using a named tag, for instance the following code jumps to a 
//! stop command by referencing `prog_end` as the value parameter to a jump instruction. 
//! 
//! ```text
//! jmp $prog_end
//! 
//! :prog_end
//! stp
//! ```
//! 
//! ## Absolute values
//! Often you will need to include values in your program code in, you will have to 
//! add these to your program stack in memory, to do this you can use the `abs #` directive, 
//! this looks looks like an instruction but in reality just tells the assembler to 
//! just put whatever value you wrote into the program stack. 
//! 
//! These the value can be of any kind from the values table above, including a tag 
//! reference, you can also write negative values here like `0x-A`, `0d-10`, `0o-12`, `0b-1010`
//! (tag references cannot be written as negative however). 
//! 
//! The below program loads a "10" into the accumulator from memory by negatively loading a
//! "-10" from a location denoted by the tag `foo`: 
//! ```text
//! ldn $foo
//! 
//! :foo
//! abs 0d-10
//! ```
//! 
//! ## Comments 
//! Comments can be used with a `;` as with other assembly languages. 
//! 
//! ```text
//! ; This is a comment
//! JMP 0d1 ; And so is this 
//! ```
//! 
//! ## Full example 
//! This shows a simple program in both modern and original notation that 
//! loads negative a -10 into the accumulator (or, loads 10 into the accumulator),
//! then enters a loop where it subtracts 1 from the accumulator and jumps back 
//! to the start of the loop if the accumulator is not negative, when the accumulator 
//! is negative, a stop instruction is hit and the machine stops. 
//! 
//! Modern notation: 
//! ```text
//! ldn $start_value  ; Loads 10 into the accumulator 
//! 
//! :loop_start
//! sub $subtract_val ; Subtract 1 from the accumulator 
//! cmp               ; Skip the next jump instruction if the accumulator is negative 
//! jmp $loop_start   ; Jump to the start of the loop 
//! stp               ; Program stops when the accumulator is negative 
//! 
//! :subtract_val     ; Value to be subtracted
//! abs 0d1
//! 
//! :start_value ; Value to start in the accumulator 
//! abs 0d-10
//! ```
//! 
//! Original notation: 
//! ```text
//! -$start_value, C  ; Loads 10 into the accumulator 
//! 
//! :loop_start
//! SUB $subtract_val ; Subtract 1 from the accumulator 
//! Test              ; Skip the next jump instruction if the accumulator is negative 
//! $loop_start, CL   ; Jump to the start of the loop 
//! Stop              ; Program stops when the accumulator is negative 
//! 
//! :subtract_val     ; Value to be subtracted
//! abs 0d1
//! 
//! :start_value      ; Value to start in the accumulator 
//! abs 0d-10
//! ```
//! 
use crate::core::instructions::BabyInstruction;
use errors::AssemblyError;


/// Contains types and functionality for parsing Baby asm. 
pub mod parser;
/// Contains types and functionality for linking asm post parsing.  
pub mod linker;
/// Contains types for handling errors found during the overall 
/// assembling process. 
pub mod errors;

/// Assembles a string of Baby asm to a vector of [BabyInstruction] machine code instructions. 
/// 
/// Can assemble for both modern and original notation depending on `og_notation`. 
/// 
/// If sucessful it will return a vector of [BabyInstruction]. 
/// 
/// This type can be fed straight into [BabyInstruction::to_numbers] to return an
/// array of [i32] that can be used to directly instantiate [BabyModel][crate::core::BabyModel]
/// via [BabyModel::new_with_program][crate::core::BabyModel::new_with_program] and 
/// run the assembled program.
/// 
/// Returns [AssemblyError] if an error is encountered at any point. 
/// 
/// Possible errors are that a tag reference that cannot be bound or the assembled 
/// program stack is greater than the Baby's total available memory (see [MEMORY_WORDS][crate::core::MEMORY_WORDS]). 
/// 
/// # Parameters
/// * `asm` - The assembly string. 
/// * `og_notation` - If true, will use original notation. 
/// 
pub fn assemble(asm: &String, og_notation: bool) -> Result<Vec<BabyInstruction>, AssemblyError> {
    let parse_result = match parser::parse_asm_string(asm, og_notation) {
        Ok(v) => v,
        Err((l, e)) => return Err(AssemblyError::ParserError(l, e))
    };
    match linker::link_parsed_lines(parse_result) {
        Ok(v) => Ok(v),
        Err(e) => Err(AssemblyError::LinkerError(e))
    }
}
