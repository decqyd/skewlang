extern crate core;

use std::fs::File;
use std::io::Read;
use anyhow::Result;

mod lexer;
mod result;

use lexer::Lexer;
use crate::result::error::SkewResult;

fn main() -> Result<()>{
    let mut file = File::open("test.skw")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let lexer = Lexer::new(contents.as_str());
    let result = lexer.lex();
    let tokens = result::handle_result(result);
    println!("{tokens:#?}");


    Ok(())
}


