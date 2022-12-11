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

    println!("args: {args:?}");
}
