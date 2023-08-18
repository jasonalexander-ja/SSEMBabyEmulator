//! # Linker Errors
//! 
//! This module includes all the possible errors that can be thrown 
//! during linking, all error types implement [LinkerError][crate::assembler::linker_errors::LinkerError].
//! 
//! The main type in this module is [LinkingError][crate::assembler::linker_errors::LinkingError]
//! whcih is an enum that has a branch for each possible error that can be thrown 
//! with each branch wrapping the corresponding error type that contains metadata 
//! on that error. 
//! 
//! Any new error types and object should be added to this enum. 
//! 

/// Defines common behaviour for any error thrown by the linker. 
pub trait LinkerError {
    /// Returns a short string describing the error. 
    fn describe(&self) -> String;
}

/// Possible errors thrown when resolving a tag name. 
#[derive(Clone, Debug, PartialEq)]
pub enum TagError {
    /// A tag reference could not be resolved. 
    UnknownTagName(String)
}

impl LinkerError for TagError {
    fn describe(&self) -> String {
        match self {
            TagError::UnknownTagName(s) => format!("The tag reference `{}` is not declared. ", s)
        }
    }
}

/// Possible errors thrown during the linking process. 
#[derive(Debug, PartialEq)]
pub enum LinkingError {
    /// Error thrown during resolving a tag. 
    TagError(TagError)
}

impl LinkerError for LinkingError {
    fn describe(&self) -> String {
        match self {
            LinkingError::TagError(t) => format!("There was an error linking a tag. {}", t.describe())
        }
    }
}
