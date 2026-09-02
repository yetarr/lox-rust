use crate::prelude::*;
use crate::frontend::parser::{Parser, stmt::FunType};

impl<'a> Parser<'a> {
    pub fn declaration(&mut self) -> Result<Stmt, ParseError> {
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
        if !self.check_cur(&TokenT::Identifier) {
            return self.statement();
        }
        
        let name = self.consume(TokenT::Identifier, &format!("Expect {} name.", fun_t))?.clone();
        self.consume(TokenT::LeftParen, &format!("Expect '(' after {} name.", fun_t))?;
        
        let (params, body) = self.parameters(fun_t)?;
        Ok(Stmt::Function { name, params, body })
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

        if self.tkn_match(&[TokenT::Keyword(Keyword::Return)]) {
            return self.return_stmt();
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

    fn return_stmt(&mut self) -> Result<Stmt, ParseError> {
        let key = self.previous().clone();
        let mut val = None;
        if !self.check_cur(&TokenT::Semicolon) {
            val = Some(self.expression()?);
        }

        self.consume(TokenT::Semicolon, "Expect ';' after return value.")?;
        Ok(Stmt::Return { key, val })
    }
}