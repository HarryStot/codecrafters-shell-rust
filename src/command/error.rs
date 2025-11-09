use std::fmt;

#[derive(Debug)]
pub enum CommandError {
    NotFound(String),
    InvalidArguments(String),
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandError::NotFound(cmd) => write!(f, "{}: command not found", cmd),
            CommandError::InvalidArguments(cmd) => write!(f, "Invalid arguments for command '{}'", cmd),
        }
    }
}

impl std::error::Error for CommandError {}
