use crate::{backend::utils::{binary_op, validator}, frontend::{lexer::token::{LitVal, Token, TokenT}, parser::expr::Expr}};
use super::super::{interpreter::{Interpreter, error::RuntimeError}};

#[allow(dead_code)]
impl<'a> Interpreter<'a> {
    pub(in super::super::interpreter) fn eval(&mut self, expr: &Expr) -> Result<LitVal, RuntimeError> {
        match expr {
            Expr::Literal(lit)               => Ok(self.literal(lit)),
            Expr::Grouping(inner)            => self.grouping(inner),
            Expr::Unary { op, right }        => self.unary(op, right),
            Expr::Binary { left, op, right } => self.binary(left, op, right),
            _                                => Ok(LitVal::Nil)
        }
    }

    fn literal(&self, lit: &LitVal) -> LitVal {
        lit.clone()
    }

    fn grouping(&mut self, inner: &Expr) -> Result<LitVal, RuntimeError> {
        self.eval(inner)
    }

    fn unary(&mut self, op: &Token, right: &Expr) -> Result<LitVal, RuntimeError> {
        let right = self.eval(right)?;
        let val = match op.token_t {
            TokenT::Minus => LitVal::Number(-validator::check_num(op, right.as_number())?),
            TokenT::Bang  => LitVal::Boolean(self.is_truthy(&right)),
            _ => LitVal::Nil
        };
        Ok(val)
    }

    fn binary(&mut self, left: &Expr, op: &Token, right: &Expr) -> Result<LitVal, RuntimeError> {
        let left = self.eval(left)?;
        let right = self.eval(right)?;

        match op.token_t {
            TokenT::Minus        => binary_op::minus(op, &left, &right),
            TokenT::Plus         => binary_op::plus(op, &left, &right),
            TokenT::Slash        => binary_op::div(op, &left, &right),
            TokenT::Star         => binary_op::mult(op, &left, &right),
            TokenT::Greater      => binary_op::greater(op, &left, &right),
            TokenT::GreaterEqual => binary_op::greater_eq(op, &left, &right),
            TokenT::Less         => binary_op::less(op, &left, &right),
            TokenT::LessEqual    => binary_op::less_eq(op, &left, &right),
            TokenT::BangEqual    => binary_op::not_eq(&left, &right),
            TokenT::EqualEqual   => binary_op::eq(&left, &right),
            _                    => Ok(LitVal::Nil)
        }
    }
}