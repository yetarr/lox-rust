pub mod evaluator;
pub mod error;

use crate::lox::Lox;
use crate::frontend::{lexer::token::LitVal, parser::expr::Expr};

#[allow(dead_code)]
pub struct Interpreter<'a> {
    lox: &'a mut Lox,
    expr: Expr,
}

#[allow(dead_code)]
impl<'a> Interpreter<'a> {
    pub fn new(lox: &'a mut Lox, expr: Expr) -> Self {
        Interpreter { lox, expr }
    }

    pub fn interpret(&mut self) -> LitVal {
        self.evaluate(&self.expr.clone())
    }

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