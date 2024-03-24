use crate::lexer::tokens::{Token, TokenList};
use std::iter::Peekable;

pub struct Parser {
    token_list: Vec<Token>,
    current_token: Option<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            token_list: tokens,
            current_token: None,
        }
    }
    pub fn parse(&self) {
        let mut tokens = self.token_list.iter().peekable();
        while let Some(token) = tokens.next() {
            println!("tokens: {tokens:?}");
            println!("{}: {}", token.kind, token.value);
        }
    }
    

}
