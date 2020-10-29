use std::error::Error;

use crate::error::CodeError;
use crate::token::{get_keyword_token, LiteralType, Token, TokenType};
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: u32,
    current: u32,
    line: u32,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        let tokens: Vec<Token> = Vec::new();
        Scanner {
            source,
            tokens,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn display_tokens(self) {
        for token in self.tokens.iter() {
            println!("{}", token);
        }
    }

    pub fn scan_tokens(&mut self) -> Result<(), Box<dyn Error>> {
        let source = self.source.clone();
        let mut characters = source.chars().peekable();
        while let Some(c) = characters.next() {
            self.start = self.current;
            match c {
                '(' => self.add_token(TokenType::LeftParen),
                ')' => self.add_token(TokenType::RightParen),
                '{' => self.add_token(TokenType::LeftBrace),
                '}' => self.add_token(TokenType::RightBrace),
                ',' => self.add_token(TokenType::Comma),
                '.' => self.add_token(TokenType::Dot),
                '-' => self.add_token(TokenType::Minus),
                '+' => self.add_token(TokenType::Plus),
                ';' => self.add_token(TokenType::Semicolon),
                '*' => self.add_token(TokenType::Star),
                '!' => match characters.peek() {
                    Some(&val) => {
                        if val == '=' {
                            self.add_token(TokenType::BangEqual);
                            characters.next();
                        }
                    }
                    None => self.add_token(TokenType::Bang),
                },
                '=' => match characters.peek() {
                    Some(&val) => {
                        if val == '=' {
                            self.add_token(TokenType::EqualEqual);
                            characters.next();
                        }
                    }
                    None => self.add_token(TokenType::Equal),
                },
                '>' => match characters.peek() {
                    Some(&val) => {
                        if val == '=' {
                            self.add_token(TokenType::GreaterEqual);
                            characters.next();
                        }
                    }
                    None => self.add_token(TokenType::Greater),
                },
                '<' => match characters.peek() {
                    Some(&val) => {
                        if val == '=' {
                            self.add_token(TokenType::LessEqual);
                            characters.next();
                        }
                    }
                    None => self.add_token(TokenType::Less),
                },
                '/' => match characters.peek() {
                    Some(&val) => {
                        if val == '/' {
                            while let Some(&val) = characters.peek() {
                                if val != '\n' {
                                    characters.next();
                                }
                            }
                        }
                    }
                    None => self.add_token(TokenType::Slash),
                },
                ' ' | '\r' | '\t' => (),
                '\n' => self.line += 1,
                '"' => {
                    let mut string_literal = String::new();
                    loop {
                        match characters.peek() {
                            Some(&val) => {
                                if val != '"' {
                                    if val == '\n' {
                                        self.line += 1
                                    }
                                    string_literal.push(val);
                                    characters.next();
                                } else {
                                    break;
                                }
                            }
                            None => {
                                return Err(Box::new(CodeError::new(
                                    self.line,
                                    String::new(),
                                    format!("Unterminated String"),
                                )))
                            }
                        }
                    }
                    self.add_token(TokenType::Literals(LiteralType::String(string_literal)));
                }
                '0'..='9' => {
                    let mut number_literal = String::new();
                    number_literal.push(c);
                    while let Some(&val) = characters.peek() {
                        if val.is_ascii_digit() {
                            number_literal.push(val);
                            characters.next();
                        } else if val == '.' {
                            number_literal.push(val);
                            characters.next();
                            match characters.peek() {
                                Some(ch) => {
                                    if !ch.is_ascii_digit() {
                                        return Err(Box::new(CodeError::new(
                                            self.line,
                                            String::new(),
                                            format!("Invalid Number"),
                                        )));
                                    }
                                }
                                None => {
                                    return Err(Box::new(CodeError::new(
                                        self.line,
                                        String::new(),
                                        format!("Invalid Number"),
                                    )))
                                }
                            }
                        } else {
                            break;
                        }
                    }
                    let number: f32 = number_literal.parse()?;
                    self.add_token(TokenType::Literals(LiteralType::Integer(number)));
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    let mut identifier = String::new();
                    identifier.push(c);
                    while let Some(&val) = characters.peek() {
                        if val.is_ascii_alphanumeric() {
                            identifier.push(val);
                            characters.next();
                        } else {
                            break;
                        }
                    }
                    let token = get_keyword_token(&identifier);
                    self.add_token(token);
                }
                err => {
                    return Err(Box::new(CodeError::new(
                        self.line,
                        String::new(),
                        format!("Unexpected character {}", err),
                    )))
                }
            };
        }
        let eof = Token::new(TokenType::EOF, String::new(), self.line);
        self.tokens.push(eof);
        Ok(())
    }
    fn add_token(&mut self, kind: TokenType) {
        let start = self.start as usize;
        let current = self.current as usize;
        let text = &self.source[start..current];
        let token = Token::new(kind, text.to_owned(), self.line);
        self.tokens.push(token);
    }
}
