use std::fmt;
#[derive(Debug)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    // End of file
    EOF,
}

pub struct Token {
    kind: TokenType,
    lexeme: String,
    literal: Option<TokenType>,
    line: u32,
}

impl Token {
    pub fn new(kind: TokenType, lexeme: String, literal: Option<TokenType>, line: u32) -> Token {
        Token {
            kind,
            lexeme,
            literal,
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.literal {
            Some(val) => write!(f, "{:?} {} {:?}", self.kind, self.lexeme, val),
            None => write!(f, "{:?} {}", self.kind, self.lexeme),
        }
    }
}
