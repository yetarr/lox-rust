use crate::error::ParseError;
use crate::frontend::lexer::token::LitVal;
use crate::frontend::parser::stmt::{FunType, Stmt};

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
    pub(in super::super::parser) fn declaration(&mut self) -> Result<Stmt, ParseError> {
        if self.tkn_match(&[TokenT::Keyword(Keyword::Var)]) {
            return self.var_decl()
        } 

        if self.tkn_match(&[TokenT::Keyword(Keyword::Fun)]) {
            return self.fun_decl(FunType::Function)
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

    fn fun_decl(&mut self, fun_t: FunType) -> Result<Stmt, ParseError> {
        let name = self.consume(TokenT::Identifier, &format!("Expect {} name.", fun_t))?.clone();
        self.consume(TokenT::LeftParen, &format!("Expect '(' {} name.", fun_t))?;
        let mut params = Vec::new();
        if !self.check_cur(&TokenT::RightParen) {
            loop {
                if params.len() >= 255 {
                    self.error(&self.peek().clone(), "Can't have more than 255 parameters.");
                }

                params.push(self.consume(TokenT::Identifier, "Expect parameter name.")?.clone());

                if !self.tkn_match(&[TokenT::Comma]) {
                    break;
                }
            }
        }
        self.consume(TokenT::RightParen, "Expect ')' after parameters.")?;
        self.consume(TokenT::LeftBrace, &format!("Expect '{{' before {} body.", fun_t))?;
        Ok(Stmt::Function { name, params, body: self.block()? })
    }

    pub fn statement(&mut self) -> Result<Stmt, ParseError> {
        if self.tkn_match(&[TokenT::Keyword(Keyword::Print)]) {
            return self.print_stmt();
        }

        if self.tkn_match(&[TokenT::Keyword(Keyword::If)]) {
            return self.if_stmt();
        }

        if self.tkn_match(&[TokenT::Keyword(Keyword::While)]) {
            return self.while_stmt();
        }

        if self.tkn_match(&[TokenT::Keyword(Keyword::For)]) {
            return self.for_stmt();
        }
        
        if self.tkn_match(&[TokenT::LeftBrace]) {
            return self.block_stmt();
        }

        if self.tkn_match(&[TokenT::Keyword(Keyword::Break)]) {
            return self.break_stmt();
        }
        
        self.expr_stmt()
    }

    fn print_stmt(&mut self) -> Result<Stmt, ParseError> {
        let value = self.expression()?;
        self.consume(TokenT::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::Print(value))
    }

    fn if_stmt(&mut self) -> Result<Stmt, ParseError> {
        self.consume(TokenT::LeftParen, "Expect '(' after 'if'.")?;
        let cond = self.expression()?;
        self.consume(TokenT::RightParen, "Expext ')' after if condition.")?;
        let then = self.statement()?;
        let mut else_opt = None; 

        if self.tkn_match(&[TokenT::Keyword(Keyword::Else)]) {
            else_opt = Some(Box::new(self.statement()?));
        }

        Ok(Stmt::If { cond, then_br: Box::new(then), else_br: else_opt })
    }

    fn while_stmt(&mut self) -> Result<Stmt, ParseError> {
        self.consume(TokenT::LeftParen, "Expect '(' after 'while'.")?;
        let cond = self.expression()?;
        self.consume(TokenT::RightParen, "Expext ')' after while condition.")?;
        self.loop_depth += 1;
        let block = self.statement()?;
        self.loop_depth -= 1;

        Ok(Stmt::While { cond, block: Box::new(block) })
    }

    fn for_stmt(&mut self) -> Result<Stmt, ParseError> {
        self.consume(TokenT::LeftParen, "Expect '(' after 'while'.")?;

        let init;
        if self.tkn_match(&[TokenT::Keyword(Keyword::Var)]) {
            init = Some(self.var_decl()?);
        } else {
            init = Some(self.expr_stmt()?);
        }

        let mut cond = None;
        if !self.check_cur(&TokenT::Semicolon) {
            cond = Some(self.expression()?);
        }
        self.consume(TokenT::Semicolon, "Expect ';' after loop condition.")?;

        let mut incr = None;
        if !self.check_cur(&TokenT::RightParen) {
            incr = Some(self.expression()?);
        }
        self.consume(TokenT::RightParen, "Expect ')' after for clauses.")?;

        self.loop_depth += 1;
        let mut body = self.statement()?;
        self.loop_depth -= 1;

        if let Some(incr) = incr {
            body = Stmt::Block(vec![body, Stmt::Expression(incr)]);
        }

        body = match cond {
            Some(cond) => Stmt::While { cond, block: Box::new(body) },
            None       => Stmt::While { 
                cond: Expr::Literal(LitVal::Boolean(true)), 
                block: Box::new(body) 
            },
        };

        if let Some(init) = init {
            body = Stmt::Block(vec![init, body]);
        }
        
        Ok(body)
    }

    fn break_stmt(&mut self) -> Result<Stmt, ParseError> {
        if self.loop_depth == 0 {
            return Err(ParseError::new(self.previous().clone(), "'break' outside of loop."))
        }
        self.consume(TokenT::Semicolon, "Expect ';' after 'break'")?;
        Ok(Stmt::Break)
    }

    fn expr_stmt(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.expression()?;
        self.consume(TokenT::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::Expression(expr))
    }

    fn block_stmt(&mut self) -> Result<Stmt, ParseError> {
        Ok(Stmt::Block(self.block()?))
    }

    fn block(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut stmts = Vec::new();
        while !self.check_cur(&TokenT::RightBrace) {
            stmts.push(self.declaration()?);
        }
        
        self.consume(TokenT::RightBrace, "Expect '}' after block.")?;
        Ok(stmts)
    }

    pub fn expression(&mut self) -> Result<Expr, ParseError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, ParseError> {
        let expr = self.logic_or()?;
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