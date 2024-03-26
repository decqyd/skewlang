#![allow(dead_code, unused_mut)]
use crate::lexer::tokens::{Token, TokenKind, TokenList};
use crate::result::error::SkewError;
use crate::result::{error::SkewErrorType, result_type::ResultType};
use std::iter::Peekable;
use std::str::Chars;

pub mod tokens;
use tokens::ValueToken;

#[derive(Clone)]
pub struct Lexer<'a> {
    input: &'a str,
    current_char: Option<char>,
    chars: Peekable<Chars<'a>>,
    pub line: i32,
    pub loc: i32,
    tokens: TokenList,
}

impl<'a> Lexer<'a> {
    // public methods
    pub fn new(input: &'a str) -> Self {
        let mut chars = input.chars().peekable();
        let current_char = chars.peek().copied();
        let mut line = 1;
        let mut loc = -1;
        let list = Vec::new();
        let tokens: TokenList = TokenList { list };
        Self {
            input,
            chars,
            current_char,
            line,
            loc,
            tokens,
        }
    }

    pub fn lex(mut self) -> Result<TokenList, SkewError> {
        while let Some(char) = self.consume() {
            self.current_char = Some(char);
            self.loc += 1;

            // TODO: add more tokens
            match char {
                '+' => self.make_token(TokenKind::Plus, char.to_string()),
                '-' => self.make_token(TokenKind::Minus, char.to_string()),
                '*' => self.make_token(TokenKind::Multiply, char.to_string()),
                '=' => match self.check_next('=') {
                    true => {
                        let next = self.consume();
                        self.make_token(TokenKind::Equals, self.char_concat(char, next));
                    }
                    false => self.make_token(TokenKind::Assignment, char.to_string()),
                },
                ';' => self.make_token(TokenKind::SemiColon, char.to_string()),
                '(' => self.make_token(TokenKind::BracketOpen, char.to_string()),
                ')' => self.make_token(TokenKind::BracketClose, char.to_string()),
                //'{' => self.make_token(TokenKind::SquirlyOpen, char.to_string()),
                //'}' => self.make_token(TokenKind::SquirlyClose, char.to_string()),
                '.' => self.make_token(TokenKind::Dot, char.to_string()),
                '\'' => self.make_token(TokenKind::QuoteSingle, char.to_string()),
                '"' => match self.handle_string(char) {
                    Err(e) => {
                        return self.error(
                            ResultType::FAILURE,
                            Some(SkewErrorType::UnterminatedString),
                            Some(e),
                        )
                    }
                    _ => continue,
                },
                '>' => match self.check_next('=') {
                    true => {
                        let next = self.consume();
                        self.make_token(TokenKind::GreaterEqual, self.char_concat(char, next))
                    }
                    false => self.make_token(TokenKind::Greater, char.to_string()),
                },
                '<' => match self.check_next('=') {
                    true => {
                        let next = self.consume();
                        self.make_token(TokenKind::LowerEqual, self.char_concat(char, next))
                    }
                    false => self.make_token(TokenKind::Lower, char.to_string()),
                },

                // extra
                ' ' | '\r' => (),
                '\n' => {
                    self.loc = 0;
                    self.line += 1;
                }
                '/' => {
                    if self.check_next('/') {
                        while self.peek().unwrap_or(&'\0') != &'\n' && self.peek().is_some() {
                            self.consume();
                            self.loc += 1
                        }
                    } else {
                        self.make_token(TokenKind::Divide, char.to_string());
                    }
                }
                _ => {
                    if char.is_ascii_alphabetic() {
                        self.handle_identifier(char);
                    } else if char.is_ascii_digit() {
                        match self.handle_number(char) {
                            Err(e) => {
                                return self.error(
                                    ResultType::FAILURE,
                                    Some(SkewErrorType::TypeError),
                                    Some(e),
                                )
                            }
                            _ => continue,
                        }
                    } else {
                        return self.error(
                            ResultType::FAILURE,
                            Some(SkewErrorType::UnexpectedToken),
                            Some(char.to_string()),
                        );
                    }
                }
            };
        }
        self.make_token(TokenKind::Eof, String::new());
        Ok(self.tokens)
    }

    fn handle_identifier(&mut self, input: char) -> ResultType {
        let mut identifier = String::from(input);
        while !self.peek().unwrap_or(&'\0').is_ascii_whitespace()
            && self.peek().is_some()
            && self.peek().unwrap_or(&'\0').is_alphanumeric()
            || self.peek().unwrap_or(&'\0') == &'_'
        {
            identifier.push(self.consume().unwrap_or('\0'));
        }
        match identifier.as_str() {
            "let" => self.make_token(TokenKind::Let, identifier),
            "fn" => self.make_token(TokenKind::Fn, identifier),
            "if" => self.make_token(TokenKind::If, identifier),
            "else" => self.make_token(TokenKind::Else, identifier),
            "while" => self.make_token(TokenKind::While, identifier),
            "for" => self.make_token(TokenKind::For, identifier),
            "ret" => self.make_token(TokenKind::Return, identifier),
            "do" => self.make_token(TokenKind::Do, identifier),
            "end" => self.make_token(TokenKind::End, identifier),
            "puts" => self.make_token(TokenKind::Puts, identifier),
            "true" | "false" => self.make_token(TokenKind::Value(ValueToken::Boolean), identifier),
            _ => self.make_token(TokenKind::Identifier, identifier),
        }
        ResultType::SUCCESS
    }

    fn handle_number(&mut self, input: char) -> Result<(), String> {
        let mut number = String::from(input);
        while !self.peek().unwrap_or(&'\0').is_ascii_whitespace()
            && self.peek().is_some()
            && self.peek().unwrap_or(&'\0') != &';'
        {
            number.push(self.consume().unwrap_or('\0'));
        }
        if number.parse::<i64>().is_ok() {
            self.make_token(TokenKind::Value(ValueToken::Number), number);
        } else if number.parse::<f64>().is_ok() {
            self.make_token(TokenKind::Value(ValueToken::Float), number);
        } else {
            return Err(number);
        }
        Ok(())
    }

    fn handle_string(&mut self, input: char) -> Result<(), String> {
        let mut value = String::from(input);
        while self.peek().unwrap_or(&'\0') != &'"' && self.peek().is_some() {
            value.push(self.consume().unwrap_or('\0'));
        }
        value.push(self.consume().unwrap_or('\0')); // for the last set of double quotes
        if value.contains('\0') {
            return Err(value);
        }
        self.make_token(TokenKind::Value(ValueToken::String), value);
        Ok(())
    }

    fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    fn check_next(&mut self, char: char) -> bool {
        self.peek().is_some() && self.peek().unwrap().to_owned() == char
    }

    fn consume(&mut self) -> Option<char> {
        self.loc += 1;
        self.chars.next()
    }

    fn char_concat(&self, c1: char, c2: Option<char>) -> String {
        String::from(c1) + &String::from(c2.unwrap())
    }

    fn make_token(&mut self, token: TokenKind, value: String) {
        self.tokens.list.push(Token { kind: token, value });
    }

    fn error(
        &self,
        result_type: ResultType,
        error_type: Option<SkewErrorType>,
        cause: Option<String>,
    ) -> SkewError {
        SkewError {
            error_type,
            line: self.line,
            loc: self.loc,
            cause: cause.expect("no cause"),
        }
    }
}
