use crate::frontend::{lexer::token::Token, parser::expr::Expr};

#[derive(Clone)]
pub enum Stmt {
    Print(Expr),
    Expression(Expr),
    Var {
        name: Token,
        init: Option<Expr>
    }
}