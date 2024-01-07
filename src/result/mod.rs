use std::process::exit;
use crate::lexer::tokens::TokenList;
use crate::result::error::{ReturnType, SkewResult};

pub mod error;
pub mod result_type;

pub fn handle_result(result: SkewResult) -> TokenList {
    match result.data {
        ReturnType::Vec(t_list) => return t_list,
        ReturnType::Char(char) => {
            if result.error_type.is_some() {
                eprintln!("error!! {:?} at line {}, char {}: {:?}", result.error_type.unwrap(), result.line, result.loc, char);
                exit(1);
            }
            unreachable!()
        }
    }
}