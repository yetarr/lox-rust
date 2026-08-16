use crate::parser::expr::Expr;

pub fn print(expr: Expr) {
    println!("{}", expr.expand());
}