use std::process::exit;
use crate::lexer::tokens::TokenList;
use crate::result::error::{ReturnType, SkewResult};
use std::rc::Rc;

pub mod error;
pub mod result_type;

pub fn handle_result(filename: &str, result: SkewResult) -> TokenList {
    match result.data {
        ReturnType::Vec(t_list) => return t_list,
        ReturnType::String(cause) => {
            if result.error_type.is_some() {
                eprintln!("{filename}:{}:{}: error!! {:?}: {:?}", result.line, result.loc, result.error_type.unwrap(), cause);
                exit(1);
            }
            unreachable!()
        }
    }
}