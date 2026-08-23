use crate::frontend::parser::expr::Expr;

#[derive(Clone)]
pub enum Stmt {
    Print(Expr),
    Expression(Expr),
}