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
    pub fn scan_tokens(&mut self) {
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
                _ => (),
            }
        }
        let eof = Token::new(TokenType::EOF, String::new(), None, self.line);
        self.tokens.push(eof);
    }
    fn add_token(&mut self, kind: TokenType, literal: Option<TokenType>) {
        let start = self.start as usize;
        let current = self.current as usize;
        let text = &self.source[start..current];
        let token = Token::new(kind, text.to_owned(), literal, self.line);
        self.tokens.push(token);
    }
}
