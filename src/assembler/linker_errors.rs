
pub trait LinkerError {
    fn describe(&self) -> String;
}

pub enum TagError {
    UnknownTagName(String)
}

impl LinkerError for TagError {
    fn describe(&self) -> String {
        match self {
            TagError::UnknownTagName(s) => format!("The tag reference `{}` is not declared. ", s)
        }
    }
}

pub enum LinkingError {
    TagError(TagError)
}

impl LinkerError for LinkingError {
    fn describe(&self) -> String {
        match self {
            LinkingError::TagError(t) => format!("There was an error linking a tag. {}", t.describe())
        }
    }
}
