use std::ops::Neg;

pub trait BabyError {
    fn get_descriptor(&self) -> String;
    fn at(&self) -> u16;
}

pub struct OperandAddressOutOfRange {
    instruction: BabyInstruction,
    at: u16,
    operand: u16
}

impl BabyError for OperandAddressOutOfRange {
    fn get_descriptor(&self) -> String {
        format!("Operand points to an address outside of the allocated memory; for instruction {}; at address {:#06x}; operand {};\n", self.instruction.get_instr_description(), self.at, self.operand)
    }

    fn at(&self) -> u16 {
        self.at
    }
}

pub enum BabyInstruction {
    Jump(u16),
    RelativeJump(u16),
    Negate(u16),
    Store(u16),
    Subtract(u16),
    SkipNextIfNegative,
    Stop
}

impl BabyInstruction {
    fn get_instr_description(&self) -> String {
        match self {
            BabyInstruction::Jump(_) => "jump instruction".to_owned(),
            BabyInstruction::RelativeJump(_) => "relative jump instruction".to_owned(),
            BabyInstruction::Negate(_) => "negate instruction".to_owned(),
            BabyInstruction::Store(_) => "store instruction".to_owned(),
            BabyInstruction::Subtract(_) => "subtract instruction".to_owned(),
            BabyInstruction::SkipNextIfNegative => "skip next if negative instruction".to_owned(),
            BabyInstruction::Stop => "jump instruction".to_owned(),
        }
    }
}

impl BabyInstruction {
    pub fn from_number(value: u16) -> BabyInstruction {
        let opcode = value >> 13;
        let operand = value & 0x1FFF;
        match opcode {
            0b000 => BabyInstruction::Jump(operand),
            0b100 => BabyInstruction::RelativeJump(operand),
            0b010 => BabyInstruction::Negate(operand),
            0b110 => BabyInstruction::Store(operand),
            0b001 | 0b101 => BabyInstruction::Subtract(operand),
            0b011 => BabyInstruction::SkipNextIfNegative,
            _ => BabyInstruction::Stop,
        }
    }
}

pub struct BabyModel {
    pub main_store: [i32; 32],
    pub accumulator: i32,
    pub instruction_address: u16,
    pub instruction: u16,
}

impl BabyModel {
    pub fn new() -> BabyModel {
        BabyModel {
            main_store: [0; 32],
            accumulator: 0,
            instruction_address: 0,
            instruction: 0,
        }
    }

    pub fn new_with_program(main_store: [i32; 32]) -> BabyModel {
        BabyModel { 
            main_store,
            accumulator: 0,
            instruction_address: 0,
            instruction: 0
        }
    }

    pub fn jump(&self, addr: u16) -> BabyModel {
        let address = self.main_store[addr as usize] & 0xFFFF;
        BabyModel { 
            main_store: self.main_store.clone(),
            accumulator: self.accumulator,
            instruction_address: address as u16,
            instruction: self.instruction
        }
    }

    pub fn relative_jump(&self, addr: u16) -> BabyModel {
        let address = self.main_store[addr as usize] & 0xFFFF;
        BabyModel { 
            main_store: self.main_store.clone(),
            accumulator: self.accumulator,
            instruction_address: self.instruction_address + (address as u16),
            instruction: self.instruction
        }
    }

    pub fn negate(&self, addr: u16) -> BabyModel {
        let value = self.main_store[addr as usize].neg();
        BabyModel { 
            main_store: self.main_store.clone(),
            accumulator: value,
            instruction_address: self.instruction_address + 1,
            instruction: self.instruction
        }
    }

    pub fn store(&self, addr: u16) -> BabyModel {
        let mut main_store = self.main_store.clone();
        let address = self.main_store[addr as usize] & 0xFFFF;
        main_store[address as usize] = self.accumulator;
        BabyModel { 
            main_store,
            accumulator: self.accumulator,
            instruction_address: self.instruction_address + 1,
            instruction: self.instruction
        }
    }

    pub fn subtract(&self, addr: u16) -> BabyModel {
        let value = self.main_store[addr as usize] & 0xFFFF;
        BabyModel { 
            main_store: self.main_store.clone(),
            accumulator: self.accumulator - value,
            instruction_address: self.instruction_address + 1,
            instruction: self.instruction
        }
    }

    pub fn test(&self) -> BabyModel {
        let next = if self.accumulator.is_negative() { 2 }
        else { 1 };
        BabyModel { 
            main_store: self.main_store.clone(),
            accumulator: self.accumulator,
            instruction_address: self.instruction_address + next,
            instruction: self.instruction
        }
    }

    pub fn core_dump(&self) -> String {
        let mut res = format!("Accumulator: {:#010x}; Instruction Register: {:#06x};\n", self.accumulator, self.instruction);
        res += &format!("Instruction Address: {:#06x}; Main Store: \n", self.instruction_address);
        for i in 0..8 {
            let offset = i * 4;
            for i2 in 0..4 {
                let addr = i2 + offset;
                res += &format!("{:#04x}: {:#010x}; ", addr, self.main_store[addr]);
            }
            res += "\n";
        }
        res += "\n";
        return res;
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
