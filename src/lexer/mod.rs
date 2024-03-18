use std::iter::Peekable;
use std::str::{Chars};
use crate::lexer::tokens::{Token, TokenKind, TokenList};
use crate::result::{error::{SkewErrorType, SkewResult, ReturnType}, result_type::ResultType};

pub mod tokens;

#[derive(Clone)]
pub struct Lexer <'a>{
    input: &'a str,
    current_char: Option<char>,
    chars: Peekable<Chars<'a>>,
    pub line: i32,
    pub loc: i32,
    tokens: TokenList
}


impl<'a> Lexer<'a> {
    // public methods
    pub fn new(input: &'a str) -> Self {
        let mut chars = input.chars().peekable();
        let current_char = chars.peek().copied();
        let mut line = 1;
        let mut loc = -1;
        let list = Vec::new();
        let tokens :TokenList = TokenList {
            list
        };
        Self {
            input,
            chars,
            current_char,
            line,
            loc,
            tokens
        }
    }

    pub fn lex(mut self) -> SkewResult {
        while let Some(char) = self.consume() {
            self.current_char = Some(char);
            self.loc += 1;

            // TODO: add more tokens
            match char {
              '+' => self.make_token(TokenKind::Plus, char.to_string()),
              '-' => self.make_token(TokenKind::Minus, char.to_string()),
              '*' => self.make_token(TokenKind::Multiply, char.to_string()),
              '=' => {
                if self.check_next('=') {
                    let next = self.consume();
                    self.make_token(TokenKind::Equals,String::from(char) + &String::from(next.unwrap()));
                } else {
                    self.make_token(TokenKind::Assignment, char.to_string());
                }
              },
                ';' => {
                    self.make_token(TokenKind::SemiColon, char.to_string());
                },
                '(' => {
                    self.make_token(TokenKind::BracketOpen, char.to_string());
                },
                ')' => {
                    self.make_token(TokenKind::BracketClose, char.to_string());
                },
                '{' => {
                    self.make_token(TokenKind::SquirlyOpen, char.to_string());
                },
                '}' => {
                    self.make_token(TokenKind::SquirlyClose, char.to_string())
                },

                // extra
                ' ' => (),
                '\n' => {
                  self.loc = 0;
                  self.line += 1;
                },
                '\r' => (),
                '/'  => {
                    //let next = self.consume();
                    //let mut comment = String::new();
                    if self.check_next('/') {
                        while self.peek().unwrap_or(&'\0') != &'\n' {
                            //comment.push(self.consume().unwrap_or('\0'));
                            self.consume();
                            self.loc += 1
                       }
                    } else {
                        self.make_token(TokenKind::Divide, char.to_string());
                    }
                    //self.make_token(TokenKind::Comment, self.char_concat(char, next) );
                }
                _ => {
                    if char.is_ascii_alphanumeric() {
                        let mut identifier = String::from(char);
                        while self.peek().unwrap_or(&'\0').is_ascii_alphanumeric() {
                            identifier.push(self.consume().unwrap_or('\0'));
                            self.loc += 1
                        }
                        match identifier.as_str() {
                            "let" => self.make_token(TokenKind::Let, identifier),
                            "fn" => self.make_token(TokenKind::FunctionDecl, identifier),
                            _ => {
                                if identifier.parse::<i32>().is_ok() || identifier.parse::<f32>().is_ok() {
                                    self.make_token(TokenKind::Number, identifier)
                                } 
                                else {
                                    self.make_token(TokenKind::Identifier, identifier)
                                }
                            }
                        }
                    } else {
                          return self.return_as(ResultType::FAILURE, Some(SkewErrorType::UnexpectedToken))
                    }
                }
            };
        };
        self.return_as(ResultType::SUCCESS, None)
    }

    fn next_str(&mut self) -> String {
        self.consume().unwrap().to_string()
    }

    fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    fn check_next(&mut self, char: char) -> bool {
        self.peek().is_some() && self.peek().unwrap().to_owned() == char
    }

    fn check_current(&mut self, char: char) -> bool {
        self.current_char.unwrap() == char
    }

    fn consume(&mut self) -> Option<char> {
        self.chars.next()
    }

    fn char_concat(&self, c1: char, c2: Option<char>) -> String {
        String::from(c1) + &String::from(c2.unwrap())
    }

    fn make_token(
        &mut self,
        token: TokenKind,
        value: String
    ) {
        self.tokens.list.push(Token {
            kind: token,
            value
        });
    }

    fn return_as(&self, result_type: ResultType, error_type: Option<SkewErrorType>) -> SkewResult {
        match result_type {
            ResultType::SUCCESS => SkewResult {
                error_type: None,
                line: self.line,
                loc: self.loc,
                data: ReturnType::Vec(self.clone().tokens)
            },
            ResultType::FAILURE => {
                SkewResult {
                    error_type,
                    line: self.line,
                    loc: self.loc,
                    data: ReturnType::Char(self.current_char.expect("no character"))
                }
            },
        }
    }
}

