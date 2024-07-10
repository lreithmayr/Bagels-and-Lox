use std::fmt::{Display, Formatter};
use std::fs;
use std::process::exit;

use anyhow::{Context, Error};
use clap::{Parser, Subcommand};
use strum_macros::Display;

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

#[allow(non_camel_case_types, dead_code, clippy::upper_case_acronyms)]
#[derive(Debug, Display, Eq, PartialEq)]
enum TokenType {
    // Illegal Token
    LEXICAL_ERROR,
    SKIP,

    // Single-character tokens.
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOL,
    EOF,
}

#[derive(Debug)]
struct Token {
    token_type: TokenType,
    lexeme: String,
    line: i32,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.token_type,
            self.lexeme,
            String::from("null")
        )
    }
}

impl Token {
    fn new(token_type: TokenType, lexeme: String, line: i32) -> Self {
        Self {
            token_type,
            lexeme,
            line,
        }
    }
}

struct Lexer {
}

impl Lexer {
    fn lex(file: &str) -> i32 {
        let mut error_code: i32 = 0;
        let mut line_nr: i32 = 1;
        let mut skip: bool = false;
        let mut tokens = Vec::new();

        let mut f_iter = file.chars().peekable();
        while let Some(c) = f_iter.next() {
            let next_char = f_iter.peek();
            let t = Self::scan_char(c, next_char, &mut line_nr, &mut skip);
            tokens.push(t);
        }

        for t in tokens.into_iter().flatten() {
            if t.token_type == TokenType::LEXICAL_ERROR {
                eprintln!(
                    "[line {}] Error: Unexpected character: {}",
                    t.line, t.lexeme
                );
                error_code = 65;
            } else {
                println!("{}", t);
            }
        }
        println!("EOF  null");
        error_code
    }

    fn scan_char(c: char, n: Option<&char>, line_nr: &mut i32, skip: &mut bool) -> Option<Token> {
        if *skip {
            *skip = false;
            return None;
        }
        match c {
            '(' => Some(Token::new(TokenType::LEFT_PAREN, String::from(c), *line_nr)),
            ')' => Some(Token::new(
                TokenType::RIGHT_PAREN,
                String::from(c),
                *line_nr,
            )),
            '{' => Some(Token::new(TokenType::LEFT_BRACE, String::from(c), *line_nr)),
            '}' => Some(Token::new(
                TokenType::RIGHT_BRACE,
                String::from(c),
                *line_nr,
            )),
            '*' => Some(Token::new(TokenType::STAR, String::from(c), *line_nr)),
            '.' => Some(Token::new(TokenType::DOT, String::from(c), *line_nr)),
            ',' => Some(Token::new(TokenType::COMMA, String::from(c), *line_nr)),
            '+' => Some(Token::new(TokenType::PLUS, String::from(c), *line_nr)),
            '-' => Some(Token::new(TokenType::MINUS, String::from(c), *line_nr)),
            ';' => Some(Token::new(TokenType::SEMICOLON, String::from(c), *line_nr)),
            '\n' => {
                *line_nr += 1;
                None
            }
            '=' => {
                if let Some(n) = n {
                    if Self::match_next(*n, '=') {
                        *skip = true;
                        return Some(Token::new(
                            TokenType::EQUAL_EQUAL,
                            String::from("=="),
                            *line_nr,
                        ));
                    }
                }
                Some(Token::new(TokenType::EQUAL, String::from(c), *line_nr))
            }
            _ => Some(Token::new(
                TokenType::LEXICAL_ERROR,
                String::from(c),
                *line_nr,
            )),
        }
    }

    fn match_next(next: char, comp: char) -> bool {
        next == comp
    }
}

fn main() -> Result<(), Error> {
    let args = Args::parse();
    match args.command {
        Command::Tokenize { file } => {
            let file_contents = fs::read_to_string(file).context("failed to read file")?;
            let code = Lexer::lex(&file_contents);
            exit(code);
        }
    }
}
