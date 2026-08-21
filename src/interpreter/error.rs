use crate::lexer::token::Token;

#[derive(Debug)]
#[allow(dead_code)]
pub struct RuntimeError {
    op: Token,
    msg: String
}

impl RuntimeError {
    pub fn new(op: &Token, msg: &str) -> Self {
        Self { op: op.clone(), msg: msg.to_string() }
    }
}