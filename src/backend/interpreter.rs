pub mod evaluator;
pub mod error;

use crate::backend::interpreter::error::RuntimeError;
use crate::frontend::parser::stmt::Stmt;
use crate::lox::Lox;
use crate::frontend::{lexer::token::LitVal};

#[allow(dead_code)]
pub struct Interpreter<'a> {
    stmts: Vec<Stmt>,
    lox: &'a mut Lox,
}

#[allow(dead_code)]
impl<'a> Interpreter<'a> {
    pub fn new(lox: &'a mut Lox, stmts: Vec<Stmt>) -> Self {
        Interpreter { lox, stmts }
    }

    pub fn interpret(&mut self) {
        let stmts = std::mem::take(&mut self.stmts);
        for s in stmts {
            match self.exec(&s) {
                Ok(_)  => {},
                Err(e) => self.lox.error_runtime(&e),
            }
        }
    }

    fn exec(&mut self, stmt: &Stmt) -> Result<(), RuntimeError> {
        match stmt {
            Stmt::Print(val)       => println!("{}", self.eval(&val)?.to_string()),
            Stmt::Expression(expr) => match self.eval(expr) {
                Ok(_)  => {},
                Err(e) => return Err(e),
            },
        }
        Ok(())
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
                LitVal::Number(b) => a.eq(b),
                _                 => false
            }
            LitVal::String(a)  => match b {
                LitVal::String(b) => a.eq(b),
                _                 => false
            }
            LitVal::Boolean(a) => match b {
                LitVal::Boolean(b) => a.eq(b),
                _                  => false
            }
        }
    }
}