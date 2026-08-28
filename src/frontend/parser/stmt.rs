use crate::frontend::parser::expr::Expr;
use crate::frontend::lexer::token::Token;

#[derive(Debug, Clone)]
pub enum Stmt {
    Print(Expr),
    Expression(Expr),
    Block(Vec<Stmt>),
    Var {
        name: Token,
        init: Option<Expr>
    }
}