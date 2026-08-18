use super::Parser;
use crate::{scanner::token::{Keyword, TokenT}, parser::expr::Expr};

macro_rules! binary_rule {
    ($name:ident, $next:ident, [$($tkn:expr), +]) => {
        fn $name(&mut self) -> Option<Expr> {
            let mut expr = self.$next();
            while self.tkn_match(&[$($tkn),+]) {
                let op = self.previous().clone();
                let right = self.$next()?;
                expr = Some(Expr::Binary { 
                    left: Box::new(expr?),
                    op,
                    right: Box::new(right),
                });
            }
            expr
        }
    };
}

#[allow(dead_code)]
impl<'a> Parser<'a> {
    pub(in crate::parser) fn expression(&mut self) -> Option<Expr> {
        self.equality()
    }

    binary_rule!(equality, comparison, [TokenT::BangEqual, TokenT::EqualEqual]);
    
    binary_rule!(comparison, term, [TokenT::Greater, TokenT::GreaterEqual, TokenT::Less, TokenT::LessEqual]);
    
    binary_rule!(term, factor, [TokenT::Plus, TokenT::Minus]);
    
    binary_rule!(factor, unary, [TokenT::Slash, TokenT::Star]);

    fn unary(&mut self) -> Option<Expr> {
        if self.tkn_match(&[TokenT::Bang, TokenT::Minus]) {
            let op = self.previous().clone();
            let right = self.unary()?;
            return Some(Expr::Unary { 
                op, 
                right: Box::new(right)
            });
        }
        self.primary()
    }

    fn primary(&mut self) -> Option<Expr> {
        if self.tkn_match(&[
            TokenT::Keyword(Keyword::Nil),
            TokenT::Keyword(Keyword::True),
            TokenT::Keyword(Keyword::False),
            TokenT::Literal
        ]) {
            return Some(Expr::Literal(self.previous().lit.clone()))
        }

        if self.tkn_match(&[TokenT::LeftParen]) {
            let expr = self.expression();
            self.consume(TokenT::RightParen, "Expect ')' after expression,");
            return Some(Expr::Grouping(Box::new(expr?)))
        }
        
        self.error(&self.peek().clone(), "Expect expression.");
        None
    }
}