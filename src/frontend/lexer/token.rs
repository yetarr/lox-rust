use std::rc::Rc;

use crate::backend::interpreter::callable::Callable;

#[derive(Debug, Clone)]
pub struct Token {
    pub token_t: TokenT,
    pub lex: String,
    pub lit: LitVal,
    pub ln: usize
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenT {
    // Single-character tokens
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,
    Colon, Query,

    // One or two character tokens
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    // Literals
    Keyword(Keyword), Identifier, Literal,

    EOF
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Keyword {
    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While, 
    Break
}

#[derive(Debug, Clone)]
pub enum LitVal {
    String(String),
    Number(f64),
    Boolean(bool),
    Nil,
    Callable(Rc<dyn Callable>),
}

impl PartialEq for LitVal {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (LitVal::String(a), LitVal::String(b)) => a == b,
            (LitVal::Number(a), LitVal::Number(b)) => a == b,
            (LitVal::Boolean(a), LitVal::Boolean(b)) => a == b,
            (LitVal::Nil, LitVal::Nil) => true,
            (LitVal::Callable(a), LitVal::Callable(b)) => Rc::ptr_eq(a, b),
            _ => false,
        }
    }
}

impl LitVal {
    pub fn to_string(&self) -> String {
        match self {
            LitVal::String(s)      => s.clone(),
            LitVal::Number(n)      => n.to_string(),
            LitVal::Boolean(b)     => b.to_string(),
            LitVal::Nil            => String::from("nil"),
            LitVal::Callable(func) => func.to_string(),
        }
    }
    
    pub fn as_number(&self) -> Option<f64> {
        match self {
            LitVal::Number(n) => Some(*n),
            _ => None,
        }
    }
    
    pub fn as_string(&self) -> Option<String> {
        match self {
            LitVal::String(s) => Some(s.clone()),
            _ => None,
        }
    }
    
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            LitVal::Boolean(b) => Some(*b),
            _ => None,
        }
    }
}

impl Token {
    pub fn new(token_t: TokenT, lex: String, lit: LitVal, ln: usize) -> Self {
        Token {
            token_t,
            lex,
            lit,
            ln
        }
    }
    
    pub fn to_string(&self) -> String {
        format!("{:?} {}", self.token_t, self.lex)
    }
}