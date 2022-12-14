//! Error types for the Orion language. Used to handle the different errors
//! in the compiler, and how to handle displaying them.

use std::fmt;
use self::OrionError::*;
use colored::*;

#[derive(Debug)]
pub enum OrionError {
    // GENERAL ERRORS
    /// Unimplemented feature
    Unimplemented,
    // LEXER ERRORS
    /// Unknown character was passed to the lexer
    UnknownCharacter(char),
    // PARSER ERRORS
    // ...
}

impl OrionError {
    pub fn message(&self) -> String {
        match *self {
            Unimplemented => "not implemented.".to_string(),
            UnknownCharacter(ref chr) => format!("unknown character: {chr}."),
        }
    }
}

impl fmt::Display for OrionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let prefix = "[ERROR]".red().bold();

        write!(f, "{}: {}", prefix, self.message())
    }
}
