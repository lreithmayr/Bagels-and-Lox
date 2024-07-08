use std::fs;
use clap::{Parser, Subcommand};
use anyhow::Context;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Interpreter command
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Tokenize {
        file: String,
    }
}

// #[derive(Debug)]
// enum Token {
//     EOF,
//     LeftParen,
//     RightParen,
// }


fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    match args.command {
        Command::Tokenize { file } => {
            let file_contents = fs::read_to_string(file).context("failed to read file")?;
            for c in file_contents.chars() {
                match c {
                    '(' => {
                        println!("LEFT_PAREN ( null");
                    }
                    ')' => {
                        println!("RIGHT_PAREN ) null");
                    }
                    '{' => {
                        println!("LEFT_BRACE {{ null");
                    }
                    '}' => {
                        println!("RIGHT_BRACE }} null");
                    }
                    '*' => {
                        println!("STAR * null");
                    }
                    '.' => {
                        println!("DOT . null");
                    }
                    ',' => {
                        println!("COMMA , null");
                    }
                    '+' => {
                        println!("PLUS + null");
                    }
                    '-' => {
                        println!("MINUS - null");
                    }
                    ';' => {
                        println!("SEMICOLON ; null");
                    }
                    _ => {
                        anyhow::bail!("Can't handle this char yet");
                    }
                }
            }
            // End of file or empty file
            println!("EOF  null")
        }
    }
    Ok(())
}

