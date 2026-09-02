pub mod expr;
pub mod stmt;
pub mod rules;

use crate::prelude::*;
use crate::lox::Lox;

pub struct Parser<'a> {
    tkns: &'a [Token],
    ptr: usize,
    lox: &'a mut Lox,
    loop_depth: u8,
    in_args: bool
}

impl<'a> Parser<'a> {
    pub fn new(tkns: &'a [Token], lox: &'a mut Lox) -> Self {
        Parser {
            tkns,
            ptr: 0,
            lox,
            loop_depth: 0,
            in_args: false,
        }
    }

    pub fn parse(&mut self) -> Option<Vec<Stmt>> {
        let mut stmts = Vec::new();
        while !self.eof() {
            match self.declaration() {
                Ok(stmt) => stmts.push(stmt),
                Err(err) => {
                    self.error(&err.tkn, &err.msg);
                    self.sync();
                }
            }
        }
        if stmts.is_empty() { None }
        else                { Some(stmts) }
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

    fn retreat(&mut self) {
        if !self.eof() { self.ptr -= 1; }
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
        self.advance();

        while !self.eof() {
            if self.previous().token_t == TokenT::Semicolon {
                return;
            }

            match self.peek().token_t {
                TokenT::Keyword(kw) => match kw {
                    Keyword::Return | Keyword::Class | Keyword::For |
                    Keyword::Fun    | Keyword::Var   | Keyword::If  |
                    Keyword::While  => return,
                    _ => self.advance(),
                }
                _ => self.advance(),
            };
        }
    }
}