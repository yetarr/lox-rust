pub mod expr;
pub mod rules;

use crate::{lox::Lox, parser::expr::Expr, scanner::token::{Token, TokenT}};

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

    pub fn parse(&mut self) -> Option<Expr> {
        self.expression()
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

    fn consume(&mut self, token_t: TokenT, msg: &str) -> Option<&Token> {
        if self.check_cur(&token_t) { return Some(self.advance()); }
        let tkn = self.peek().clone();
        self.error(&tkn, msg);
        None
    }

    fn error(&mut self, tkn: &Token, msg: &str) {
        self.lox.error_parse(tkn, msg);
    }
}