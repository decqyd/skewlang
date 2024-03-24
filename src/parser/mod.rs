use crate::lexer::tokens::{Token, TokenList};
use std::iter::Peekable;

pub struct Parser {
    token_list: Peekable<Vec<Token>>,
    current_token: Option<Token>,
}

impl Parser {
    pub fn new(tokens: TokenList) -> Self {
        Parser {
            token_list: tokens,
            current_token: None,
        }
    }
    pub fn parse(&self) {
        println!("{}", self.token_list);
    }

    fn get_current_token(&self) -> Option<Token> {
        self.current_token
    }
}
