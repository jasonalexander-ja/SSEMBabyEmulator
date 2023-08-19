//! # Parser 
//! 
//! This module parses Baby asm strings, verifying the syntax, and tokenising 
//! the strings into types.
//! 
//! The main functionality of this module is in [parse_asm_string][crate::assembler::parser::parse_asm_string],
//! this takes in a full asm string that can be read from a file, and tries to 
//! parse it into [LineType][crate::assembler::parser::LineType] that represents 
//! all the possible types of lines in Baby asm. 
//! 
//! This can assemble both modern and original notation depending on the 
//! value passed to `og_notation`. 
//! 
//! The output of this can be fed straight into [linker::link_parsed_lines][crate::assembler::linker::link_parsed_lines]
//! to produce a final fully assembled machine code from the asm string. 
//! 
//! # Example
//! ```
//! use baby_emulator::assembler::parser;
//! use baby_emulator::assembler::linker;
//! use baby_emulator::assembler::errors::AssemblyError;
//! use baby_emulator::core::instructions::BabyInstruction;
//! 
//! pub fn assemble(asm: &String) -> Result<Vec<BabyInstruction>, AssemblyError> {
//!     let parse_result = match parser::parse_asm_string(asm, false) {
//!         Ok(v) => v,
//!         Err((l, e)) => return Err(AssemblyError::ParserError(l, e))
//!     };
//!     match linker::link_parsed_lines(parse_result) {
//!         Ok(v) => Ok(v),
//!         Err(e) => Err(AssemblyError::LinkerError(e))
//!     }
//! }
//! ```
//! 

use parse_errors::{
    LineParseError,
    TagError,
    AbsoluteError,
    InstructionError,
    ValueParseError,
};


/// Contains types for handling errors during parsing. 
pub mod parse_errors;
#[cfg(test)]
mod tests;

/// Represents the possible nations for a value. 
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    /// A literal value. 
    Value(i32),
    /// A reference to a [LineType::Tag] value E.G. `$SomeTag`.
    Tag(String),
}

impl Value {

    /// Tried to parse a value expression into an instance of [Value]. 
    /// 
    /// Returns [Ok(Value)] if there is a valid value detected, returns an 
    /// [Err(ValueParseError)] if the value is invalid, detailing what is 
    /// wrong. 
    /// 
    /// # Can parse
    /// * Hex - `0xA` = 10
    /// * Decimal - `0d10` = 10
    /// * Octal - `0o12` = 10
    /// * Binary - `0b1010` = 10
    /// * Tags - `$foo` = "foo"
    pub fn parse(value: &str) -> Result<Value, ValueParseError> {
        let value = value.trim();
        match value {
            v if v.starts_with("0x") => Self::parse_hex(v.replace("0x", "")),
            v if v.starts_with("0d") => Self::parse_decimal(v.replace("0d", "")),
            v if v.starts_with("0o") => Self::parse_octal(v.replace("0o", "")),
            v if v.starts_with("0b") => Self::parse_binary(v.replace("0b", "")),
            v if v.starts_with("$") => Self::parse_tag_name(v.replace("$", "")),
            _ => Err(ValueParseError::InvalidValue(value.to_string()))
        }
    }

    /// Tries to parse a hex string. 
    /// 
    /// Returns a [ValueParseError] if it fails. 
    pub fn parse_hex(value: String) -> Result<Value, ValueParseError> {
        let res = match i32::from_str_radix(&value, 16) {
            Ok(v) => v,
            Err(_) => return Err(ValueParseError::InvalidHex(value.to_string()))
        };
        Ok(Value::Value(res))
    }

    /// Tries to parse a decimal string. 
    /// 
    /// Returns a [ValueParseError] if it fails. 
    pub fn parse_decimal(value: String) -> Result<Value, ValueParseError> {
        match i32::from_str_radix(&value, 10) {
            Ok(v) => Ok(Value::Value(v)),
            Err(_) => Err(ValueParseError::InvalidDecimal(value.to_string()))
        }
    }

    /// Tries to parse an octal string. 
    /// 
    /// Returns a [ValueParseError] if it fails. 
    pub fn parse_octal(value: String) -> Result<Value, ValueParseError> {
        match i32::from_str_radix(&value, 8) {
            Ok(v) => Ok(Value::Value(v)),
            Err(_) => Err(ValueParseError::InvalidOctal(value.to_string()))
        }
    }

    /// Tries to parse a binary string. 
    /// 
    /// Returns a [ValueParseError] if it fails. 
    pub fn parse_binary(value: String) -> Result<Value, ValueParseError> {
        match i32::from_str_radix(&value, 2) {
            Ok(v) => Ok(Value::Value(v)),
            Err(_) => Err(ValueParseError::InvalidBinary(value.to_string()))
        }
    }

