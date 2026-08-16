use crate::{lexer::token::{Keyword, Token, TokenT}, parser::expr::Expr};

pub struct Parser {
    tkns: Vec<Token>,
    ptr: usize,
}

#[allow(dead_code)]
impl Parser {
    pub fn new(tkns: Vec<Token>) -> Self {
        Parser { 
            tkns, 
            ptr: 0 
        }
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();
        while self.tkn_match(&[TokenT::BangEqual, TokenT::EqualEqual]) {
            let op = self.previous().clone();
            let right = self.comparison();
            expr = Expr::Binary { 
                left: Box::new(expr),
                op: op, 
                right: Box::new(right),
            }
        }
        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();
        while self.tkn_match(&[
            TokenT::Greater, 
            TokenT::GreaterEqual,
            TokenT::Less,
            TokenT::LessEqual
        ]) {
            let op = self.previous().clone();
            let right = self.term();
            expr = Expr::Binary { 
                left: Box::new(expr),
                op: op, 
                right: Box::new(right),
            }
        }
        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();
        while self.tkn_match(&[TokenT::Plus, TokenT::Minus]) {
            let op = self.previous().clone();
            let right = self.factor();
            expr = Expr::Binary { 
                left: Box::new(expr),
                op: op, 
                right: Box::new(right),
            }
        }
        expr
    }
    
    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();
        while self.tkn_match(&[TokenT::Slash, TokenT::Star]) {
            let op = self.previous().clone();
            let right = self.unary();
            expr = Expr::Binary { 
                left: Box::new(expr),
                op: op, 
                right: Box::new(right),
            }
        }
        expr
    }

    fn unary(&mut self) -> Expr {
        if self.tkn_match(&[TokenT::Bang, TokenT::Minus]) {
            let op = self.previous().clone();
            let right = self.unary();
            return Expr::Unary { 
                op, 
                right: Box::new(right)
            };
        }
        self.primary()
    }

    fn primary(&mut self) -> Expr {
        if self.tkn_match(&[
            TokenT::Literal, 
            TokenT::Keyword(Keyword::Nil),
            TokenT::Keyword(Keyword::True),
            TokenT::Keyword(Keyword::False),
        ]) {
            return Expr::Literal(self.previous().lit.clone())
        }

        if self.tkn_match(&[TokenT::LeftParen]) {
            let expr = self.expression();
            return Expr::Grouping(Box::new(expr))
        }
        panic!("expected expression, got {:?}", self.peek())
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

    fn consume(&mut self) {
        if self.tkn_match(&[TokenT::RightParen]) {
            self.advance();
        } else {
            
        }
    }

    fn eof(&self) -> bool {
        self.peek().token_t == TokenT::EOF
    }
}