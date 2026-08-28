use crate::error::ParseError;
use crate::frontend::parser::stmt::Stmt;

use super::Parser;
use super::super::parser::expr::Expr;
use super::super::lexer::token::{Keyword, TokenT};

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

impl<'a> Parser<'a> {
    pub(in super::super::parser) fn declaration(&mut self) -> Result<Stmt, ParseError> {
        if self.tkn_match(&[TokenT::Keyword(Keyword::Var)]) {
            return self.var_decl()
        } 
        self.statement()
    }

    fn var_decl(&mut self) -> Result<Stmt, ParseError> {
        let name = self.consume(TokenT::Identifier, "Expect variable name.")?.clone();
        let mut init: Option<Expr> = None;
        if self.tkn_match(&[TokenT::Equal]) {
            init = Some(self.expression()?);
        }
        
        self.consume(TokenT::Semicolon, "Expect ';' after variable declaration.")?;
        Ok(Stmt::Var { name, init })
    }

    pub fn statement(&mut self) -> Result<Stmt, ParseError> {
        if self.tkn_match(&[TokenT::Keyword(Keyword::Print)]) {
            return self.print_stmt();
        }
        
        if self.tkn_match(&[TokenT::LeftBrace]) {
            return self.block_stmt();
        }
        
        self.expr_stmt()
    }

    fn print_stmt(&mut self) -> Result<Stmt, ParseError> {
        let value = self.expression()?;
        self.consume(TokenT::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::Print(value))
    }

    fn expr_stmt(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.expression()?;
        self.consume(TokenT::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::Expression(expr))
    }

    fn block_stmt(&mut self) -> Result<Stmt, ParseError> {
        let mut stmts = Vec::new();
        while !self.check_cur(&TokenT::RightBrace) {
            stmts.push(self.declaration()?);
        }
        
        self.consume(TokenT::RightBrace, "Expect '}' after block.")?;
        Ok(Stmt::Block(stmts))
    }

    pub fn expression(&mut self) -> Result<Expr, ParseError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, ParseError> {
        let expr = self.ternary()?;
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

    fn ternary(&mut self) -> Result<Expr, ParseError> {
        let cond = self.comma();

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

    binary_rule!(comma, equality, [TokenT::Comma]);

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
        self.primary()
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