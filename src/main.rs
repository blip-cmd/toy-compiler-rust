use toy_compiler::token::TokenKind;
use toy_compiler::token::Token;

fn main() {
    // let tok = TokenKind::Plus;
    // println!("{:?}", tok);

    // let tok2 = TokenKind::Int(42);
    // println!("{:?}", tok2);

    // let tok3 = TokenKind::Eof;
    // println!("{:?}", tok3);

    // let tok4 = TokenKind::LParen;
    // println!("{:?}", tok4);

    // let tok5 = TokenKind::RParen;
    // println!("{:?}", tok5);

    // let tok6 = TokenKind::Star;
    // println!("{:?}", tok6);

    // let tok7 = TokenKind::Slash;
    // println!("{:?}", tok7);

    let tokens = [
        TokenKind::Plus,
        TokenKind::Minus,
        TokenKind::Int(42),
        TokenKind::Star,
        TokenKind::Slash,
        TokenKind::LParen,
        TokenKind::RParen,
        TokenKind::Eof,
    ];

    for tok in tokens.iter() {
        // println!("{:?}", tok);
        println!("{:?}", tok);
    }

    let token = Token::new(TokenKind::Int(42), 1, 1);
    println!("{:?}", token);
}
