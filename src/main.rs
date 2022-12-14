use orion::error::OrionError;
use std::{path::PathBuf, fs};
use clap::Parser;
use colored::*;

mod lexer;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The path of the file to compile
    #[arg(short, long)]
    file: Option<PathBuf>
}

fn main() -> Result<(), OrionError>{
    let args = Args::parse();
    let mut contents = String::new();
    let prefix = "[lexer]".green().bold();

    // Check if a path was supplied
    if let Some(file) = args.file {
        // File was passed
        contents = fs::read_to_string(&file).unwrap();
        
        println!("{prefix} {file:?}");
    } else {
        // File was not passed. Eventually I'd like to implement a REPL, however
        // for the time being, I'll just default to the 'examples/main.ori' file
        // to lex/parse/etc..
        contents = fs::read_to_string("examples/main.ori").unwrap();
        
        println!("{prefix} \"examples/main.ori\"");
    }

    // Ensure we have a file
    assert!(!contents.is_empty(), "Unable to read file");

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
