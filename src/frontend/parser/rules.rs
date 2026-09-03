pub mod expressions;
pub mod statements;

use crate::frontend::parser::Parser;
use crate::frontend::parser::stmt::FunType;
use crate::prelude::*;

impl<'a> Parser<'a> {
    pub(super) fn block(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut stmts = Vec::new();
        while !self.check_cur(&TokenT::RightBrace) {
            stmts.push(self.declaration()?);
        }

        self.consume(TokenT::RightBrace, "Expect '}' after block.")?;
        Ok(stmts)
    }

    pub(super) fn parameters(
        &mut self,
        fun_t: FunType,
    ) -> Result<(Vec<Token>, Vec<Stmt>), ParseError> {
        let mut params = Vec::new();
        if !self.check_cur(&TokenT::RightParen) {
            loop {
                if params.len() >= 255 {
                    self.error(&self.peek().clone(), "Can't have more than 255 parameters.");
                }

                params.push(
                    self.consume(TokenT::Identifier, "Expect parameter name.")?
                        .clone(),
                );

                if !self.tkn_match(&[TokenT::Comma]) {
                    break;
                }
            }
        }
        self.consume(TokenT::RightParen, "Expect ')' after parameters.")?;
        self.consume(
            TokenT::LeftBrace,
            &format!("Expect '{{' before {} body.", fun_t),
        )?;
        Ok((params, self.block()?))
    }
}
