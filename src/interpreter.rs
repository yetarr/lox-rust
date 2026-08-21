pub mod evaluator;
pub mod error;

use crate::{lexer::token::LitVal, parser::expr::Expr};

#[allow(dead_code)]
pub struct Interpreter {
    exprs: Vec<Expr>,
}

#[allow(dead_code)]
impl Interpreter {
    fn is_truthy(&self, val: &LitVal) -> bool {
        match val {
            LitVal::Boolean(b) => *b,
            LitVal::Nil        => false,
            _others            => true
        }
    }

    fn is_equal(&self, a: &LitVal, b: &LitVal) -> bool {
        match a {
            LitVal::Nil => match b {
                LitVal::Nil => true,
                _           => false
            }
            LitVal::Number(a)  => match b {
                LitVal::Number(b) => a == b,
                _                 => false
            }
            LitVal::String(a)  => match b {
                LitVal::String(b) => a.eq(b),
                _                 => false
            }
            LitVal::Boolean(a) => match b {
                LitVal::Boolean(b) => a == b,
                _                  => false
            }
        }
    }
}