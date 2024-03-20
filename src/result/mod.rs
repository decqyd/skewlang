use crate::lexer::tokens::TokenList;
use crate::result::error::{ReturnType, SkewResult};
use std::process::exit;
use std::rc::Rc;

pub mod error;
pub mod result_type;

pub fn handle_result(filename: Option<&str>, result: SkewResult) -> TokenList {
    match result.data {
        ReturnType::Vec(t_list) => return t_list,
        ReturnType::String(cause) => {
            if result.error_type.is_some() {
                eprintln!(
                    "{}:{}:{}: error!! {:?}: {:?}",
                    filename.unwrap_or("no file"),
                    result.line,
                    result.loc,
                    result.error_type.unwrap(),
                    cause
                );
                exit(1);
            }
            unreachable!()
        }
    }
}

