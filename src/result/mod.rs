use crate::lexer::tokens::TokenList;
use std::process::exit;

use self::error::SkewError;

pub mod error;
pub mod result_type;

pub fn handle_error(filename: Option<&str>, result: SkewError) {
    if result.error_type.is_some() {
        eprintln!(
            "{}:{}:{}: error!! {:?}: {:?}",
            filename.unwrap_or("no file"),
            result.line,
            result.loc,
            result.error_type.unwrap(),
            result.cause
        );
        exit(1);
    }
}
