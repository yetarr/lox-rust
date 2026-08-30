use crate::error::RuntimeError;
use crate::frontend::lexer::token::LitVal;
use crate::backend::interpreter::Interpreter;


pub trait Callable: std::fmt::Debug {
    fn arity(&self) -> usize;
    fn to_string(&self) -> String;
    fn call(&self, intr: &mut Interpreter, args: Vec<LitVal>) -> Result<LitVal, RuntimeError>;
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