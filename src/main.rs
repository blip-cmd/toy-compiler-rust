use toy_compiler::token::TokenKind;
// use toy_compiler::token::Token;
use std::env::args;
use toy_compiler::lexer::Lexer;

fn main() {
    let args: Vec<String> = args().collect();
    // println!("{:?}", args);

    //guard clause
    if args.len() <3 || args[1] != "--dump-tokens"{
        eprintln!("Usage: toy-compiler --dump-tokens \"<expression>\"");
        std::process::exit(1);
    }

    let source = &args[2];
    // println!("Source expression: {}", source);

    let mut lexer = Lexer::new(source);

    loop {
        match lexer.next_token(){
            Ok(token)=>{
                let is_eof = token.kind == TokenKind::Eof;
                println!("{:?}", token);
                if is_eof{
                    break;
            }
            }
            Err(e) => {
                eprintln!("Error: {}", e.render(source));
                std::process::exit(1);
            }
        }
    }

}
