use std::fmt::{Display, Formatter};

use strum_macros::Display;

#[allow(non_camel_case_types, dead_code, clippy::upper_case_acronyms)]
#[derive(Debug, Display, Eq, PartialEq, Ord, PartialOrd)]
pub enum TokenType {
    // Errors
    LEXICAL_ERROR, // Implemented
    UNTERM_STR,

    // Single-character tokens.
    LEFT_PAREN,  // Implemented
    RIGHT_PAREN, // Implemented
    LEFT_BRACE,  // Implemented
    RIGHT_BRACE, // Implemented
    COMMA,       // Implemented
    DOT,         // Implemented
    MINUS,       // Implemented
    PLUS,        // Implemented
    SEMICOLON,   // Implemented
    SLASH,       // Implemented
    STAR,        // Implemented

    // One or two character tokens.
    BANG,          // Implemented
    BANG_EQUAL,    // Implemented
    EQUAL,         // Implemented
    EQUAL_EQUAL,   // Implemented
    GREATER,       // Implemented
    GREATER_EQUAL, // Implemented
    LESS,          // Implemented
    LESS_EQUAL,    // Implemented
    QMARK,

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

    EOF,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub appendix: String,
    pub line: i32,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.token_type,
            self.lexeme,
            self.appendix,
        )
    }
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, appendix:String, line: i32) -> Self {
        Self {
            token_type,
            lexeme,
            appendix,
            line,
        }
    }
}
