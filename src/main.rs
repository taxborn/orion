use clap::Parser;
use colored::*;
use orion::error::OrionError;
use std::path::PathBuf;

mod lexer;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The path of the file to compile
    #[arg(short, long)]
    file: Option<PathBuf>,
}

fn main() -> Result<(), OrionError> {
    let args = Args::parse();
    let mut contents = String::new();
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

    let mut lexer = lexer::Lexer::new(contents);
    let tokens = lexer.lex();

    if let Err(error) = lexer.lex() {
        println!("{error}");

        // exit here?
        std::process::exit(-1);
    }

    println!("toks: {:?}", tokens.unwrap());

    Ok(())
}
