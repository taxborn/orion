use clap::Parser as ClapParser;
use colored::*;
use orion::error::OrionError;
use orion::lexer::state::Lexer;
use orion::parser::state::Parser;
use std::path::PathBuf;

#[derive(ClapParser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The path of the file to compile
    #[arg(short, long)]
    file: Option<PathBuf>,
    /// Whether the compiler should emit statistics about
    #[arg(short, long)]
    tokens: bool,

    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<(), OrionError> {
    let contents;
    let args = Args::parse();
    let prefix = "[Orion]".purple().bold();

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

    if args.tokens {
        orion::print_tokens(&mut lexer, args.verbose)?;
    }

    let mut parser = Parser::new(lexer);
    let mut stmts = parser.parse();

    println!("{stmts:?}");

    Ok(())
}
