pub mod types;
use crate::lexer::tokens::{Token, TokenKind};
use std::iter::Peekable;
use std::vec::IntoIter;

pub struct Parser {
    token_list: Peekable<IntoIter<Token>>,
    current_token: Option<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            token_list: tokens.into_iter().peekable(),
            current_token: None,
        }
    }

    pub fn parse(&mut self) {
        while let Some(token) = self.token_list.next() {
            println!("{token}");
            match token.kind {
                TokenKind::Puts => {
                    println!("this is a puts!!");
                }

                _ => (),
            }
        }
    }
}
