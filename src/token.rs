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