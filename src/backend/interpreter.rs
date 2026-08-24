pub mod evaluator;
pub mod error;
pub mod environment;

use crate::backend::interpreter::environment::Environment;
use crate::backend::interpreter::error::RuntimeError;
use crate::frontend::parser::stmt::Stmt;
use crate::lox::Lox;
use crate::frontend::{lexer::token::LitVal};

#[allow(dead_code)]
pub struct Interpreter<'a> {
    stmts: Vec<Stmt>,
    lox: &'a mut Lox,
    env: Environment,
}

#[allow(dead_code)]
impl<'a> Interpreter<'a> {
    pub fn new(lox: &'a mut Lox, stmts: Vec<Stmt>) -> Self {
        Interpreter { lox, stmts, env: Environment::new() }
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
            Stmt::Print(val)        => println!("{}", self.eval(&val)?.to_string()),
            Stmt::Expression(expr)  => match self.eval(expr) {
                Ok(_)  => {},
                Err(e) => return Err(e),
            },
            Stmt::Var { name, init } => {
                let mut val = LitVal::Nil;
                if let Some(i) = init {
                    val = self.eval(i)?;
                }
                self.env.define(&name.lex, val);
            }
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