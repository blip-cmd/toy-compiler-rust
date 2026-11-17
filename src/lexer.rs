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

// in rust, impl means implementation, and it is used to define methods for a struct or enum. In this case, we are implementing methods for the Lexer struct. The methods defined in the impl block can be called on instances of the Lexer struct.
impl CompileError{
    pub fn render(&self, source: &str) -> String{
        //find the specific line of text by splitting on newlines
        let line_text = source.lines().nth(self.line -1).unwrap_or("");
        let indent=" ".repeat(self.col.saturating_sub(1)); //.saturating_sub(x) means that if self.col is less than x, it will return 0 instead of panicking

        //format!() means that we are creating a formatted string. The {} are placeholders for the variables
        format!(
            "error: {}\n  --> line {}, col {}:\n   |\n{:2} | {}\n   | {}^",
            self.message, self.line, self.col, self.line, line_text, indent
        )
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

    #[test]
    fn test_errot_caret_display(){
        let input = "1 $ 2";
        let mut lexer = Lexer::new(input);

        //in rust assert_eq! has an ! meaning that it is a macro, not a function. Macros are like functions, but they can take a variable number of arguments and can be used to generate code at compile time. The ! means that it is a macro, not a function. 
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Int(1));

        //unwrap_err is for Result<T,E> and it will return the error value if the result is an Err, otherwise it will panic. In this case, we are expecting an error because the next token is a $ which is not a valid token.
        //a panic is a runtime error that occurs when the program encounters an unexpected condition. In this case, we are expecting an error because the next token is a $ which is not a valid token.
        //TODO why unwrap and unwrap_err?
        let err = lexer.next_token().unwrap_err();
        assert_eq!(err.line, 1);
        assert_eq!(err.col, 3);

        //err here is a CompileError struct, and we are calling the render method on it to get a formatted string that shows the error message, line number, column number, and the line of code where the error occurred with a caret pointing to the column where the error occurred.
        let rendered = err.render(input);
        println!("\nRendered Error:\n{}", rendered);

        assert!(rendered.contains("unexpected character: $"));
        assert!(rendered.contains("1 | 1 $ 2"));
        assert!(rendered.contains("   |   ^"));

        

    }
}

