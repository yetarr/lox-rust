use crate::frontend::{parser::expr::Expr, lexer::token::{Token, TokenT, LitVal}};
use super::super::{interpreter::{Interpreter, error::RuntimeError}};

#[allow(dead_code)]
impl<'a> Interpreter<'a> {
    pub(in super::super::interpreter) fn evaluate(&mut self, expr: &Expr) -> LitVal {
        let res = match expr {
            Expr::Literal(lit)               => Ok(self.literal(lit)),
            Expr::Grouping(inner)            => Ok(self.grouping(inner)),
            Expr::Unary { op, right }        => self.unary(op, right),
            Expr::Binary { left, op, right } => self.binary(left, op, right),
            _                                => Ok(LitVal::Nil)
        };
        
        match res {
            Ok(val) => {
                println!("{}", val.to_string());
                val
            },
            Err(e)  => {
                self.lox.error_runtime(e);
                LitVal::Nil
            },
        }
    }

    fn literal(&self, lit: &LitVal) -> LitVal {
        lit.clone()
    }

    fn grouping(&mut self, inner: &Expr) -> LitVal {
        self.evaluate(inner)
    }

    fn unary(&mut self, op: &Token, right: &Expr) -> Result<LitVal, RuntimeError> {
        let right = self.evaluate(right);
        let val = match op.token_t {
            TokenT::Minus => LitVal::Number(-self.check_num(op, right.as_number())?),
            TokenT::Bang  => LitVal::Boolean(self.is_truthy(&right)),
            _ => LitVal::Nil
        };
        Ok(val)
    }

    fn binary(&mut self, left: &Expr, op: &Token, right: &Expr) -> Result<LitVal, RuntimeError> {
        let left = self.evaluate(left);
        let right = self.evaluate(right);

       let res = match op.token_t {
            TokenT::Minus => {
                let (x, y) = self.check_nums(op, left.as_number(), right.as_number())?;
                LitVal::Number(x - y)
            }
            
            TokenT::Plus  => match left {
                LitVal::Number(_) => {
                    let (x, y) = self.check_nums(op, left.as_number(), right.as_number())?;
                    LitVal::Number(x + y)
                }

                LitVal::String(_) => {
                    let (mut x, y) = self.check_strs(op, left.as_string(), right.as_string())?;
                    x.push_str(&y);
                    LitVal::String(x)
                }

                _ => return Err(
                    RuntimeError::new(op, "Operands must be two numbers or two strings")
                ),
            },

            TokenT::Slash => {
                let (x, y) = self.check_nums(op, left.as_number(), right.as_number())?;
                LitVal::Number(x / y)
            }

            TokenT::Star  => {
                let (x, y) = self.check_nums(op, left.as_number(), right.as_number())?;
                LitVal::Number(x * y)
            }

            TokenT::Greater => {
                let (x, y) = self.check_nums(op, left.as_number(), right.as_number())?;
                LitVal::Boolean(x > y)
            }

            TokenT::GreaterEqual => {
                let (x, y) = self.check_nums(op, left.as_number(), right.as_number())?;
                LitVal::Boolean(x >= y)
            }

            TokenT::Less => {
                let (x, y) = self.check_nums(op, left.as_number(), right.as_number())?;
                LitVal::Boolean(x < y)
            }

            TokenT::LessEqual => {
                let (x, y) = self.check_nums(op, left.as_number(), right.as_number())?;
                LitVal::Boolean(x <= y)
            }

            TokenT::BangEqual => LitVal::Boolean(!self.is_equal(&left, &right)),

            TokenT::EqualEqual => LitVal::Boolean(self.is_equal(&left, &right)),

            _ => LitVal::Nil
        };
        Ok(res)
    }

    fn check_num(&self, op: &Token, x: Option<f64>) 
            -> Result<f64, RuntimeError>
    {
        if let Some(x) = x {
            return Ok(x)
        }
        Err(RuntimeError::new(op, "Operands must be numbers."))
    }

    fn check_nums(&self, op: &Token, x: Option<f64>, y: Option<f64>) 
            -> Result<(f64, f64), RuntimeError>
    {
        if let (Some(x), Some(y)) = (x, y) {
            return Ok((x, y))
        }
        Err(RuntimeError::new(op, "Operands must be numbers."))
    }

    fn check_strs(&self, op: &Token, x: Option<String>, y: Option<String>) 
            -> Result<(String, String), RuntimeError>
    {
        if let (Some(x), Some(y)) = (x, y) {
            return Ok((x, y))
        }
        Err(RuntimeError::new(op, "Operands must be strings."))
    }
}