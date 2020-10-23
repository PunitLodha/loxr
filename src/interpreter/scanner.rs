use super::error::CodeError;
use super::token::{Token, TokenType};
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

    pub fn scan_tokens(&mut self) -> Result<(), CodeError> {
        let source = self.source.clone();
        let mut characters = source.chars().peekable();
        while let Some(c) = characters.next() {
            self.start = self.current;
            match c {
                '(' => self.add_token(TokenType::LeftParen, None),
                ')' => self.add_token(TokenType::RightParen, None),
                '{' => self.add_token(TokenType::LeftBrace, None),
                '}' => self.add_token(TokenType::RightBrace, None),
                ',' => self.add_token(TokenType::Comma, None),
                '.' => self.add_token(TokenType::Dot, None),
                '-' => self.add_token(TokenType::Minus, None),
                '+' => self.add_token(TokenType::Plus, None),
                ';' => self.add_token(TokenType::Semicolon, None),
                '*' => self.add_token(TokenType::Star, None),
                '!' => match characters.peek() {
                    Some(val) => {
                        if *val == '=' {
                            self.add_token(TokenType::BangEqual, None);
                            characters.next();
                        }
                    }
                    None => self.add_token(TokenType::Bang, None),
                },
                '=' => match characters.peek() {
                    Some(val) => {
                        if *val == '=' {
                            self.add_token(TokenType::EqualEqual, None);
                            characters.next();
                        }
                    }
                    None => self.add_token(TokenType::Equal, None),
                },
                '>' => match characters.peek() {
                    Some(val) => {
                        if *val == '=' {
                            self.add_token(TokenType::GreaterEqual, None);
                            characters.next();
                        }
                    }
                    None => self.add_token(TokenType::Greater, None),
                },
                '<' => match characters.peek() {
                    Some(val) => {
                        if *val == '=' {
                            self.add_token(TokenType::LessEqual, None);
                            characters.next();
                        }
                    }
                    None => self.add_token(TokenType::Less, None),
                },
                '/' => match characters.peek() {
                    Some(val) => {
                        if *val == '/' {
                            while let Some(val) = characters.peek() {
                                if *val != '\n' {
                                    characters.next();
                                }
                            }
                        }
                    }
                    None => self.add_token(TokenType::Slash, None),
                },
                ' ' | '\r' | '\t' => (),
                '\n' => self.line += 1,
                err => {
                    return Err(CodeError::new(
                        self.line,
                        String::new(),
                        format!("Unexpected character {}", err),
                    ))
                }
            };
        }
        let eof = Token::new(TokenType::EOF, String::new(), None, self.line);
        self.tokens.push(eof);
        Ok(())
    }
    fn add_token(&mut self, kind: TokenType, literal: Option<TokenType>) {
        let start = self.start as usize;
        let current = self.current as usize;
        let text = &self.source[start..current];
        let token = Token::new(kind, text.to_owned(), literal, self.line);
        self.tokens.push(token);
    }
}
