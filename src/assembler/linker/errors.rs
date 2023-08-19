//! # Linker Errors
//! 
//! This module includes all the possible errors that can be thrown 
//! during linking, all error types implement [LinkerError][crate::assembler::linker::errors::LinkerError].
//! 
//! The main type in this module is [LinkingError][crate::assembler::linker::errors::LinkingError]
//! whcih is an enum that has a branch for each possible error that can be thrown 
//! with each branch wrapping the corresponding error type that contains metadata 
//! on that error. 
//! 
//! Any new error types and object should be added to this enum. 
//! 

use crate::core::MEMORY_WORDS;


/// Defines common behaviour for any error thrown by the linker. 
pub trait LinkerError {
    /// Returns a short string describing the error. 
    /// 
    /// # Parameters
    /// * `line_breaks` - Add in line breaks between each embedded error. 
    /// 
    fn describe(&self, line_breaks: bool) -> String;
}

/// Possible errors thrown when resolving a tag name. 
#[derive(Clone, Debug, PartialEq)]
pub enum TagError {
    /// A tag reference could not be resolved. 
    UnknownTagName(String)
}

impl LinkerError for TagError {
    fn describe(&self, _line_breaks: bool) -> String {
        match self {
            TagError::UnknownTagName(s) => format!("The tag reference `{}` is not declared. ", s)
        }
    }
}

/// The linked program stack is greater than the Baby's memory. 
#[derive(Clone, Debug, PartialEq)]
pub struct MemoryExceedingError {
    /// The number of words in the linked program stack.  
    pub linked_size: usize
}

impl LinkerError for MemoryExceedingError {
    fn describe(&self, _line_breaks: bool) -> String {
        format!("The linked program stack is `{}` words in length, maximum {MEMORY_WORDS}. ", self.linked_size)
    }
}

/// Possible errors thrown during the linking process. 
#[derive(Debug, PartialEq)]
pub enum LinkingError {
    /// Error thrown during resolving a tag. 
    TagError(TagError),
    /// The linked program stack is greater than the Baby's memory. 
    MemoryExceedingError(MemoryExceedingError)
}

impl LinkerError for LinkingError {
    fn describe(&self, line_breaks: bool) -> String {
        let line_break = if line_breaks { "\n" } else { "" };
        match self {
            LinkingError::TagError(t) => format!("There was an error linking a tag. {line_break} {}", t.describe(line_breaks)),
            LinkingError::MemoryExceedingError(m) => format!("There was an error positiong the program. {line_break} {}", m.describe(line_breaks))
        }
    }
}
