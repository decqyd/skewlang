use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // maths
    Plus,
    Minus,
    Multiply,
    Divide,

    // symbols
    Assignment,
    SemiColon,
    BracketOpen,
    BracketClose,
    //SquirlyOpen,
    //SquirlyClose,
    QuoteSingle,
    QuoteDouble,
    Bang,
    Dot,

    // words
    Identifier,
    Let,
    Return,
    Fn,
    Import,
    Do,
    End,
    Puts,

    // conditional
    If,
    Else,

    Value(ValueToken),

    // comparison
    And,
    Or,
    Equals,
    Greater,
    GreaterEqual,
    Lower,
    LowerEqual,

    // loops
    While,
    For,

    Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TokenList {
    pub list: Vec<Token>,
}

impl TokenList {
    pub fn get_list(&mut self) -> Vec<TokenKind> {
        self.list.clone().into_iter().map(|t| t.kind).collect()
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Display for TokenList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self.list)
    }
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum ValueToken {
    // types
    Number,
    Float,
    String,
    Boolean,
}
