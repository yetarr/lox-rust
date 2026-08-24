use std::fmt::Display;

use super::super::lexer::token::{LitVal, Token};

#[derive(Clone)]
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
    Ternary {
        cond: Box<Expr>,
        first: Box<Expr>,
        second: Box<Expr>
    },
    Assign {
        name: Token,
        val: Box<Expr>,
    },
    Variable(Token)
}

impl Expr {
    pub fn expand(&self) -> String {
        match self {
            Self::Literal(v)                      => v.to_string(),
            Self::Grouping(expr)                  => parenthesize("group", &[expr]),
            Self::Binary { left, op, right }      => parenthesize(&op.lex, &[left, right]),
            Self::Unary { op, right }             => parenthesize(&op.lex, &[right]),
            Self::Ternary { cond, first, second } => parenthesize("?:", &[cond, first, second]),
            Self::Assign { name, val }            => parenthesize(&format!("={}", name.lex), &[val]),
            Self::Variable(id)                    => parenthesize(&format!("var {}", id.lex), &[])
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

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.expand())
    }
}