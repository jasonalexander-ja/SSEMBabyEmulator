//! # Assembler Errors
//! 
//! This is a simple helper module containing the type [AssemblyError][crate::assembler::errors::AssemblyError],
//! that can contain all the errors thrown during the assembling 
//! process. 
//! 
//! Since this assembler is quite simple and simply takes the 
//! output of the parser and feeds it into the linker, returning
//! the resultant machine code, this enum just has 2 values, 
//! [AssemblyError::ParserError][crate::assembler::errors::AssemblyError::LinkerError] & [AssemblyError::LinkerError][crate::assembler::errors::AssemblyError::LinkerError].
//! 
//! These are wrappers for the overarching error types exported by 
//! [assembler::parse_errors][crate::assembler::parse_errors] & [assembler::linker_errors][crate::assembler::linker_errors].
//! These represent the errors thrown by the parsing and linking. 
//! 
//! For simple debug purposes, [crate::assembler::errors::AssemblyError::describe]
//! can be used to simply log any error to the console. 
//! 
//! # Example 
//! ```
//! use baby_emulator::assembler::assemble;
//! 
//! fn assemble_and_run(asm: String) {
//!     let instructions = match assemble(&asm, false) {
//!         Ok(v) => (),
//!         Err(e) => { println!("{}", e.describe(true)); return; }
//!     };
//! }
//! ```
//! 

use super::linker::linker_errors::{LinkingError, LinkerError};
use super::parser::parse_errors::{LineParseError, ParseError}; 


/// Possble errors thrown at parts of the assembly process. 
pub enum AssemblyError {
    /// Error parsing a line (line no, error). 
    ParserError(usize, LineParseError),
    /// Error thrown during linking. 
    LinkerError(LinkingError),
}

impl AssemblyError {

    /// Returns a string describing the error. 
    /// 
    /// # Parameters
    /// * `line_breaks` - Add in line breaks between each embedded error. 
    /// 
    pub fn describe(&self, line_breaks: bool) -> String {
        match self {
            AssemblyError::ParserError(i, p) => format!("An error was thrown during parsing at line {}. {}", i, p.describe(line_breaks)),
            AssemblyError::LinkerError(l) => format!("An error was thrown during linking. {}", l.describe(line_breaks)),
        }
    }
}
