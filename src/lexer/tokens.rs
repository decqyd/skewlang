use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // maths
    Plus,
    Minus,
    Multiply,
    Divide,
    Number,

    // symbols
    Assignment,
    Equals,
    SemiColon,
    BracketOpen,
    BracketClose,
    SquirlyOpen,
    SquirlyClose,
    
    // other??
    Identifier,
    Let,
    FunctionDecl,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TokenList {
    pub list: Vec<Token>
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
        write!(f, "{:#?}", self.list.iter().map(|tkn| &tkn.value))  
    }
}
