pub mod evaluator;
pub mod environment;
pub mod callable;

use std::rc::Rc;

use crate::prelude::*;
use crate::backend::interpreter::callable::LoxFn;
use crate::backend::interpreter::environment::Environment;
use crate::backend::native_fn;
use crate::lox::Lox;

enum ExecCode {
    Success,
    Return(LitVal),
    Break,
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
            Stmt::Print(val)                      => println!("{}", self.eval(&val)?.to_string()),
            Stmt::Expression(expr)                => { self.eval(expr)?; },
            Stmt::Break                           => return Ok(ExecCode::Break),
            Stmt::Block(stmts)                    => {
                let prev_env = std::mem::take(&mut self.env);
                let env = Environment::enclose(prev_env);
                let code = self.exec_block(stmts, env);
                return code;
            }
            Stmt::If { cond, then_br, else_br }   => {
                let cond_val = self.eval(cond)?;
                if self.is_truthy(&cond_val) {
                    return self.exec(then_br);
                } else {
                    if let Some(else_br) = else_br {
                        return self.exec(else_br);
                    }
                }
            }
            Stmt::While { cond, block }           => {
                loop {
                    let cond_val = self.eval(cond)?;
                    if self.is_truthy(&cond_val) {
                        match self.exec(block)? {
                            ExecCode::Success   => {},
                            ExecCode::Break     => break,
                            ExecCode::Return(v) => return Ok(ExecCode::Return(v))
                        }
                    } else {
                        break;
                    }
                }
            }
            Stmt::Var { name, init }              => {
                let val = match init {
                    Some(i) => Some(self.eval(i)?),
                    None    => None,
                };
                self.env.define(&name.lex, val);
            }
            Stmt::Function { name, params, body } => {
                let fun = Rc::new(
                    LoxFn::new(name.clone(), params.clone(), body.clone())
                );
                self.env.define(&name.lex, Some(LitVal::Callable(fun)));
            }
            Stmt::Return { key: _, val }          => {
                let val = match val {
                    Some(v) => self.eval(v)?,
                    None    => LitVal::Nil, 
                };
                return Ok(ExecCode::Return(val));
            }
        };
        
        Ok(ExecCode::Success)
    }

    fn exec_block(&mut self, stmts: &Vec<Stmt>, env: Environment) -> Result<ExecCode, RuntimeError> {
        self.env = env;
        for stmt in stmts {
            match self.exec(stmt)? {
                ExecCode::Success   => {},
                ExecCode::Break     => return Ok(ExecCode::Break),
                ExecCode::Return(v) => return Ok(ExecCode::Return(v))
            }
        }
        self.env = self.env.take_env().unwrap();
        Ok(ExecCode::Success)
    }

    fn is_truthy(&self, val: &LitVal) -> bool {
        match val {
            LitVal::Boolean(b) => *b,
            LitVal::Nil        => false,
            _others            => true
        }
    }
}