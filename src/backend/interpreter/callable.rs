use crate::prelude::*;
use crate::backend::interpreter::environment::Environment;
use crate::backend::interpreter::{ExecCode, Interpreter};

pub trait Callable: std::fmt::Debug {
    fn arity(&self) -> usize;
    fn to_string(&self) -> String;
    fn call(&self, intr: &mut Interpreter, args: Vec<LitVal>) -> Result<LitVal, RuntimeError>;
}

#[derive(Debug, Clone)]
pub struct LoxFn {
    name: Token,
    params: Vec<Token>,
    body: Vec<Stmt>
}

impl LoxFn {
    pub fn new(name: Token, params: Vec<Token>, body: Vec<Stmt>) -> Self {
        LoxFn { name, params, body }
    }
}

impl Callable for LoxFn {
    fn arity(&self) -> usize {
        self.params.len()
    }

    fn to_string(&self) -> String {
        format!("<fn {}>", self.name.lex)
    }

    fn call(&self, intr: &mut Interpreter, mut args: Vec<LitVal>) -> Result<LitVal, RuntimeError> {
        let prev_env = std::mem::take(&mut intr.env);
        let mut env = Environment::enclose(prev_env);

        args.reverse();
        self.params.iter().for_each(|p| {
            env.define(&p.lex, args.pop());
        });

        if let ExecCode::Return(v) = intr.exec_block(&self.body, env)? {
            return Ok(v);
        }
        Ok(LitVal::Nil)
    }
}

#[derive(Debug)]
pub struct AnonymousFn {
    params: Vec<Token>,
    body: Vec<Stmt>
}

impl AnonymousFn {
    pub fn new(params: Vec<Token>, body: Vec<Stmt>) -> Self {
        AnonymousFn { params, body }
    }
}

impl Callable for AnonymousFn {
    fn arity(&self) -> usize {
        self.params.len()
    }

    fn to_string(&self) -> String {
        String::from("<anonymous fn>")
    }

    fn call(&self, intr: &mut Interpreter, mut args: Vec<LitVal>) -> Result<LitVal, RuntimeError> {
        let prev_env = std::mem::take(&mut intr.env);
        let mut env = Environment::enclose(prev_env);

        args.reverse();
        self.params.iter().for_each(|p| {
            env.define(&p.lex, args.pop());
        });

        if let ExecCode::Return(v) = intr.exec_block(&self.body, env)? {
            return Ok(v);
        }
        Ok(LitVal::Nil)
    }
}

pub struct NativeFn {
    pub arity: usize,
    pub func: Box<dyn Fn(&mut Interpreter, Vec<LitVal>) -> Result<LitVal, RuntimeError>>,
}

impl std::fmt::Debug for NativeFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn>")
    }
}

impl Callable for NativeFn {
    fn arity(&self) -> usize {
        self.arity
    }

    fn to_string(&self) -> String {
        String::from("<native fn>")
    }
    
    fn call(&self, interpreter: &mut Interpreter, args: Vec<LitVal>) -> Result<LitVal, RuntimeError> {
        (self.func)(interpreter, args)
    }
}