use std::collections::HashMap;

use crate::{error::RuntimeError, frontend::lexer::token::{LitVal, Token}};

#[derive(Default)]
pub struct Environment {
    vals: HashMap<String, LitVal>,
    enclosing: Option<Box<Environment>>,
}

impl Environment {
    pub fn global() -> Self {
        Environment { vals: HashMap::new(), enclosing: None }
    }
    
    pub fn enclose(enclosing: Self) -> Self {
        Environment { vals: HashMap::new(), enclosing: Some(Box::new(enclosing)) }
    }

    pub fn take_enc(&mut self) -> Option<Self> {
        if let Some(env) = &mut self.enclosing {
            return Some(std::mem::take(env));
        } 
        None
    }
    
    pub fn define(&mut self, name: &str, val: LitVal) {
        self.vals.insert(name.to_string(), val);
    }

    pub fn assign(&mut self, name: &Token, val: LitVal) -> Result<(), RuntimeError> {
        if let Some(prev) = self.vals.get_mut(&name.lex) {
            *prev = val;
            return Ok(());
        } 

        if let Some(enc) = &mut self.enclosing {
            return enc.assign(name, val);
        }

        Err(RuntimeError::new(
            name, 
            &format!("Undefined variable '{}'.", name.lex)
        ))
    }

    pub fn get(&self, name: &Token) -> Result<&LitVal, RuntimeError> {
        if let Some(val) = self.vals.get(&name.lex) {
            return Ok(val);
        }
        
        if let Some(enc) = &self.enclosing {
            return enc.get(name);
        }

        Err(RuntimeError::new(
            name, 
            &format!("Undefined variable '{}'.", name.lex)
        ))
    }
}