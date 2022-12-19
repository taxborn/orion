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

fn main() -> Result<(), OrionError<'static>> {
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

    match lexer.lex() {
        Ok(tokens) => {
            println!("{}", "tokens:".green().bold());
            for token in tokens {
                println!("{token}");
            }
        }
        Err(error) => {
            println!("{error}");

            // exit here?
            std::process::exit(-1);
        }
    }

    Ok(())
}
