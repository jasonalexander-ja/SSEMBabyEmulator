use super::parser::Instruction;


/// Defines common behaviour for all errors thrown whilst parsing Baby asm. 
pub trait ParseError {
    /// Returns a string describing an error thrown. 
    fn describe(&self) -> String;
}

/// Thrown when an invalid value is encountered. 
pub enum ValueParseError {
    /// No discernable value detected where one is expected. 
    InvalidValue(String),
    /// Invalid hex value. 
    InvalidHex(String),
    /// Invalid decimal value. 
    InvalidDecimal(String),
    /// Invalid octal value. 
    InvalidOctal(String),
    /// Invalid binary value. 
    InvalidBinary(String),
    /// Invalid tag name. 
    InvalidTagName(String),
}

impl ParseError for ValueParseError {
    fn describe(&self) -> String {
        match self {
            ValueParseError::InvalidValue(v) => format!("The value: {}; is an invalid value. ", v),
            ValueParseError::InvalidHex(v) => format!("The value: {}; is invalid hex value. ", v),
            ValueParseError::InvalidDecimal(v) => format!("The value: {}; is invalid decimal value. ", v),
            ValueParseError::InvalidOctal(v) => format!("The value: {}; is invalid octal value. ", v),
            ValueParseError::InvalidBinary(v) => format!("The value: {}; is invalid binary value. ", v),
            ValueParseError::InvalidTagName(v) => format!("The value: {}; is invalid tag name. ", v),
        }
    }
}

/// Thrown when errors are found parsing Baby asm instructions. 
pub enum InstructionError {
    /// A given instruction isn't correct. 
    UnkownInstruction(String),
    /// Attempting to parse a instructions operand threw an error. 
    OperandValueParseError(Instruction, ValueParseError)
}

impl ParseError for InstructionError {
    fn describe(&self) -> String { 
        match self {
            InstructionError::UnkownInstruction(v) => format!("The specified instruction {} is not known. ", v),
            InstructionError::OperandValueParseError(c, v) => format!("Failed to parse operand for {}. {}", c.describe(), v.describe())
        }
    }
}

/// Thrown when an error was encountered parsing an absolute value. 
pub enum AbsoluteError {
    /// Attempt to parse a value to an absolute value encountered an error. 
    ValueError(ValueParseError)
}

impl ParseError for AbsoluteError {
    fn describe(&self) -> String {
        match self {
            AbsoluteError::ValueError(v) => format!("Failed to parse value for absolute value declaration. {}", v.describe())
        }
    }
}

/// Thrown when an error is encountered trying to parse a tag declaration. 
pub enum TagError {
    /// Thrown when a delcared tag name has whitespace. 
    TagNameWhitespace(String)
}

impl ParseError for TagError {
    fn describe(&self) -> String {
        match self {
            TagError::TagNameWhitespace(v) => format!("The tag name `{}` is invalid. ", v)
        }
    }
}

/// Thrown when an error is encountered parsing a Baby asm line. 
pub enum LineParseError {
    /// Thrown when an error is encountered parsing a tag declaration. 
    TagError(TagError),
    /// Thrown when an error is encountered parsing an absolute value declaration. 
    AbsoluteError(AbsoluteError),
    /// Thrown when an error is encountered parsing a instruction use. 
    InstructionError(InstructionError),
}

impl ParseError for LineParseError {
    fn describe(&self) -> String {
        match self {
            LineParseError::TagError(v) => format!("Error parsing a tag line, {}", v.describe()),
            LineParseError::AbsoluteError(v) => format!("Error parsing an absolute value line. {}", v.describe()),
            LineParseError::InstructionError(v) => format!("Error parsing a instruction line. {}", v.describe()),
        }
    }
}
