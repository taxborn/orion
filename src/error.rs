//! Error types for the Orion language. Used to handle the different errors
//! in the compiler, and how to handle displaying them.

use self::OrionError::*;
use colored::*;

#[derive(Debug)]
pub enum OrionError {
    // GENERAL ERRORS
    /// Unimplemented feature
    Unimplemented,
    IOError,
    // LEXER ERRORS
    /// Unknown character was passed to the lexer
    UnknownCharacter(char),
    // PARSER ERRORS
    // ...
}

/// Implementing the error trait for Orion's custom error
impl std::error::Error for OrionError {}

/// For now, generalize all std::io errors to the OrionError::IOError error
impl From<std::io::Error> for OrionError {
    fn from(_: std::io::Error) -> Self {
        OrionError::IOError
    }
}

impl OrionError {
    /// Get the message for each error, saying what happened
    pub fn message(&self) -> String {
        match self {
            Unimplemented => "not implemented.".to_string(),
            IOError => "file not found.".to_string(),
            UnknownCharacter(chr) => format!("unknown character: {chr}."),
        }
    }
}

impl std::fmt::Display for OrionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let prefix = "[ERROR]".red().bold();

        write!(f, "{prefix}: {}", self.message())
    }
}