    /// Tries to parse a tag reference. 
    /// 
    /// Returns a [ValueParseError] if it contains any whitespace. 
    pub fn parse_tag_name(v: String) -> Result<Value, ValueParseError> {
        if !v.contains(char::is_whitespace) {
            return Ok(Value::Tag(v))
        }
        Err(ValueParseError::InvalidTagName(v.to_string()))
    }

}

/// Represents all the instructions. 
#[derive(Clone, Debug, PartialEq)]
pub enum Instruction {
    /// See [BabyInstruction::Jump][crate::core::instructions::BabyInstruction::Jump].
    Jump(Value),
    /// See [BabyInstruction::RelativeJump][crate::core::instructions::BabyInstruction::RelativeJump].
    RelativeJump(Value),
    /// See [BabyInstruction::Negate][crate::core::instructions::BabyInstruction::Negate].
    Negate(Value),
    /// See [BabyInstruction::Store][crate::core::instructions::BabyInstruction::Store].
    Store(Value),
    /// See [BabyInstruction::Subtract][crate::core::instructions::BabyInstruction::Subtract].
    Subtract(Value),
    /// See [BabyInstruction::SkipNextIfNegative][crate::core::instructions::BabyInstruction::SkipNextIfNegative].
    Test,
    /// See [BabyInstruction::Stop][crate::core::instructions::BabyInstruction::Stop].
    Stop,
}

impl Instruction {

    /// Parses Baby asm instruction & operands using modern notation 
    /// 
    /// # Possible Instruction 
    /// | Asm   | Description                                                                                                                                           |
    /// |-------|-------------------------------------------------------------------------------------------------------------------------------------------------------|
    /// | JMP # | Jump to the instruction at the address obtained from the specified memory address # (absolute unconditional jump)                                     |
    /// | JRP # | Jump to the instruction at the program counter plus (+) the relative value obtained from the specified memory address # (relative unconditional jump) |
    /// | LDN # | Take the number from the specified memory address #, negate it, and load it into the accumulator                                                      |
    /// | STO # | Store the number in the accumulator to the specified memory address #                                                                                 |
    /// | SUB # | Subtract the number at the specified memory address # from the value in accumulator, and store the result in the accumulator                          |
    /// | CMP   | Skip next instruction if the accumulator contains a negative value                                                                                    |
    /// | STP   | Stop                                                                                                                                                  |
    /// 
    /// * `#` is a always a memory address, and will try to be parsed into a [Value]. 
    /// 
    pub fn parse(instruction: &str) -> Result<Instruction, InstructionError> {
        let instruction = instruction.trim().to_lowercase();
        let v = Value::Value(0);
        match instruction {

            c if c.starts_with("jmp ") => 
                Self::make_instruction(Instruction::Jump(v), c.replace("jmp ", "")),
            c if c.starts_with("jrp ") => 
                Self::make_instruction(Instruction::RelativeJump(v), c.replace("jrp ", "")),
            c if c.starts_with("ldn ") => 
                Self::make_instruction(Instruction::Negate(v), c.replace("ldn ", "")),
            c if c.starts_with("sto ") => 
                Self::make_instruction(Instruction::Store(v), c.replace("sto ", "")),
            c if c.starts_with("sub ") => 
                Self::make_instruction(Instruction::Subtract(v), c.replace("sub ", "")),

            c if c.starts_with("cmp") => Ok(Instruction::Test),
            c if c.starts_with("stp") => Ok(Instruction::Stop),

            _ => Err(InstructionError::UnkownInstruction(instruction.to_string()))
        }
    }

