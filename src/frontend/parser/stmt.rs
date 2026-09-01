use std::fmt::Display;

use crate::frontend::parser::expr::Expr;
use crate::frontend::lexer::token::Token;

#[derive(Debug, Clone)]
pub enum Stmt {
    Print(Expr),
    Expression(Expr),
    Block(Vec<Stmt>),
    Break,
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
    Function {
        name: Token,
        params: Vec<Token>,
        body: Vec<Stmt>
    },
    Return {
        key: Token,
        val: Option<Expr>,
    }
}

pub enum FunType {
    Function,
    Method
}

impl Display for FunType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FunType::Function => write!(f, "function"),
            FunType::Method   => write!(f, "method")
        }
    }
}