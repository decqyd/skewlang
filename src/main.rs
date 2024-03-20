extern crate core;

use anyhow::Result;
use std::fs::File;
use std::io::Read;
use std::rc::Rc;

mod ast;
mod lexer;
mod result;

use crate::result::error::SkewResult;
use lexer::Lexer;

fn main() -> Result<()> {
    let filename: &str = "test.skw";
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let lexer = Lexer::new(contents.as_str());
    let result = lexer.lex();
    let tokens = result::handle_result(Some(filename), result);
    println!("{tokens}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lexer::{tokens::*, Lexer};
    use crate::result::handle_result;
    #[test]
    fn lex_text() {
        let mut tokens = handle_result(None, Lexer::new("let x = 5;").lex());
        assert_eq!(
            tokens.get_list(),
            vec![
                TokenKind::Let,
                TokenKind::Identifier,
                TokenKind::Assignment,
                TokenKind::Number,
                TokenKind::SemiColon
            ]
        );
    }

    #[test]
    fn string_parse() {
        let tokens = handle_result(None, Lexer::new("\"this is a string\"").lex());
        assert_eq!(
            tokens,
            TokenList {
                list: vec![Token {
                    kind: TokenKind::String,
                    value: "\"this is a string\"".to_string()
                }]
            }
        )
    }
}
