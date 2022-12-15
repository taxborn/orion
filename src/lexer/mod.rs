//! Entrypoint for the lexer portion of the Orion compiler

pub mod lexer;
pub mod tokens;

fn is_whitespace(chr: char) -> bool {
    matches!(chr, ' ' | '\r' | '\t')
}
