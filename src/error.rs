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
pub enum OrionError {
    // GENERAL ERRORS
    /// Unimplemented feature
    Unimplemented,
    /// Input/Output error
    IOError,
    // LEXER ERRORS
    /// General Lexer error
    LexerError,
    /// Unterminated quote for a string. e.g. `let a := "abc..`
    UnterminatedQuote,
    /// Unclosed multiline comment. e.g. `/* this is a comment without a close.`
    UnclosedMultilineComment,
    // PARSER ERRORS
    // ...
}

/// Implementing the error trait for Orion's custom error
impl error::Error for OrionError {}

/// For now, generalize all std::io errors to the OrionError::IOError error
impl From<io::Error> for OrionError {
    fn from(_: io::Error) -> Self {
        OrionError::IOError
    }
}

impl OrionError {
    /// Get the message for each error, saying what happened
    pub fn message(&self) -> String {
        match self {
            Unimplemented => "not implemented.".to_string(),
            IOError => "file not found.".to_string(),
            LexerError => "there was an error in lexing.".to_string(),
            UnterminatedQuote => "There was an unterminated quote.".to_string(),
            UnclosedMultilineComment => "A multiline comment was unclosed.".to_string(),
        }
    }
}

impl Display for OrionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let prefix = "[ERROR]".red().bold();

        write!(f, "{prefix}: {}", self.message())
    }
}
