//! # Baby Instructions 
//! 
//! Just a library for the helper type [BabyInstruction][crate::core::instructions::BabyInstruction]
//! that is an enum containing all the possible entries in a program stack, this is 
//! all 7 instructions plus [BabyInstruction::AbsoluteValue][crate::core::instructions::BabyInstruction::AbsoluteValue]
//! which is any program data. 
//! 
//! This type has has several methods for helping converting between 
//! instances of the enum and numerical values that can be put into 
//! an array that can be used to instantiate a new baby model with 
//! a program loaded into the stack. 


use crate::core::MEMORY_WORDS;


/// Defines each of the 7 instructions of the Baby's ISA. 
#[derive(Debug, Clone, Copy)]
pub enum BabyInstruction {
    /// Jump to the instruction at the address obtained from the 
    /// specified memory address (absolute unconditional jump). 
    Jump,
    /// Jump to the instruction at the program counter plus (+) 
    /// the relative value obtained from the specified memory 
    /// address (relative unconditional jump). 
    RelativeJump,
    /// Take the number from the specified memory address S, 
    /// negate it, and load it into the accumulator. 
    Negate,
    /// Store the number in the accumulator to the specified 
    /// memory address S. 
    Store,
    /// Subtract the number at the specified memory address S from the 
    /// value in accumulator, and store the result in the accumulator. 
    Subtract,
    /// Skip next instruction if the accumulator contains a 
    /// negative value. 
    SkipNextIfNegative,
    /// Stop. 
    Stop,
    /// A helper instruction denoting a program data in memory. 
    AbsoluteValue(i32),
}

impl BabyInstruction {
    /// Gets a short description of the instruction. 
    pub fn get_instr_description(&self) -> String {
        match self {
            BabyInstruction::Jump => "jump instruction".to_owned(),
            BabyInstruction::RelativeJump => "relative jump instruction".to_owned(),
            BabyInstruction::Negate => "negate instruction".to_owned(),
            BabyInstruction::Store => "store instruction".to_owned(),
            BabyInstruction::Subtract => "subtract instruction".to_owned(),
            BabyInstruction::SkipNextIfNegative => "skip next if negative instruction".to_owned(),
            BabyInstruction::Stop => "jump instruction".to_owned(),
            BabyInstruction::AbsoluteValue(v) => format!("absolute value {}", v) 
        }
    }
    
    /// Decodes a 16 bit Baby program instruction, returns the 
    /// instruction and operand. 
    /// 
    /// In a program instruction, bits 0–12 represented the memory 
    /// address of the operand to be used, and bits 13–15 specified 
    /// the operation to be executed, such as storing a number in memory; 
    /// the remaining 16 bits were unused.
    /// 
    /// The 3 bit operand is decoded as such: 
    /// 
    /// | Binary   | Instruction                             |
    /// |----------|-----------------------------------------|
    /// | 000      | [`BabyInstruction::Jump`]               |
    /// | 100      | [`BabyInstruction::RelativeJump`]       |
    /// | 010      | [`BabyInstruction::Negate`]             |
    /// | 110      | [`BabyInstruction::Store`]              |
    /// | 001\|101 | [`BabyInstruction::Subtract`]           |
    /// | 011      | [`BabyInstruction::SkipNextIfNegative`] |
    /// | 111      | [`BabyInstruction::Stop`]               |
    /// 
    /// # Parameters
    /// 
    /// * `value` - The instruction to be decoded. 
    /// 
    pub fn from_number(value: u16) -> (BabyInstruction, u16) {
        let opcode = value >> 13;
        let operand = value & 0x1F;
        let res = match opcode {
            0b000 => BabyInstruction::Jump,
            0b100 => BabyInstruction::RelativeJump,
            0b010 => BabyInstruction::Negate,
            0b110 => BabyInstruction::Store,
            0b001 | 0b101 => BabyInstruction::Subtract,
            0b011 => BabyInstruction::SkipNextIfNegative,
            _ => BabyInstruction::Stop,
        };
        (res, operand)
    }

    /// Encodes an instruction and operand into a program instrcution. 
    /// 
    /// Converts the instruction into the relevant value, and incorporates 
    /// the memory address operand, returning the full program instruction 
    /// that can be executed. 
    /// 
    /// # Parameters 
    /// 
    /// * `operand` - The operand memory address to be included in the program instruction. 
    /// 
    pub fn to_number(&self, operand: u16) -> i32 {
        match self {
            BabyInstruction::Jump => (0b000 << 13) | (operand & 0x1F) as i32,
            BabyInstruction::RelativeJump => (0b100 << 13) | (operand & 0x1F) as i32,
            BabyInstruction::Negate => (0b010 << 13) | (operand & 0x1F) as i32,
            BabyInstruction::Store => (0b110 << 13) | (operand & 0x1F) as i32,
            BabyInstruction::Subtract => (0b001 << 13) | (operand & 0x1F) as i32,
            BabyInstruction::SkipNextIfNegative => 0b011 << 13 as i32,
            BabyInstruction::Stop => 0b111 << 13 as i32,
            BabyInstruction::AbsoluteValue(v) => *v
        }
    }

    /// Encodes an array of instructions into an array of program instructions. 
    /// 
    /// Takes a vector of tuples containing [BabyInstruction] and the memory 
    /// address operand ([u16]), returns an array of program instruction values, 
    /// this can be used to initialise [BabyModel][crate::core::BabyModel]. 
    /// 
    /// # Parameters
    /// 
    /// * `instructions` - A vector of [BabyInstruction] and the memory address operand. 
    ///  
    /// # Example 
    /// ```
    /// use baby_emulator::core::{BabyModel, instructions::BabyInstruction};
    /// use baby_emulator::errors::{BabyError, BabyErrors};
    /// 
    /// let instrs = vec![
    ///     (BabyInstruction::Negate, 5),
    ///     (BabyInstruction::Subtract, 5),
    ///     (BabyInstruction::Store, 6),
    ///     (BabyInstruction::Negate, 6),
    ///     (BabyInstruction::Stop, 0)
    /// ];
    /// let mut main_store = BabyInstruction::to_numbers(instrs);
    /// main_store[5] = 5;
    /// 
    /// let model = BabyModel::new_with_program(main_store);
    /// ```
    /// 
    pub fn to_numbers(instructions: Vec<(BabyInstruction, u16)>) -> [i32; MEMORY_WORDS] {
        let res: [usize; MEMORY_WORDS] = core::array::from_fn(|i| i + 1);
        res.map(|i| {
            if let Some((inst, op)) = instructions.get(i) { inst.to_number(*op) }
            else { 0 }
        })
    }
}
