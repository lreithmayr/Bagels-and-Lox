use crate::token::{Token, TokenType};

pub struct Lexer {
    line_nr: i32,
}

impl Default for Lexer {
    fn default() -> Self {
        Self { line_nr: 1 }
    }
}

impl Lexer {
    pub fn lex(&mut self, file: &str) -> i32 {
        let mut tokens = Vec::new();

        let mut f_iter = file.chars().peekable();
        while let Some(c) = f_iter.next() {
            let next_char = f_iter.peek();
            if let Some(t) = Self::scan_char(self, c) {
                match t.token_type {
                    TokenType::EQUAL if next_char == Some(&'=') => {
                        tokens.push(Token::new(
                            TokenType::EQUAL_EQUAL,
                            String::from("=="),
                            String::from("null"),
                            self.line_nr,
                        ));
                        // Skip the next character
                        f_iter.nth(0);
                    }
                    TokenType::BANG if next_char == Some(&'=') => {
                        tokens.push(Token::new(
                            TokenType::BANG_EQUAL,
                            String::from("!="),
                            String::from("null"),
                            self.line_nr,
                        ));
                        // Skip the next character
                        f_iter.nth(0);
                    }
                    TokenType::GREATER if next_char == Some(&'=') => {
                        tokens.push(Token::new(
                            TokenType::GREATER_EQUAL,
                            String::from(">="),
                            String::from("null"),
                            self.line_nr,
                        ));
                        // Skip the next character
                        f_iter.nth(0);
                    }
                    TokenType::LESS if next_char == Some(&'=') => {
                        tokens.push(Token::new(
                            TokenType::LESS_EQUAL,
                            String::from("<="),
                            String::from("null"),
                            self.line_nr,
                        ));
                        // Skip the next character
                        f_iter.nth(0);
                    }
                    TokenType::SLASH if next_char == Some(&'/') => {
                        if !f_iter.any(|c| c == '\n') {
                            f_iter.by_ref().last();
                        }
                        // Increment the line number manually, since iter.any() eats the newline
                        // character
                        self.line_nr += 1;
                    }
                    TokenType::QMARK => {
                        let mut string = String::new();
                        while let c = f_iter.next() {
                            match c {
                                Some(c) => {
                                    if c == '"' {
                                        let mut lexeme = string.clone();
                                        lexeme.insert(0, '"');
                                        lexeme.push('"');
                                        tokens.push(Token::new(
                                            TokenType::STRING,
                                            lexeme,
                                            string,
                                            self.line_nr,
                                        ));
                                        break;
                                    }
                                    string.push(c);
                                }
                                None => {
                                    tokens.push(Token::new(
                                        TokenType::UNTERM_STR,
                                        String::from(""),
                                        String::from(""),
                                        self.line_nr,
                                    ));
                                    break;
                                }
                            }
                        }
                    }
                    _ => tokens.push(t),
                }
            }
        }
        Self::print_lex(&mut tokens)
    }

    fn scan_char(&mut self, c: char) -> Option<Token> {
        match c {
            '(' => Some(Token::new(
                TokenType::LEFT_PAREN,
                String::from(c),
                String::from("null"),
                self.line_nr,
            )),
            ')' => Some(Token::new(
                TokenType::RIGHT_PAREN,
                String::from(c),
                String::from("null"),
                self.line_nr,
            )),
            '{' => Some(Token::new(
                TokenType::LEFT_BRACE,
                String::from(c),
                String::from("null"),
                self.line_nr,
            )),
            '}' => Some(Token::new(
                TokenType::RIGHT_BRACE,
                String::from(c),
                String::from("null"),
                self.line_nr,
            )),
            '*' => Some(Token::new(
                TokenType::STAR,
                String::from(c),
                String::from("null"),
                self.line_nr,
            )),
            '.' => Some(Token::new(
                TokenType::DOT,
                String::from(c),
                String::from("null"),
                self.line_nr,
            )),
            ',' => Some(Token::new(
                TokenType::COMMA,
                String::from(c),
                String::from("null"),
                self.line_nr,
            )),
            '+' => Some(Token::new(
                TokenType::PLUS,
                String::from(c),
                String::from("null"),
                self.line_nr,
            )),
            '-' => Some(Token::new(
                TokenType::MINUS,
                String::from(c),
                String::from("null"),
                self.line_nr,
            )),
            ';' => Some(Token::new(
                TokenType::SEMICOLON,
                String::from(c),
                String::from("null"),
                self.line_nr,
            )),
            '=' => Some(Token::new(
                TokenType::EQUAL,
                String::from(c),
                String::from("null"),
                self.line_nr,
            )),
            '!' => Some(Token::new(
                TokenType::BANG,
                String::from(c),
                String::from("null"),
                self.line_nr,
            )),
            '<' => Some(Token::new(
                TokenType::LESS,
                String::from(c),
                String::from("null"),
                self.line_nr,
            )),
            '>' => Some(Token::new(
                TokenType::GREATER,
                String::from(c),
                String::from("null"),
                self.line_nr,
            )),
            '/' => Some(Token::new(
                TokenType::SLASH,
                String::from(c),
                String::from("null"),
                self.line_nr,
            )),
            '"' => Some(Token::new(
                TokenType::QMARK,
                String::from(c),
                String::from("null"),
                self.line_nr,
            )),
            '\n' => {
                self.line_nr += 1;
                None
            }
            // Whitespace
            ' ' | '\t' | '\r' => None,
            _ => Some(Token::new(
                TokenType::LEXICAL_ERROR,
                String::from(c),
                String::from("null"),
                self.line_nr,
            )),
        }
    }

    fn print_lex(tokens: &mut [Token]) -> i32 {
        let mut error_code: i32 = 0;

        tokens
            .iter()
            .filter(|t| t.token_type == TokenType::LEXICAL_ERROR)
            .for_each(|t| {
                eprintln!(
                    "[line {}] Error: Unexpected character: {}",
                    t.line, t.lexeme
                );
                error_code = 65;
            });

        tokens
            .iter()
            .filter(|t| t.token_type != TokenType::LEXICAL_ERROR)
            .for_each(|t| {
                if t.token_type == TokenType::UNTERM_STR {
                    eprintln!("[line {}] Error: Unterminated string.", t.line);
                    error_code = 65;
                } else {
                    println!("{}", t);
                }
            });

        println!("EOF  null");
        error_code
    }
}
