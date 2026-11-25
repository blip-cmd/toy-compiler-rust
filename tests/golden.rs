use std::fs; // used for reading the golden file
use toy_compiler::lexer::Lexer;
use toy_compiler::token::TokenKind;

fn lex_to_string(input: &str) -> String {
    let mut lexer = Lexer::new(input);
    let mut output = String::new();

    // TODO: loop and match on lexer.next_token()
    // - On Ok(token):
    //     let is_eof = token.kind == TokenKind::Eof;
    //     output.push_str(&format!("{:?}\n", token));
    //     if is_eof { break; }
    // - On Err(err):
    //     return err.render(input);

    loop{
        match lexer.next_token(){
            Ok(token)=>{
                let is_eof = token.kind == TokenKind::Eof;
                output.push_str(&format!("{:?}\n", token)); // explain syntax to me
                if is_eof{
                    break;
                }
            }
            Err(e)=>{
                output.push_str(&e.render(input));
                return output;
            }
        }
    }
    output
}

#[test]
fn golden_lexer_tests() {
    let mut count = 0;

    for entry in fs::read_dir("tests/cases").expect("could not read tests/cases") { //return iterator of folders contents
        let path = entry.expect("bad directory entry").path(); //not clear

        if path.extension().and_then(|e| e.to_str()) == Some("in") {//not clear
            // TODO:
            // 1. Read input: let input = fs::read_to_string(&path).expect("failed to read .in file");
            // 2. Get expected path: let expected_path = path.with_extension("expected");
            // 3. Read expected: let expected = fs::read_to_string(&expected_path).expect("failed to read .expected file");
            // 4. Run lexer: let actual = lex_to_string(&input);
            // 5. Normalise line endings:
            //    let actual_norm = actual.replace("\r\n", "\n");
            //    let expected_norm = expected.replace("\r\n", "\n");
            // 6. Compare: assert_eq!(actual_norm.trim_end(), expected_norm.trim_end(), "mismatch in {:?}", path);
            // 7. count += 1;

            let input=fs::read_to_string(&path).expect("failed to read .in file");

            let expected_path = path.with_extension("expected");
            let expected = fs::read_to_string(&expected_path).expect("failed to read .expected file");

            let actual = lex_to_string(&input);

            let actual_norm = actual.replace("\r\n", "\n"); // \r\n is the carriage return and newline characters used in Windows, while \n is the newline character used in Unix-based systems. This line replaces all occurrences of \r\n with \n to normalize line endings for comparison.
            let expected_norm = expected.replace("\r\n", "\n");

            assert_eq!(actual_norm.trim_end(), expected_norm.trim_end(), "mismatch in {:?}", path);
            count += 1;


        }
    }

    assert!(count >= 10, "expected at least 10 golden test cases, found {}", count);
}
