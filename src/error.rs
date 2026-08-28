use crate::frontend::lexer::token::Token;

pub struct LoxErr {
    pub msg: String
}

impl LoxErr {
    pub fn new(msg: String) -> Self {
        LoxErr { msg }
    }
}

#[derive(Debug)]
pub struct ParseError {
    pub tkn: Token,
    pub msg: String
}

impl ParseError {
    pub fn new(tkn: Token, msg: &str) -> Self {
        ParseError { tkn, msg: msg.to_string() }
    }
}

#[derive(Debug)]
pub struct RuntimeError {
    pub tkn: Token,
    pub msg: String
}

impl RuntimeError {
    pub fn new(tkn: &Token, msg: &str) -> Self {
        Self { tkn: tkn.clone(), msg: msg.to_string() } 
    }
}