    /// Parses Baby asm instructions & operands using original notation 
    /// 
    /// # Possible Instructions 
    /// | Asm       | Description                                                                                                                                           |
    /// |-----------|-------------------------------------------------------------------------------------------------------------------------------------------------------|
    /// | #, Cl     | Jump to the instruction at the address obtained from the specified memory address # (absolute unconditional jump)                                     |
    /// | Add #, Cl | Jump to the instruction at the program counter plus (+) the relative value obtained from the specified memory address # (relative unconditional jump) |
    /// | -#, C     | Take the number from the specified memory address #, negate it, and load it into the accumulator                                                      |
    /// | c, #      | Store the number in the accumulator to the specified memory address #                                                                                 |
    /// | SUB #     | Subtract the number at the specified memory address # from the value in accumulator, and store the result in the accumulator                          |
    /// | Test      | Skip next instruction if the accumulator contains a negative value                                                                                    |
    /// | Stop      | Stop                                                                                                                                                  |
    /// 
    /// * `#` is a always a memory address 
    /// 
    pub fn parse_ogn(instruction: &str) -> Result<Instruction, InstructionError> {
        let instruction = instruction.trim();
        let v = Value::Value(0);
        match instruction {
            c if c.ends_with(", Cl") && !c.starts_with("Add ") => 
                Self::make_instruction(Instruction::Jump(v), c.replace(", Cl", "")),
            c if c.starts_with("Add ") && c.ends_with(", Cl") => 
                Self::make_instruction(Instruction::RelativeJump(v), c.replace("Add ", "").replace(", Cl", "")),
            c if c.starts_with("-") && c.ends_with(", C") => 
                Self::make_instruction(Instruction::Negate(v), c.replace("-", "").replace(", C", "")),
            c if c.starts_with("c, ") => 
                Self::make_instruction(Instruction::Store(v), c.replace("c, ", "")),
            c if c.starts_with("SUB ") => 
                Self::make_instruction(Instruction::Subtract(v), c.replace("SUB ", "")),

            c if c.starts_with("Test") => Ok(Instruction::Test),
            c if c.starts_with("Stop") => Ok(Instruction::Stop),

            _ => Err(InstructionError::UnkownInstruction(instruction.to_string()))
        }
    }

    /// Returns a string with a short description of the instruction. 
    pub fn describe(&self) -> String {
        match self {
            Instruction::Jump(_) => "jump".to_owned(),
            Instruction::RelativeJump(_) => "relative jump".to_owned(),
            Instruction::Negate(_) => "negate".to_owned(),
            Instruction::Store(_) => "store".to_owned(),
            Instruction::Subtract(_) => "subtract".to_owned(),
            Instruction::Test => "test".to_owned(),
            Instruction::Stop => "stop".to_owned(),
        }
    }

    /// Returns the stored memory address operand of a instruction,
    /// returns a 0 if a stop or test.
    pub fn get_operand(&self) -> Value {
        match self {
            Instruction::Jump(v) => v.clone(),
            Instruction::RelativeJump(v) => v.clone(),
            Instruction::Negate(v) => v.clone(),
            Instruction::Store(v) => v.clone(),
            Instruction::Subtract(v) => v.clone(),
            Instruction::Test => Value::Value(0),
            Instruction::Stop => Value::Value(0),
        }
    }

    /// Tries to parse an operand value expression, combining it 
    /// with an instruction. 
    /// 
    /// Will return an [InstructionError] if parsing the operand value 
    /// fails.
    /// 
    /// # Parameters
    /// 
    /// * `instr` - The instruction to be used. 
    /// * `operand` - The operand value expression to be parsed and combined. 
    /// 
    pub fn make_instruction(instr: Instruction, operand: String) -> Result<Instruction, InstructionError> {
        let value = match Value::parse(&operand) {
            Ok(v) => v,
            Err(e) => return Err(InstructionError::OperandValueParseError(instr, e))
        };
        let res = match instr {
            Instruction::Jump(_) => Instruction::Jump(value),
            Instruction::RelativeJump(_) => Instruction::RelativeJump(value),
            Instruction::Negate(_) => Instruction::Negate(value),
            Instruction::Store(_) => Instruction::Store(value),
            Instruction::Subtract(_) => Instruction::Subtract(value),
            v => v
        };
        Ok(res)
    }
}

/// Represents all the possible syntaxes for a line. 
#[derive(Clone, Debug, PartialEq)]
pub enum LineType {
    /// A named reference to a position in the program code. 
    /// 
    /// # Asm Example 
    /// ```text
    /// :start
    /// JMP $start ;jumps to the start of the program 
    /// ```
    Tag(String),
    /// An absolute value in the program stack. 
    Absolute(Value),
    /// An actual instruction directive telling the computer to 
    /// perform an action. 
    Instruction(Instruction),
}

/// Splits an asm string into lines, removes the blank lines and
/// tries to parse each one. 
/// 
/// Basically a wrapper for [parse_lines]. 
/// 
/// Returns a list of [LineType] corresponding to the line type 
/// and metadata. Each line can either be an absolute value, a 
/// instruction, or a tag to reference back to a location in the 
/// program stack. 
/// 
/// Will return a tuple of [usize] and [LineParseError] if an error is 
/// encountered, containing the metatdata on the error encountered and the
/// index of the line it was found on. 
/// 
pub fn parse_asm_string(asm: &String, og_notation: bool) -> Result<Vec<LineType>, (usize, LineParseError)> {
    let lines: Vec<String> = asm.lines()
        .map(strip_comments)
        .filter(|l| !l.is_empty())
        .collect();
    parse_lines(lines, og_notation)
}

