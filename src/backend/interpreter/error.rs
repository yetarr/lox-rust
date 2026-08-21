use crate::frontend::lexer::token::Token;

#[derive(Debug)]
#[allow(dead_code)]
pub struct RuntimeError {
    pub tkn: Token,
    pub msg: String
}

impl RuntimeError {
    pub fn new(tkn: &Token, msg: &str) -> Self {
        Self { tkn: tkn.clone(), msg: msg.to_string() } 
    }
}