pub mod evaluator;
pub mod environment;

use crate::backend::interpreter::environment::Environment;
use crate::error::RuntimeError;
use crate::frontend::parser::stmt::Stmt;
use crate::lox::Lox;
use crate::frontend::{lexer::token::LitVal};

#[allow(dead_code)]
pub struct Interpreter<'a> {
    stmts: &'a [Stmt],
    lox: &'a mut Lox,
    env: Environment,
}

impl<'a> Interpreter<'a> {
    pub fn empty(lox: &'a mut Lox) -> Self {
        Interpreter { lox, stmts: &[], env: Environment::global() }
    }
    
    pub fn new(lox: &'a mut Lox, stmts: &'a [Stmt]) -> Self {
        Interpreter { lox, stmts, env: Environment::global() }
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
            Stmt::Print(val)                    => println!("{}", self.eval(&val)?.to_string()),
            Stmt::Expression(expr)              => { self.eval(expr)?; },
            Stmt::Block(stmts)                  => self.exec_block(stmts)?,
            Stmt::If { cond, then_br, else_br } => {
                let cond_val = self.eval(cond)?;
                if self.is_truthy(&cond_val) {
                    self.exec(then_br)?;
                } else {
                    if let Some(else_br) = else_br {
                        self.exec(else_br)?;
                    }
                }
            }
            Stmt::While { cond, block }         => {
                loop {
                    let cond_val = self.eval(cond)?;
                    if self.is_truthy(&cond_val) {
                        self.exec(block)?;
                    } else {
                        break;
                    }
                }
            }
            Stmt::Var { name, init }            => {
                let val = match init {
                    Some(i) => Some(self.eval(i)?),
                    None    => None,
                };
                self.env.define(&name.lex, val);
            }
        }
        Ok(())
    }

    fn exec_block(&mut self, stmts: &Vec<Stmt>) -> Result<(), RuntimeError> {
        let prev_env = std::mem::take(&mut self.env);
        self.env = Environment::enclose(prev_env);
        for stmt in stmts {
            self.exec(&stmt)?;
        }
        self.env = self.env.take_env().unwrap();
        Ok(())
    }

    fn is_truthy(&self, val: &LitVal) -> bool {
        match val {
            LitVal::Boolean(b) => *b,
            LitVal::Nil        => false,
            _others            => true
        }
    }
}