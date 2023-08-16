use super::linker_errors::{LinkingError, LinkerError};
use super::parse_errors::{LineParseError, ParseError}; 


/// Possble errors thrown at parts of the assembly process. 
pub enum AssemblyError {
    /// Error parsing a line (line no, error). 
    ParserError(usize, LineParseError),
    /// Error thrown during linking. 
    LinkerError(LinkingError),
}

impl AssemblyError {

    /// Returns a string describing the error. 
    pub fn describe(&self) -> String {
        match self {
            AssemblyError::ParserError(i, p) => format!("An error was thrown during parsing at line {}. {}", i, p.describe()),
            AssemblyError::LinkerError(l) => format!("An error was thrown during linking. {}", l.describe()),
        }
    }
}
