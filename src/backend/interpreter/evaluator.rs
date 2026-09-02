use std::rc::Rc;

use crate::backend::interpreter::callable::AnonymousFn;
use crate::error::RuntimeError;
use crate::frontend::parser::expr::Expr;
use crate::frontend::lexer::token::{Keyword, LitVal, Token, TokenT};
use crate::backend::utils::{binary_op, validator};
use crate::backend::interpreter::Interpreter;
use crate::frontend::parser::stmt::Stmt;

impl<'a> Interpreter<'a> {
    pub fn eval(&mut self, expr: &Expr) -> Result<LitVal, RuntimeError> {
        match expr {
            Expr::Literal(lit)                 => Ok(self.eval_lit(lit)),
            Expr::Grouping(inner)              => self.eval_group(inner),
            Expr::Unary { op, right }          => self.eval_unary(op, right),
            Expr::Binary { left, op, right }   => self.eval_bin(left, op, right),
            Expr::Logical { left, op, right }  => self.eval_logic(left, op, right),
            Expr::Variable(name)               => self.eval_var(name),
            Expr::Assign { name, val }         => self.eval_assign(name, val),
            Expr::AnonFun { params, body }     => self.eval_anon_fun(params.clone(), body.clone()),
            Expr::Call { callee, paren, args } => self.eval_call(callee, paren, args),
            _                                  => Ok(LitVal::Nil)
        }
    }

    fn eval_lit(&self, lit: &LitVal) -> LitVal {
        lit.clone()
    }

    fn eval_var(&mut self, name: &Token) -> Result<LitVal, RuntimeError> {
        Ok(self.env.get(name)?.clone())
    }

    fn eval_assign(&mut self, name: &Token, val: &Expr) -> Result<LitVal, RuntimeError> {
        let val = self.eval(val)?;
        self.env.assign(name, val.clone())?;
        Ok(val)
    }

    fn eval_group(&mut self, inner: &Expr) -> Result<LitVal, RuntimeError> {
        self.eval(inner)
    }

    fn eval_logic(&mut self, left: &Expr, op: &Token, right: &Expr) -> Result<LitVal, RuntimeError> {
        let left = self.eval(left)?;
        match op.token_t {
            TokenT::Keyword(Keyword::Or)  => {
                if self.is_truthy(&left) { return Ok(left) }
            },
            TokenT::Keyword(Keyword::And) => {
                if !self.is_truthy(&left) { return Ok(left) }
            },
            _ => {}
        }
        self.eval(right)
    }

    fn eval_unary(&mut self, op: &Token, right: &Expr) -> Result<LitVal, RuntimeError> {
        let right = self.eval(right)?;
        let val = match op.token_t {
            TokenT::Minus => LitVal::Number(-validator::check_num(op, right.as_number())?),
            TokenT::Bang  => LitVal::Boolean(self.is_truthy(&right)),
            _ => LitVal::Nil
        };
        Ok(val)
    }

    fn eval_bin(&mut self, left: &Expr, op: &Token, right: &Expr) -> Result<LitVal, RuntimeError> {
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

    fn eval_anon_fun(&mut self, params: Vec<Token>, body: Vec<Stmt>) -> Result<LitVal, RuntimeError> {
        let fun = AnonymousFn::new(params, body);
        Ok(LitVal::Callable(Rc::new(fun)))
    }

    fn eval_call(&mut self, callee: &Expr, paren: &Token, args: &Vec<Expr>) -> Result<LitVal, RuntimeError> {
        let callee = self.eval(callee)?;
        let mut args_val = Vec::new();
        for arg in args {
            args_val.push(self.eval(arg)?);
        }

        match callee {
            LitVal::Callable(func) => {
                if func.arity() != args.len() {
                    return Err(
                        RuntimeError::new(
                            paren, 
                            &format!(
                                "Expected {} arguments but got {}", 
                                func.arity(), 
                                args_val.len())
                        )
                    );
                }

                func.call(self, args_val)
            }
            _ => return Err(RuntimeError::new(paren, "Can only call functions and classes."))
        }
    }
}