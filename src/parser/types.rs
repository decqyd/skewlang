use crate::lexer::tokens::*;
pub enum Stmt {
    VarDecl {
        name: TokenKind,
        value: Option<Token>,
    },
    FnDecl {
        name: TokenKind,
        params: Option<Vec<TokenKind>>,
        body: Vec<Box<Stmt>>,
    },
    Print {
        value: Option<TokenKind>,
    },
    If {
        condition: Expr,
        body: Vec<Box<Stmt>>,
    },
    While {
        condition: Expr,
        body: Vec<Box<Stmt>>,
    },
    Return {
        value: Expr,
    },
}

#[derive(Clone, Debug)]
pub enum Expr {
    Assign {
        name: TokenKind,
        value: Option<Token>,
    },
}