/// Tries to parse a vector of lines of Baby asm 
/// 
/// Returns a list of [LineType] corresponding to the 
/// line type and metadata. Each line can either be an absolute 
/// value, a instruction, or a tag to reference back to a location 
/// in the program stack. 
/// 
/// Will return a tuple of [usize] and [LineParseError] if an error is 
/// encountered, containing the metatdata on the error encountered and the
/// index of the line it was found on. 
/// 
pub fn parse_lines(lines: Vec<String>, og_notation: bool) -> Result<Vec<LineType>, (usize, LineParseError)> {
    let mut res: Vec<LineType> = vec![];
    for (index, line) in lines.iter().enumerate() {
        match parse_line(line, og_notation) {
            Ok(l) => res.push(l),
            Err(e) => return Err((index, e))
        }
    }
    Ok(res)
}

/// Tries to parse a line of Baby asm 
/// 
/// Returns an instance of [LineType] corresponding to the 
/// line type and metadata. Each line can either be an absolute 
/// value, a instruction, or a tag to reference back to a location 
/// in the program stack. 
/// 
/// Will return an instance of [LineParseError] if an error is 
/// encountered, containing metatdata on the error encountered.  
/// 
pub fn parse_line(line: &String, og_notation: bool) -> Result<LineType, LineParseError> {
    let line = line.trim().to_lowercase();
    let line = strip_comments(&line);
    match line {
        l if l.starts_with(":") => parse_tag(l.replace(":", "")),
        l if l.starts_with("abs ") => parse_absolute(l.replace("abs ", "")),
        l => if og_notation { parse_instruction(l) } 
            else { parse_instruction_ogn(l) },
    }
}

/// Strips comments from a line of Baby asm. 
/// 
/// # Example
/// ```
/// use baby_emulator::assembler::parser::strip_comments;
/// 
/// assert_eq!(strip_comments("sub 0xA ;foo"), "sub 0xA ".to_owned());
/// ```
/// 
pub fn strip_comments(line: &str) -> String {
    let lines = if let Some(v) = line.split(";").next() { v }
        else { "" };
    lines.to_owned()
}

/// Parses a tag declaration. 
/// 
/// Returns [LineParseError::TagError] if the tag name contains
/// any whitepsace. 
pub fn parse_tag(tag: String) -> Result<LineType, LineParseError> {
    let tag = tag.trim();
    if tag.contains(char::is_whitespace) {
        return Err(LineParseError::TagError(TagError::TagNameWhitespace(tag.to_string())))
    }
    Ok(LineType::Tag(tag.to_string()))
}

/// Parses an absolute value expression. 
/// 
/// Will return [AbsoluteError::ValueError] if an error is thrown 
/// when parsing the value expression. 
pub fn parse_absolute(tag: String) -> Result<LineType, LineParseError> {
    match Value::parse(&tag) {
        Ok(v) => Ok(LineType::Absolute(v)),
        Err(e) => Err(LineParseError::AbsoluteError(AbsoluteError::ValueError(e)))
    }
}

/// Parses an asm instruction using modern notation. 
/// 
/// Will return [LineParseError::InstructionError] if the instruction isn't 
/// recognised or there is an error parsing the operand value. 
/// 
/// # Example
/// ```
/// use baby_emulator::assembler::parser::{LineType, Instruction, parse_instruction};
/// 
/// match parse_instruction("stp".to_owned()) {
///     Ok(LineType::Instruction(Instruction::Stop)) => println!("Sucess. "),
///     _ => panic!()
/// }
/// ```
/// 
pub fn parse_instruction(instruction: String) -> Result<LineType, LineParseError> {
    match Instruction::parse(&instruction) {
        Ok(v) => Ok(LineType::Instruction(v)),
        Err(e) => Err(LineParseError::InstructionError(e))
    }
}

/// Parses an asm instruction original notation. 
/// 
/// Will return [LineParseError::InstructionError] if the instruction isn't 
/// recognised or there is an error parsing the operand value. 
/// 
/// # Example
/// ```
/// use baby_emulator::assembler::parser::{LineType, Instruction, parse_instruction_ogn};
/// 
/// match parse_instruction_ogn("Stop".to_owned()) {
///     Ok(LineType::Instruction(Instruction::Stop)) => println!("Sucess. "),
///     _ => panic!()
/// }
/// ```
/// 
pub fn parse_instruction_ogn(instruction: String) -> Result<LineType, LineParseError> {
    match Instruction::parse_ogn(&instruction) {
        Ok(v) => Ok(LineType::Instruction(v)),
        Err(e) => Err(LineParseError::InstructionError(e))
    }
}
