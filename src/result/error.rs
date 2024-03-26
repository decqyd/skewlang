#[derive(Debug)]
pub enum SkewErrorType {
    UnexpectedToken,
    TypeError,
    UnterminatedString,
}

#[derive(Debug)]
pub struct SkewError {
    pub error_type: Option<SkewErrorType>,
    pub line: i32,
    pub loc: i32,
    pub cause: String,
}
