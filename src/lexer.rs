use crate::token::{Token,TokenKind};

pub struct Lexer{
    source: Vec<char>,
    pos: usize,
    pub line: usize,
    pub col: usize,
}

#[derive(Debug, PartialEq)]
pub struct CompileError{
    pub message: String,
    pub line: usize,
    pub col: usize,
}

impl Lexer{
    pub fn new(input: &str) -> Self{
        Self{
            source: input.chars().collect(),
            pos: 0,
            line: 1, //1-based indexing for line numbers
            col: 1,
        }
    }
    pub fn peek(&self) -> Option<char>{
        if self.pos >= self.source.len() {
            None
        } else {
            Some(self.source[self.pos])
        }
    }
    pub fn advance(&mut self) -> Option<char>{
        if self.pos >= self.source.len() {
            None
        } else {
            let ch = self.source[self.pos];
            self.pos += 1;
            if ch == '\n'{
                self.line += 1;
                self.col = 1;
            } else {
                self.col += 1;
            }
            Some(ch) //returns an Option<char> containing the character that was advanced over
            //
        }
    }
    fn skip_whitespace(&mut self){
        while let Some(ch) = self.peek(){
            if ch.is_whitespace(){
                self.advance();
            } else {
                break;
            }
        }
    }

    fn lex_number(&mut self, first_digit: char, start_line:usize, start_col: usize) -> Result<Token, CompileError>{
        let mut digits = String::new();
        digits.push(first_digit);

        while let Some(ch) = self.peek(){
            if ch.is_ascii_digit(){
                digits.push(self.advance().unwrap());
            } else {
                break;
            }
        }
        match digits.parse::<i64>(){
            Ok(value) => Ok(Token::new(TokenKind::Int(value), start_line,start_col)),
            Err(_) => Err(CompileError{
                message: format!("invalid integer literal: '{}' is out of range for i64",digits),
                line:start_line,
                col:start_col,
            }),
        }
    }
    pub fn next_token(&mut self) -> Result<Token,CompileError>{
        self.skip_whitespace();
        let start_line = self.line;
        let start_col = self.col;

        let ch = match self.advance(){
            Some(c) => c,
            None => return Ok(Token::new(TokenKind::Eof, start_line, start_col)),
        };

        let kind = match ch{
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Star,
            '/' => TokenKind::Slash,
            '(' => TokenKind::LParen,
            ')' => TokenKind::RParen,
            ch if ch.is_ascii_digit() => return self.lex_number(ch,start_line, start_col),
            unexpected => {
                return Err(CompileError{
                    message: format!("unexpected character: {}", unexpected),
                    line: start_line,
                    col: start_col,
                });
            }
        };


        Ok(Token::new(kind, start_line, start_col))

    }

    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_new() {
        let lexer = Lexer::new("Hello, World!");
        assert_eq!(lexer.source, vec!['H', 'e', 'l', 'l', 'o', ',', ' ', 'W', 'o', 'r', 'l', 'd', '!']);
        assert_eq!(lexer.pos, 0);
        assert_eq!(lexer.line, 1);
        assert_eq!(lexer.col, 1);
    }

    #[test]
    fn test_lexer_peek() {
        let lexer = Lexer::new("Hello");
        assert_eq!(lexer.peek(), Some('H'));
    }

    #[test]
    fn test_lexer_advance() {
        let mut lexer = Lexer::new("Hello");
        assert_eq!(lexer.advance(), Some('H'));
        assert_eq!(lexer.pos, 1);
    }

    #[test]
    fn test_advance_and_position() {
        let mut lexer = Lexer::new("a\nb");
        assert_eq!(lexer.advance(), Some('a'));
        assert_eq!(lexer.line, 1);
        assert_eq!(lexer.col, 2);
        assert_eq!(lexer.advance(), Some('\n'));
        assert_eq!(lexer.line, 2);
        assert_eq!(lexer.col, 1);
        assert_eq!(lexer.advance(), Some('b'));
        assert_eq!(lexer.line, 2);
        assert_eq!(lexer.col, 2);
        assert_eq!(lexer.advance(), None);
    }

    #[test]
    fn test_single_character_tokens(){
        let mut lexer = Lexer::new("+-*/()");
        let expected_kinds = [
            TokenKind::Plus,
            TokenKind::Minus,
            TokenKind::Star,
            TokenKind::Slash,
            TokenKind::LParen,
            TokenKind::RParen,
            TokenKind::Eof,
        ];

        for x in expected_kinds{
            let token = lexer.next_token().expect("expected valid token");
            assert_eq!(token.kind, x);
        }
    }

    #[test]
    fn test_integers_and_operators() {
        let mut lexer = Lexer::new("12 + 345");
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Int(12));
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Plus);
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Int(345));
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Eof);
    }
}

