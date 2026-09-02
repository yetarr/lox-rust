use crate::prelude::*;
use crate::frontend::parser::{Parser, stmt::FunType};

macro_rules! binary_rule {
    ($name:ident, $next:ident, [$($tkn:expr), +]) => {
        fn $name(&mut self) -> Result<Expr, ParseError> {
            if self.tkn_match(&[$($tkn),+]){
                let err = Err(ParseError::new(self.previous().clone(), "Expect expression before operator."));
                let _ = self.$next();
                return err;
            }
            
            let mut expr = self.$next();
            while self.tkn_match(&[$($tkn),+]) {
                let op = self.previous().clone();
                let right = self.$next()?;
                expr = Ok(Expr::Binary { 
                    left: Box::new(expr?),
                    op,
                    right: Box::new(right),
                });
            }
            expr
        }
    };
}

macro_rules! logical_rule {
    ($name:ident, $next:ident, [$($tkn:expr), +]) => {
        fn $name(&mut self) -> Result<Expr, ParseError> {
            if self.tkn_match(&[$($tkn),+]){
                let err = Err(ParseError::new(self.previous().clone(), "Expect expression before operator."));
                let _ = self.$next();
                return err;
            }
            
            let mut expr = self.$next();
            while self.tkn_match(&[$($tkn),+]) {
                let op = self.previous().clone();
                let right = self.$next()?;
                expr = Ok(Expr::Logical { 
                    left: Box::new(expr?),
                    op,
                    right: Box::new(right),
                });
            }
            expr
        }
    };
}

impl<'a> Parser<'a> {
    pub fn expression(&mut self) -> Result<Expr, ParseError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, ParseError> {
        let expr = self.anon_fun()?;
        if self.tkn_match(&[TokenT::Equal]) {
            let equals = self.previous().clone(); 
            let val = self.assignment()?;
            match expr {
                Expr::Variable(var) => 
                    return Ok(Expr::Assign { name: var, val: Box::new(val) }),
                _ => self.error(&equals, "Invalid assignment target."),
            }
        }
        Ok(expr)
    }
    
    fn anon_fun(&mut self) -> Result<Expr, ParseError> {
        let expr = self.logic_or();
        if self.tkn_match(&[TokenT::Keyword(Keyword::Fun)]) {
            self.consume(TokenT::LeftParen, "Expect '(' after 'fun'.")?;
            let (params, body) = self.parameters(FunType::Function)?;
            return Ok(Expr::AnonFun { params, body })
        }
        expr
    }
    
    logical_rule!(logic_or, logic_and, [TokenT::Keyword(Keyword::Or)]);
    
    logical_rule!(logic_and, ternary, [TokenT::Keyword(Keyword::And)]);
    
    fn ternary(&mut self) -> Result<Expr, ParseError> {
        let cond = self.equality();
    
        if self.tkn_match(&[TokenT::Query]) {
            let first = self.expression()?;
            self.consume(TokenT::Colon, "Expect ':' after first expression.")?;
            let second = self.expression()?;            
            return Ok(Expr::Ternary { 
                cond: Box::new(cond?), 
                first: Box::new(first), 
                second: Box::new(second)
            });
        }
        cond
    }
    
    // binary_rule!(comma, equality, [TokenT::Comma]);
    
    binary_rule!(equality, comparison, [TokenT::BangEqual, TokenT::EqualEqual]);
    
    binary_rule!(comparison, term, [TokenT::Greater, TokenT::GreaterEqual, TokenT::Less, TokenT::LessEqual]);
    
    binary_rule!(term, factor, [TokenT::Plus, TokenT::Minus]);
    
    binary_rule!(factor, unary, [TokenT::Slash, TokenT::Star]);
    
    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.tkn_match(&[TokenT::Bang, TokenT::Minus]) {
            let op = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::Unary { 
                op, 
                right: Box::new(right)
            });
        }
        self.call()
    }
    
    fn call(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.primary();
    
        loop {
            if self.tkn_match(&[TokenT::LeftParen]) {
                expr = self.finish_call(expr?);
            } else {
                break;
            }
        }
        expr
    }
    
    fn finish_call(&mut self, callee: Expr) -> Result<Expr, ParseError> {
        let mut args = Vec::new();
        if !self.check_cur(&TokenT::RightParen) {
            args.push(self.expression()?);
            while self.tkn_match(&[TokenT::Comma]) {
                args.push(self.expression()?);
            }
        }
    
        if args.len() >= 255 {
            self.error(&self.peek().clone(), "Can't have more than 255 arguments");
        }
        
        let paren = self.consume(TokenT::RightParen, "Expect ')' after arguments")?.clone();
        Ok(Expr::Call { callee: Box::new(callee), paren, args })
    }
    
    fn primary(&mut self) -> Result<Expr, ParseError> {
        if self.tkn_match(&[
            TokenT::Keyword(Keyword::Nil),
            TokenT::Keyword(Keyword::True),
            TokenT::Keyword(Keyword::False),
            TokenT::Literal
        ]) {
            return Ok(Expr::Literal(self.previous().lit.clone()))
        }
    
        if self.tkn_match(&[TokenT::Identifier]){
            return Ok(Expr::Variable(self.previous().clone()));
        }
    
        if self.tkn_match(&[TokenT::LeftParen]) {
            let expr = self.expression();
            self.consume(TokenT::RightParen, "Expect ')' after expression.")?;
            return Ok(Expr::Grouping(Box::new(expr?)))
        }
    
        Err(ParseError::new(self.peek().clone(), "Expect expression."))
    }
}