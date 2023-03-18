use std::fmt;

#[derive(Debug)]
pub enum FindPastPaperError {
    ServerFailed,
    CouldNotReadFile,
    InputFieldIsEmpty,
    UnsuitableInputLength,
}

impl fmt::Display for FindPastPaperError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FindPastPaperError::ServerFailed => write!(f, "server failed to start"),
            FindPastPaperError::CouldNotReadFile => write!(f, "Could not read file"),
            FindPastPaperError::InputFieldIsEmpty => {
                write!(f, "Please enter a question in the input field")
            }
            FindPastPaperError::UnsuitableInputLength => {
                write!(f, "Input must be over 3 characters")
            }
        }
    }
}
