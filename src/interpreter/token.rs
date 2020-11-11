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
    Literals(LiteralType),

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

#[derive(Debug)]
pub enum LiteralType {
    String(String),
    Integer(f32),
    Identifier(String),
}
pub struct Token {
    kind: TokenType,
    line: u32,
}

impl Token {
    pub fn new(kind: TokenType, line: u32) -> Token {
        Token { kind, line }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.kind)
    }
}

pub fn get_keyword_token(keyword: &str) -> TokenType {
    match keyword {
        "and" => TokenType::And,
        "class" => TokenType::Class,
        "else" => TokenType::Else,
        "false" => TokenType::False,
        "for" => TokenType::For,
        "fun" => TokenType::Fun,
        "if" => TokenType::If,
        "nil" => TokenType::Nil,
        "or" => TokenType::Or,
        "print" => TokenType::Print,
        "return" => TokenType::Return,
        "super" => TokenType::Super,
        "this" => TokenType::This,
        "true" => TokenType::True,
        "var" => TokenType::Var,
        "while" => TokenType::While,
        value => TokenType::Literals(LiteralType::Identifier(value.to_string())),
    }
}
