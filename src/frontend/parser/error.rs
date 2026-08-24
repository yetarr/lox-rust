use crate::frontend::lexer::token::Token;

pub struct ParseError {
    pub tkn: Token,
    pub msg: String
}

impl ParseError {
    pub fn new(tkn: Token, msg: &str) -> Self {
        ParseError { tkn, msg: msg.to_string() }
    }
}