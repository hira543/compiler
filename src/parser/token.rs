use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    I32(i32),
    I64(i64),
    String(String),
    Ident(String),                   // identifier
    TypeDeclaration(String, String), // x:i32
    Assignment,
    Plus,
    Minus,
    Star,
    Slash,
    Modulo,
    LessThan,
    GreaterThan,
    DoubleEqual,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Semicolon,
    Colon,
    Comma,
    Function,
    If,
    Else,
    While,
    Print,
    Return,
    EOF,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::I32(value) => write!(f, "Int({})", value),
            Token::I64(value) => write!(f, "Int({})", value),
            Token::String(value) => write!(f, "String(\"{}\")", value),
            Token::Ident(value) => write!(f, "Ident({})", value),
            Token::TypeDeclaration(name, ty) => write!(f, "TypeDeclaration({}, {})", name, ty),
            Token::Assignment => write!(f, "Assignment"),
            Token::Plus => write!(f, "Plus"),
            Token::Minus => write!(f, "Minus"),
            Token::Star => write!(f, "Star"),
            Token::Slash => write!(f, "Slash"),
            Token::Modulo => write!(f, "Modulo"),
            Token::LessThan => write!(f, "LessThan"),
            Token::GreaterThan => write!(f, "GreaterThan"),
            Token::DoubleEqual => write!(f, "DoubleEqual"),
            Token::LParen => write!(f, "LParen"),
            Token::RParen => write!(f, "RParen"),
            Token::LBrace => write!(f, "LBrace"),
            Token::RBrace => write!(f, "RBrace"),
            Token::Semicolon => write!(f, "Semicolon"),
            Token::Colon => write!(f, "Colon"),
            Token::Comma => write!(f, "Comma"),
            Token::Function => write!(f, "Function"),
            Token::If => write!(f, "If"),
            Token::Else => write!(f, "Else"),
            Token::While => write!(f, "While"),
            Token::Print => write!(f, "Print"),
            Token::Return => write!(f, "Return"),
            Token::EOF => write!(f, "EOF"),
        }
    }
}
