#[derive(Debug, Clone)]
pub enum TokenKind {
    Plus,
    Minus,
    Assignment,
    Equals,
    Identifier,
    Let,
    SemiColon,
    Number
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct TokenList {
    pub list: Vec<Token>
}

