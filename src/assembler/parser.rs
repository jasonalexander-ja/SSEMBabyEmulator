use super::parse_errors::{
    LineParseError,
    TagError,
    AbsoluteError,
    CommandError,
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

    fn parse_hex(v: String) -> Result<Value, ValueParseError> {
        match i32::from_str_radix(&v, 16) {
            Ok(v) => Ok(Value::Value(v)),
            Err(v) => Err(ValueParseError::InvalidHex(v.to_string()))
        }
    }

    fn parse_decimal(v: String) -> Result<Value, ValueParseError> {
        match i32::from_str_radix(&v, 10) {
            Ok(v) => Ok(Value::Value(v)),
            Err(v) => Err(ValueParseError::InvalidDecimal(v.to_string()))
        }
    }

    fn parse_octal(v: String) -> Result<Value, ValueParseError> {
        match i32::from_str_radix(&v, 8) {
            Ok(v) => Ok(Value::Value(v)),
            Err(v) => Err(ValueParseError::InvalidOctal(v.to_string()))
        }
    }

    fn parse_binary(v: String) -> Result<Value, ValueParseError> {
        match i32::from_str_radix(&v, 2) {
            Ok(v) => Ok(Value::Value(v)),
            Err(v) => Err(ValueParseError::InvalidBinary(v.to_string()))
        }
    }

    fn parse_tag_name(v: String) -> Result<Value, ValueParseError> {
        if !v.contains(char::is_whitespace) {
            return Ok(Value::Tag(v))
        }
        Err(ValueParseError::InvalidTagName(v.to_string()))
    }

}

/// Represents all the commands. 
#[derive(Clone)]
pub enum Command {
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

impl Command {

    /// Parses Baby asm commands & operands using modern notation 
    /// 
    /// # Possible Commands 
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
    pub fn parse(command: &str) -> Result<Command, CommandError> {
        let command = command.trim().to_lowercase();
        let v = Value::Value(0);
        match command {

            c if c.starts_with("jmp ") => 
                Self::make_command(Command::Jump(v), c.replace("jmp ", "")),
            c if c.starts_with("jrp ") => 
                Self::make_command(Command::RelativeJump(v), c.replace("jrp ", "")),
            c if c.starts_with("ldn ") => 
                Self::make_command(Command::Negate(v), c.replace("ldn ", "")),
            c if c.starts_with("sto ") => 
                Self::make_command(Command::Store(v), c.replace("sto ", "")),
            c if c.starts_with("sub ") => 
                Self::make_command(Command::Subtract(v), c.replace("sub ", "")),

            c if c.starts_with("cmp") => Ok(Command::Test),
            c if c.starts_with("stp") => Ok(Command::Stop),

            _ => Err(CommandError::UnkownCommand(command.to_string()))
        }
    }

    /// Parses Baby asm commands & operands using original notation 
    /// 
    /// # Possible Commands 
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
    pub fn parse_ogn(command: &str) -> Result<Command, CommandError> {
        let v = Value::Value(0);
        match command {
            c if c.ends_with(", CL") => 
                Self::make_command(Command::Jump(v), c.replace(", Cl", "")),
            c if c.starts_with("Add ") && c.ends_with(", Cl") => 
                Self::make_command(Command::RelativeJump(v), c.replace("Add ", "").replace(", Cl", "")),
            c if c.starts_with("-") && c.ends_with(", C") => 
                Self::make_command(Command::Negate(v), c.replace("-", "").replace(", C", "")),
            c if c.starts_with("c, ") => 
                Self::make_command(Command::Store(v), c.replace("c, ", "")),
            c if c.starts_with("SUB ") => 
                Self::make_command(Command::Subtract(v), c.replace("SUB ", "")),

            c if c.starts_with("Test") => Ok(Command::Test),
            c if c.starts_with("Stop") => Ok(Command::Stop),

            _ => Err(CommandError::UnkownCommand(command.to_string()))
        }
    }

    /// Returns a string with a short description of the command. 
    pub fn describe(&self) -> String {
        match self {
            Command::Jump(_) => "jump".to_owned(),
            Command::RelativeJump(_) => "relative jump".to_owned(),
            Command::Negate(_) => "negate".to_owned(),
            Command::Store(_) => "store".to_owned(),
            Command::Subtract(_) => "subtract".to_owned(),
            Command::Test => "test".to_owned(),
            Command::Stop => "stop".to_owned(),
        }
    }

    /// Returns the stored memory address operand of a command,
    /// returns a 0 if a stop or test.
    pub fn get_operand(&self) -> Value {
        match self {
            Command::Jump(v) => v.clone(),
            Command::RelativeJump(v) => v.clone(),
            Command::Negate(v) => v.clone(),
            Command::Store(v) => v.clone(),
            Command::Subtract(v) => v.clone(),
            Command::Test => Value::Value(0),
            Command::Stop => Value::Value(0),
        }
    }

    fn make_command(c: Command, op: String) -> Result<Command, CommandError> {
        let value = match Value::parse(&op) {
            Ok(v) => v,
            Err(e) => return Err(CommandError::OperandValueParseError(c, e))
        };
        let res = match c {
            Command::Jump(_) => Command::Jump(value),
            Command::RelativeJump(_) => Command::RelativeJump(value),
            Command::Negate(_) => Command::Negate(value),
            Command::Store(_) => Command::Store(value),
            Command::Subtract(_) => Command::Subtract(value),
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
    /// An actual command directive telling the computer to 
    /// perform an action. 
    Command(Command),
}

impl LineType {

    /// Tries tp parse a line of Baby asm 
    /// 
    /// Returns an instance of [LineType] corresponding to the 
    /// line type and metadata. Each line can either be an absolute 
    /// value, a command, or a tag to reference back to a location 
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
            l => if og_notation { Self::parse_command(l) } 
                else { Self::parse_command_ogn(l) },
        }
    }

    fn strip_comments(line: &str) -> String {
        let lines = if let Some(v) = line.split(";").next() { v }
            else { "" };
        lines.to_owned()
    }

    fn parse_tag(tag: String) -> Result<LineType, LineParseError> {
        let tag = tag.trim();
        if tag.contains(char::is_whitespace) {
            return Err(LineParseError::TagError(TagError::TagNameWhitespace(tag.to_string())))
        }
        Ok(LineType::Tag(tag.to_string()))
    }

    fn parse_absolute(tag: String) -> Result<LineType, LineParseError> {
        match Value::parse(&tag) {
            Ok(v) => Ok(LineType::Absolute(v)),
            Err(e) => Err(LineParseError::AbsoluteError(AbsoluteError::ValueError(e)))
        }
    }

    fn parse_command(command: String) -> Result<LineType, LineParseError> {
        match Command::parse(&command) {
            Ok(v) => Ok(LineType::Command(v)),
            Err(e) => Err(LineParseError::CommandError(e))
        }
    }

    fn parse_command_ogn(command: String) -> Result<LineType, LineParseError> {
        match Command::parse_ogn(&command) {
            Ok(v) => Ok(LineType::Command(v)),
            Err(e) => Err(LineParseError::CommandError(e))
        }
    }
}
