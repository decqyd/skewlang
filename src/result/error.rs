#[derive(Debug)]
pub enum SkewErrorType {
    UnexpectedToken,
    TypeError
}

#[derive(Debug)]
pub struct SkewResult {
    pub error_type: Option<SkewErrorType>,
    pub line: i32,
    pub loc: i32,
    pub data: ReturnType
}


#[derive(Debug)]
pub enum ReturnType {
    String(String),
    Vec(crate::lexer::tokens::TokenList)
}