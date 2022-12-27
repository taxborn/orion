pub mod error;
pub mod lexer;
pub mod parser;

use crate::error::*;
use crate::lexer::state::*;
use crate::lexer::tokens::*;
use colored::*;

pub fn print_tokens<'a>(lexer: &mut Lexer<'a>, verbose: bool) -> Result<(), OrionError> {
    let mut tokens: Vec<Token<'a>> = lexer.collect();

    // Lexer implements Iterator, so we can loop over all tokens.
    for token in lexer.by_ref() {
        tokens.push(token);
    }

    let prefix = "[Orion - Lexer]".purple().bold();
    println!("{prefix} found {} tokens", tokens.len());

    if verbose {
        for token in tokens {
            println!("{token:?}");
        }
    }

    // Check if there were any errors during lexing
    if lexer.error {
        println!("{}", "there was an error in lexing.".red().bold());
        // Maybe eventually I'll implement utilizing OrionError in the lexer,
        // for now, I want to get things moving.
        // https://users.rust-lang.org/t/handling-errors-from-iterators/2551/7
        return Err(OrionError::Unimplemented);
    }

    println!("{}", "lexing successful.".white().bold());

    Ok(())
}
