
/// Defines common behaviour for any error thrown by the linker.  
pub trait LinkerError {
    /// Returns a short string describing the error. 
    fn describe(&self) -> String;
}

/// Possible errors thrown when resolving a tag name. 
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
