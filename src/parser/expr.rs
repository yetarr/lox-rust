use crate::lexer::token::{LitVal, Token};

pub enum Expr {
    Grouping(Box<Expr>),
    Literal(LitVal),
    Binary {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>
    },
    Unary {
        op: Token,
        right: Box<Expr>
    },
}

impl Expr {
    pub fn expand(&self) -> String {
        match self {
            Self::Literal(v)                 => v.to_string(),
            Self::Grouping(expr)             => parenthesize("group", &[expr]),
            Self::Binary { left, op, right } => parenthesize(&op.lex, &[left, right]),
            Self::Unary { op, right }        => parenthesize(&op.lex, &[right]),
        }
    }
}

fn parenthesize(val: &str, exprs: &[&Expr]) -> String {
    let mut str = format!("({}", val);
    for expr in exprs {
        str = format!("{} {}", str, expr.expand());
    }
    str.push(')');
    str
}