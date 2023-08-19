//! # Parse Errors 
//! 
//! This module includes all the possible errors that can be thrown 
//! during parsing, all error types implement [ParseError][crate::assembler::parse_errors::ParseError].
//! 
//! The main type in this module is [LineParseError][crate::assembler::parse_errors::LineParseError]
//! whcih is an enum that has a branch for each possible error that can be thrown 
//! with each branch wrapping the corresponding error type that contains metadata 
//! on that error. 
//! 
//! Any new error types and object should be added to this enum. 
//! 


use super::Instruction;


/// Defines common behaviour for all errors thrown whilst parsing Baby asm. 
pub trait ParseError {
    /// Returns a string describing an error thrown. 
    /// 
    /// # Parameters
    /// * `line_breaks` - Add in line breaks between each embedded error. 
    /// 
    fn describe(&self, line_breaks: bool) -> String;
}

/// Thrown when an invalid value is encountered. 
#[derive(PartialEq, Debug)]
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
    fn describe(&self, _line_breaks: bool) -> String {
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
#[derive(PartialEq, Debug)]
pub enum InstructionError {
    /// A given instruction isn't correct. 
    UnkownInstruction(String),
    /// Attempting to parse a instructions operand threw an error. 
    OperandValueParseError(Instruction, ValueParseError)
}

impl ParseError for InstructionError {
    fn describe(&self, line_breaks: bool) -> String { 
        let line_break = if line_breaks { "\n" } else { "" };
        match self {
            InstructionError::UnkownInstruction(v) => format!("The specified instruction {} is not known. ", v),
            InstructionError::OperandValueParseError(c, v) => format!("Failed to parse operand for {}. {line_break} {}", c.describe(), v.describe(line_breaks))
        }
    }
}

/// Thrown when an error was encountered parsing an absolute value. 
#[derive(PartialEq, Debug)]
pub enum AbsoluteError {
    /// Attempt to parse a value to an absolute value encountered an error. 
    ValueError(ValueParseError)
}

impl ParseError for AbsoluteError {
    fn describe(&self, line_breaks: bool) -> String {
        let line_break = if line_breaks { "\n" } else { "" };
        match self {
            AbsoluteError::ValueError(v) => format!("Failed to parse value for absolute value declaration. {line_break} {}", v.describe(line_breaks))
        }
    }
}

/// Thrown when an error is encountered trying to parse a tag declaration. 
#[derive(PartialEq, Debug)]
pub enum TagError {
    /// Thrown when a delcared tag name has whitespace. 
    TagNameWhitespace(String)
}

impl ParseError for TagError {
    fn describe(&self, _line_breaks: bool) -> String {
        match self {
            TagError::TagNameWhitespace(v) => format!("The tag name `{}` is invalid. ", v)
        }
    }
}

/// Thrown when an error is encountered parsing a Baby asm line. 
#[derive(PartialEq, Debug)]
pub enum LineParseError {
    /// Thrown when an error is encountered parsing a tag declaration. 
    TagError(TagError),
    /// Thrown when an error is encountered parsing an absolute value declaration. 
    AbsoluteError(AbsoluteError),
    /// Thrown when an error is encountered parsing a instruction use. 
    InstructionError(InstructionError),
}

impl ParseError for LineParseError {
    fn describe(&self, line_breaks: bool) -> String {
        let line_break = if line_breaks { "\n" } else { "" };
        match self {
            LineParseError::TagError(v) => format!("Error parsing a tag line, {}", v.describe(line_breaks)),
            LineParseError::AbsoluteError(v) => format!("Error parsing an absolute value line. {line_break} {}", v.describe(line_breaks)),
            LineParseError::InstructionError(v) => format!("Error parsing a instruction line. {line_break} {}", v.describe(line_breaks)),
        }
    }
}
