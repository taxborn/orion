//! Error types for the Orion language. Used to handle the different errors
//! in the compiler, and how to handle displaying them.

use std::{
    error,
    fmt::{Display, Formatter, Result},
    io,
};

use self::OrionError::*;
use colored::*;

#[derive(Debug)]
pub enum OrionError<'a> {
    // GENERAL ERRORS
    /// Unimplemented feature
    Unimplemented,
    IOError,
    // LEXER ERRORS
    /// Unknown character was passed to the lexer
    UnknownSlice(&'a str),
    // PARSER ERRORS
    // ...
}

/// Implementing the error trait for Orion's custom error
impl<'a> error::Error for OrionError<'a> {}

/// For now, generalize all std::io errors to the OrionError::IOError error
impl<'a> From<io::Error> for OrionError<'a> {
    fn from(_: io::Error) -> Self {
        OrionError::IOError
    }
}

impl<'a> OrionError<'a> {
    /// Get the message for each error, saying what happened
    pub fn message(&self) -> String {
        match self {
            Unimplemented => "not implemented.".to_string(),
            IOError => "file not found.".to_string(),
            UnknownSlice(slice) => format!("unknown slice: {slice}."),
        }
    }
}

impl<'a> Display for OrionError<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let prefix = "[ERROR]".red().bold();

        write!(f, "{prefix}: {}", self.message())
    }
}
