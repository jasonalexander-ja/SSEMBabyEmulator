

/// Defines each of the 7 instructions of the Baby's ISA. 
#[derive(Debug, Clone, Copy)]
pub enum BabyInstruction {
    /// Jump to the instruction at the address obtained from the specified memory address (absolute unconditional jump). 
    Jump,
    /// Jump to the instruction at the program counter plus (+) the relative value obtained from the specified memory address S[a] (relative unconditional jump). 
    RelativeJump,
    /// Take the number from the specified memory address S, negate it, and load it into the accumulator. 
    Negate,
    /// Store the number in the accumulator to the specified memory address S. 
    Store,
    /// Subtract the number at the specified memory address S from the value in accumulator, and store the result in the accumulator. 
    Subtract,
    /// Skip next instruction if the accumulator contains a negative value. 
    SkipNextIfNegative,
    /// Stop. 
    Stop
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
        }
    }
    
    /// Decodes a 16 bit Baby program instruction, returns the instruction and operand. 
    /// 
    /// In a program instruction, bits 0–12 represented the memory address of the operand to be used, 
    /// and bits 13–15 specified the operation to be executed, such as storing a number in memory; 
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
    /// # Parameters 
    /// 
    /// * `op` - The operand memory address to be included in the program instruction. 
    /// 
    pub fn to_number(&self, op: u16) -> i32 {
        (match self {
            BabyInstruction::Jump => (0b000 << 13) | (op & 0x1F),
            BabyInstruction::RelativeJump => (0b100 << 13) | (op & 0x1F),
            BabyInstruction::Negate => (0b010 << 13) | (op & 0x1F),
            BabyInstruction::Store => (0b110 << 13) | (op & 0x1F),
            BabyInstruction::Subtract => (0b001 << 13) | (op & 0x1F),
            BabyInstruction::SkipNextIfNegative => 0b011 << 13,
            BabyInstruction::Stop => 0b111 << 13,
        } as i32)
    }
}
