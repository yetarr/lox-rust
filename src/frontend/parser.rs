pub mod expr;
pub mod stmt;
pub mod rules;
pub mod error;

use crate::{frontend::parser::{error::ParseError, stmt::Stmt}, lox::Lox};
use super::{lexer::token::{Token, TokenT}}; 

pub struct Parser<'a> {
    tkns: Vec<Token>,
    ptr: usize,
    lox: &'a mut Lox
}

impl<'a> Parser<'a> {
    pub fn new(tkns: Vec<Token>, lox: &'a mut Lox) -> Self {
        Parser { 
            tkns, 
            ptr: 0,
            lox
        }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();
        while !self.eof() {
            if let Some(s) = self.declaration() {
                stmts.push(s);
            }
        }
        stmts
    }

    fn tkn_match(&mut self, tkns_t: &[TokenT]) -> bool {
        for t in tkns_t {
            if self.check_cur(t) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check_cur(&self, token_t: &TokenT) -> bool {
        if self.eof() { return false; }
        self.peek().token_t == *token_t
    }

    fn peek(&self) -> &Token {
        &self.tkns[self.ptr]
    }

    fn previous(&self) -> &Token {
        &self.tkns[self.ptr - 1]
    }

    fn advance(&mut self) -> &Token {
        if !self.eof() { self.ptr += 1; }
        self.previous()
    }

    fn eof(&self) -> bool {
        self.peek().token_t == TokenT::EOF
    }

    fn consume(&mut self, token_t: TokenT, msg: &str) -> Result<&Token, ParseError> {
        if self.check_cur(&token_t) { return Ok(self.advance()); }
        let tkn = self.peek().clone();
        Err(ParseError::new(tkn, msg))
    }

    fn error(&mut self, tkn: &Token, msg: &str) {
        self.lox.error_parse(tkn, msg);
    }

    fn sync(&mut self) {
        
    }
}