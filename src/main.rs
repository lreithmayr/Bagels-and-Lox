use interpreter_starter_rust::lexer::Lexer;

use std::fs;
use std::process::exit;

use anyhow::{Context, Error};
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Interpreter command
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Tokenize { file: String },
}

fn main() -> anyhow::Result<(), Error> {
    let args = Args::parse();
    let mut lexer: Lexer = Default::default();
    match args.command {
        Command::Tokenize { file } => {
            let file_contents = fs::read_to_string(file).context("failed to read file")?;
            let code = lexer.lex(&file_contents);
            exit(code);
        }
    }
}
