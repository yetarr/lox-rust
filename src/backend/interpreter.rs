pub mod evaluator;
pub mod environment;
pub mod callable;

use crate::backend::interpreter::environment::Environment;
use crate::backend::native_fn;
use crate::error::RuntimeError;
use crate::frontend::parser::stmt::Stmt;
use crate::lox::Lox;
use crate::frontend::{lexer::token::LitVal};

enum ExecCode {
    SUCCESS,
    BREAK,
}

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
        let mut global = Environment::global();
        global.define("clock", Some(native_fn::clock()));
        
        Interpreter { lox, stmts, env: global }
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

    fn exec(&mut self, stmt: &Stmt) -> Result<ExecCode, RuntimeError> {  
        match stmt {
            Stmt::Print(val)                    => println!("{}", self.eval(&val)?.to_string()),
            Stmt::Expression(expr)              => { self.eval(expr)?; },
            Stmt::Block(stmts)                  => return self.exec_block(stmts),
            Stmt::Break                         => return Ok(ExecCode::BREAK),
            Stmt::If { cond, then_br, else_br } => {
                let cond_val = self.eval(cond)?;
                if self.is_truthy(&cond_val) {
                    return self.exec(then_br);
                } else {
                    if let Some(else_br) = else_br {
                        return self.exec(else_br);
                    }
                }
            }
            Stmt::While { cond, block }         => {
                loop {
                    let cond_val = self.eval(cond)?;
                    if self.is_truthy(&cond_val) {
                        match self.exec(block)? {
                            ExecCode::SUCCESS => {},
                            ExecCode::BREAK   => break,
                        }
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
        
        Ok(ExecCode::SUCCESS)
    }

    fn exec_block(&mut self, stmts: &Vec<Stmt>) -> Result<ExecCode, RuntimeError> {
        let prev_env = std::mem::take(&mut self.env);
        self.env = Environment::enclose(prev_env);
        for stmt in stmts {
            match self.exec(stmt)? {
                ExecCode::SUCCESS => {},
                ExecCode::BREAK   => return Ok(ExecCode::BREAK),
            }
        }
        self.env = self.env.take_env().unwrap();
        Ok(ExecCode::SUCCESS)
    }

    fn is_truthy(&self, val: &LitVal) -> bool {
        match val {
            LitVal::Boolean(b) => *b,
            LitVal::Nil        => false,
            _others            => true
        }
    }
}