use crate::frontend::parser::expr::Expr;
use crate::frontend::lexer::token::Token;

#[derive(Debug, Clone)]
pub enum Stmt {
    Print(Expr),
    Expression(Expr),
    Block(Vec<Stmt>),
    If {
        cond: Expr,
        then_br: Box<Stmt>,
        else_br: Option<Box<Stmt>>,
    },
    While {
        cond: Expr,
        block: Box<Stmt>
    },
    Var {
        name: Token,
        init: Option<Expr>,
    },
}