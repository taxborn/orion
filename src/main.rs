use clap::Parser;
use colored::*;
use orion::error::OrionError;
use orion::lexer::lexer::Lexer;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The path of the file to compile
    #[arg(short, long)]
    file: Option<PathBuf>,
}

fn main() -> Result<(), OrionError> {
    let contents;
    let args = Args::parse();
    let prefix = "[lexer]".green().bold();

    // Check if a path was supplied
    match args.file {
        Some(file) => {
            // File was passed
            match std::fs::read_to_string(&file) {
                Ok(content) => {
                    contents = content;
                    println!("{prefix} {file:?}");
                }
                Err(error) => {
                    println!("{}", OrionError::from(error));

                    // exit here?
                    std::process::exit(-1);
                }
            }
        }
        None => {
            // File was not passed. Eventually I'd like to implement a REPL, however
            // for the time being, I'll just default to the 'examples/main.ori' file
            // to lex/parse/etc..
            contents = std::fs::read_to_string("examples/main.ori").unwrap();

            println!("{prefix} \"examples/main.ori\"");
        }
    }

    let mut lexer = Lexer::new(&contents);

    println!("{}", "[tokens found]:".green().bold());
    // Lexer implements Iterator, so we can loop over all tokens.
    let mut count = 0;
    for token in lexer.by_ref() {
        count += 1;
        println!("{token:?}");
    }
    println!("{count} tokens found.");


    // Check if there were any errors during lexing
    if lexer.error {
        println!("{}", "there was an error in lexing.".red().bold());
        // Maybe eventually I'll implement utilizing OrionError in the lexer,
        // for now, I want to get things moving.
        // https://users.rust-lang.org/t/handling-errors-from-iterators/2551/7
        return Err(OrionError::Unimplemented);
    } else {
        println!("{}", "lexing successful.".white().bold());
    }

    Ok(())
}
