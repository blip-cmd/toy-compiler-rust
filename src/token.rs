#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Int(i64),
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
    Eof,
}

#[derive(Debug, PartialEq)]
pub struct Token{
    pub kind: TokenKind,
    pub line: usize,
    pub col: usize,
}

impl Token{
    pub fn new(kind: TokenKind, line: usize, col: usize) -> Self {
        Self { kind, line, col }
    }
}