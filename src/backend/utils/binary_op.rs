use crate::frontend::lexer::token::{LitVal, Token};
use crate::backend::{interpreter::error::RuntimeError, utils::validator::check_nums};

pub fn plus(op: &Token, x: &LitVal, y: &LitVal) -> Result<LitVal, RuntimeError> {
    if let Some(s) = x.as_string() {
        return Ok(string(s.clone(), y.to_string()));
    }
    
    if let Some(s) = y.as_string() {
        return Ok(string(x.to_string(), s.clone()));
    }
    
    let (a, b) = check_nums(op, x.as_number(), y.as_number())?;
    Ok(LitVal::Number(a + b))
}

pub fn minus(op: &Token, x: &LitVal, y: &LitVal) -> Result<LitVal, RuntimeError> {
    let (x, y) = check_nums(op, x.as_number(), y.as_number())?;
    Ok(LitVal::Number(x - y))
}

pub fn mult(op: &Token, x: &LitVal, y: &LitVal) -> Result<LitVal, RuntimeError> {
    let (x, y) = check_nums(op, x.as_number(), y.as_number())?;
    Ok(LitVal::Number(x * y))
}

pub fn div(op: &Token, x: &LitVal, y: &LitVal) -> Result<LitVal, RuntimeError> {
    let (x, y) = check_nums(op, x.as_number(), y.as_number())?;
    Ok(LitVal::Number(x / y))
}

pub fn greater(op: &Token, x: &LitVal, y: &LitVal) -> Result<LitVal, RuntimeError> {
    let (x, y) = check_nums(op, x.as_number(), y.as_number())?;
    Ok(LitVal::Boolean(x > y))
}

pub fn greater_eq(op: &Token, x: &LitVal, y: &LitVal) -> Result<LitVal, RuntimeError> {
    let (x, y) = check_nums(op, x.as_number(), y.as_number())?;
    Ok(LitVal::Boolean(x >= y))
}

pub fn less(op: &Token, x: &LitVal, y: &LitVal) -> Result<LitVal, RuntimeError> {
    let (x, y) = check_nums(op, x.as_number(), y.as_number())?;
    Ok(LitVal::Boolean(x < y))
}

pub fn less_eq(op: &Token, x: &LitVal, y: &LitVal) -> Result<LitVal, RuntimeError> {
    let (x, y) = check_nums(op, x.as_number(), y.as_number())?;
    Ok(LitVal::Boolean(x <= y))
}

pub fn eq(x: &LitVal, y: &LitVal) -> Result<LitVal, RuntimeError> {
    Ok(LitVal::Boolean(is_eq(x, y)))
}

pub fn not_eq(x: &LitVal, y: &LitVal) -> Result<LitVal, RuntimeError> {
    Ok(LitVal::Boolean(!is_eq(x, y)))
}

fn string(mut x: String, y: String) -> LitVal {
    x.push_str(&y);
    LitVal::String(x)
}

fn is_eq(x: &LitVal, y: &LitVal) -> bool {
    match x {
        LitVal::Nil => match y {
            LitVal::Nil => true,
            _           => false
        }
        LitVal::Number(n1)  => match y {
            LitVal::Number(n2) => n1 == n2,
            _                 => false
        }
        LitVal::String(s1)  => match y {
            LitVal::String(s2) => s1.eq(s2),
            _                 => false
        }
        LitVal::Boolean(b1) => match y {
            LitVal::Boolean(b2) => b1 == b2,
            _                  => false
        }
    }
}