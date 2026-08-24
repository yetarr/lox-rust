use std::collections::HashMap;

use crate::{backend::interpreter::error::RuntimeError, frontend::lexer::token::{LitVal, Token}};

pub struct Environment {
    vals: HashMap<String, LitVal>,
}

impl Environment {
    pub fn new() -> Self {
        Environment { vals: HashMap::new() }
    }

    pub fn define(&mut self, name: &str, val: LitVal) {
        self.vals.insert(name.to_string(), val);
    }

    pub fn get(&mut self, name: &Token) -> Result<&LitVal, RuntimeError> {
        if let Some(val) = self.vals.get(&name.lex) {
            return Ok(val);
        }
        Err(RuntimeError::new(
            name, 
            &format!("Undefined variable '{}'.", name.lex)
        ))
    }

    pub fn contains(&mut self, name: &str) -> bool {
        self.vals.contains_key(name)
    }
}