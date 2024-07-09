use anyhow::Context;
use clap::{Parser, Subcommand};
use std::fs;
use thiserror::Error;
use std::process::{exit, ExitCode};

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

// #[derive(Error, Debug)]
// enum ParseError {
//     #[error()]
//     UnexpectedCharacter {
//         char: String
//     }
// }

fn main() {
    let args = Args::parse();
    let mut code: i32 = 0;
    match args.command {
        Command::Tokenize { file } => {
            let file_contents = fs::read_to_string(file).unwrap();
            for l in file_contents.lines().enumerate() {
                for c in l.1.chars() {
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
                            eprintln!("[line {}] Error: Unexpected character: {}", l.0 + 1, c);
                            code = 65;
                        }

                    }
                }
            }
            // End of file or empty file
            println!("EOF  null")
        }
    }
    exit(code)
}
