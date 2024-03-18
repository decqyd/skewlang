extern crate core;

use std::fs::File;
use std::io::Read;
use std::rc::Rc;
use anyhow::Result;

mod lexer;
mod result;
mod ast;

use lexer::Lexer;
use crate::result::error::SkewResult;

fn main() -> Result<()>{
    let filename: &str = "test.skw";
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let lexer = Lexer::new(contents.as_str());
    let result = lexer.lex();
    let tokens = result::handle_result(filename, result);
    println!("{tokens}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lexer::{Lexer, tokens::*};
    use crate::result::handle_result;
    #[test]
    fn lex_text() {
        let mut tokens = handle_result("test.skw", 
            Lexer::new("let x = 5;").lex());
        assert_eq!(tokens.get_list(), 
        vec![TokenKind::Let, TokenKind::Identifier, TokenKind::Assignment, TokenKind::Number, TokenKind::SemiColon]);
        
    }

}

