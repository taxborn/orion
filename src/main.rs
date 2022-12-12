use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The path of the file to compile
    #[arg(short, long)]
    path: Option<PathBuf>
}

fn main() {
    let args = Args::parse();
    let mut contents = String::new();

    // Check if a path was supplied
    if let Some(file) = args.path {
        // File was passed
        contents = std::fs::read_to_string(file).unwrap();
    } else {
        // File was not passed. Eventually I'd like to implement a REPL, however
        // for the time being, I'll just default to the 'examples/main.ori' file
        // to lex/parse/etc..
        contents = std::fs::read_to_string("examples/main.ori").unwrap();
    }

    println!("{contents}");
}
