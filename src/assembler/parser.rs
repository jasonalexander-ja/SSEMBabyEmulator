use super::parse_errors::{
    LineParseError,
    TagError,
    AbsoluteError,
    InstructionError,
    ValueParseError,
};


/// Represents the possible nations for a value. 
#[derive(Clone)]
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
            v if v.starts_with("0x") => Self::parse_decimal(v.replace("0d", "")),
            v if v.starts_with("0x") => Self::parse_octal(v.replace("0o", "")),
            v if v.starts_with("0x") => Self::parse_binary(v.replace("0b", "")),
            v if v.starts_with("0x") => Self::parse_tag_name(v.replace("$", "")),
            _ => Err(ValueParseError::InvalidValue(value.to_string()))
        }
    }

    /// Tries to parse a hex string. 
    /// 
    /// Returns a [ValueParseError] if it fails. 
    pub fn parse_hex(v: String) -> Result<Value, ValueParseError> {
        match i32::from_str_radix(&v, 16) {
            Ok(v) => Ok(Value::Value(v)),
            Err(v) => Err(ValueParseError::InvalidHex(v.to_string()))
        }
    }

    /// Tries to parse a decimal string. 
    /// 
    /// Returns a [ValueParseError] if it fails. 
    pub fn parse_decimal(v: String) -> Result<Value, ValueParseError> {
        match i32::from_str_radix(&v, 10) {
            Ok(v) => Ok(Value::Value(v)),
            Err(v) => Err(ValueParseError::InvalidDecimal(v.to_string()))
        }
    }

    /// Tries to parse an octal string. 
    /// 
    /// Returns a [ValueParseError] if it fails. 
    pub fn parse_octal(v: String) -> Result<Value, ValueParseError> {
        match i32::from_str_radix(&v, 8) {
            Ok(v) => Ok(Value::Value(v)),
            Err(v) => Err(ValueParseError::InvalidOctal(v.to_string()))
        }
    }

    /// Tries to parse a binary string. 
    /// 
    /// Returns a [ValueParseError] if it fails. 
    pub fn parse_binary(v: String) -> Result<Value, ValueParseError> {
        match i32::from_str_radix(&v, 2) {
            Ok(v) => Ok(Value::Value(v)),
            Err(v) => Err(ValueParseError::InvalidBinary(v.to_string()))
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
#[derive(Clone)]
pub enum Instruction {
    /// See [crate::core::instructions::BabyInstruction::Jump].
    Jump(Value),
    /// See [crate::core::instructions::BabyInstruction::RelativeJump].
    RelativeJump(Value),
    /// See [crate::core::instructions::BabyInstruction::Negate].
    Negate(Value),
    /// See [crate::core::instructions::BabyInstruction::Store].
    Store(Value),
    /// See [crate::core::instructions::BabyInstruction::Subtract].
    Subtract(Value),
    /// See [crate::core::instructions::BabyInstruction::SkipNextIfNegative].
    Test,
    /// See [crate::core::instructions::BabyInstruction::Stop].
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
    /// * # is a always a memory address, and will try to be parsed into a [Value]. 
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
    /// * # is a always a memory address 
    /// 
    pub fn parse_ogn(instruction: &str) -> Result<Instruction, InstructionError> {
        let v = Value::Value(0);
        match instruction {
            c if c.ends_with(", CL") => 
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
#[derive(Clone)]
pub enum LineType {
    /// A named reference to a position in the program code. 
    /// 
    /// # Asm Example 
    /// ```
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

impl LineType {

    /// Tries tp parse a line of Baby asm 
    /// 
    /// Returns an instance of [LineType] corresponding to the 
    /// line type and metadata. Each line can either be an absolute 
    /// value, a instruction, or a tag to reference back to a location 
    /// in the program stack. 
    /// 
    /// Will return an instance of [LineParseError] if an error is 
    /// encountered, containing metatdata on the error encountered.  
    /// 
    pub fn parse_line(line: String, og_notation: bool) -> Result<LineType, LineParseError> {
        let line = line.trim().to_lowercase();
        let line = Self::strip_comments(&line);
        match line {
            l if l.starts_with(":") => Self::parse_tag(l.replace(":", "")),
            l if l.starts_with("abs ") => Self::parse_absolute(l.replace("abs ", "")),
            l => if og_notation { Self::parse_instruction(l) } 
                else { Self::parse_instruction_ogn(l) },
        }
    }

    /// Strips comments from a line of Baby asm. 
    /// 
    /// # Example
    /// ```
    /// use baby_emulator::assembler::parser::LineType;
    /// 
    /// assert_eq!(LineType::strip_comments("sub 0xA ;foo"), "sub 0xA ".to_owned());
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
    /// use baby_emulator::assembler::parser::{LineType, Instruction};
    /// 
    /// match LineType::parse_instruction("stp".to_owned()) {
    ///     Ok(LineType::Instruction(Instruction::Stop)) => println!("Sucess. "),
    ///     _ => panic!()
    /// }
    /// ```
    pub fn parse_instruction(instruction: String) -> Result<LineType, LineParseError> {
        match Instruction::parse(&instruction) {
            Ok(v) => Ok(LineType::Instruction(v)),
            Err(e) => Err(LineParseError::InstructionError(e))
        }
    }

    /// Parses an asm instruction original modern notation. 
    /// 
    /// Will return [LineParseError::InstructionError] if the instruction isn't 
    /// recognised or there is an error parsing the operand value. 
    /// 
    /// # Example
    /// ```
    /// use baby_emulator::assembler::parser::{LineType, Instruction};
    /// 
    /// match LineType::parse_instruction_ogn("Stop".to_owned()) {
    ///     Ok(LineType::Instruction(Instruction::Stop)) => println!("Sucess. "),
    ///     _ => panic!()
    /// }
    /// ```
    pub fn parse_instruction_ogn(instruction: String) -> Result<LineType, LineParseError> {
        match Instruction::parse_ogn(&instruction) {
            Ok(v) => Ok(LineType::Instruction(v)),
            Err(e) => Err(LineParseError::InstructionError(e))
        }
    }
}
