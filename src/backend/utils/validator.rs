use crate::prelude::*;

pub fn check_num(op: &Token, x: Option<f64>) -> Result<f64, RuntimeError> {
    if let Some(x) = x {
        return Ok(x);
    }
    Err(RuntimeError::new(op, "Operands must be numbers."))
}

pub fn check_nums(op: &Token, x: Option<f64>, y: Option<f64>) -> Result<(f64, f64), RuntimeError> {
    if let (Some(x), Some(y)) = (x, y) {
        return Ok((x, y));
    }
    Err(RuntimeError::new(op, "Operands must be numbers."))
}

pub fn check_str(op: &Token, x: Option<String>) -> Result<String, RuntimeError> {
    if let Some(x) = x {
        return Ok(x);
    }
    Err(RuntimeError::new(op, "Operand must be a string."))
}

pub fn check_strs(
    op: &Token,
    x: Option<String>,
    y: Option<String>,
) -> Result<(String, String), RuntimeError> {
    if let (Some(x), Some(y)) = (x, y) {
        return Ok((x, y));
    }
    Err(RuntimeError::new(op, "Operands must be strings."))
}
