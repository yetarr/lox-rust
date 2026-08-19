use std::{fs::File, io::Read};
use std::io::{Write, stdin, stdout};

use anyhow::Result;

use crate::parser::Parser;
use crate::parser::expr::Expr;
use crate::lexer::Scanner;
use crate::lexer::token::{LitVal, Token, TokenT};

pub struct Lox {
    had_err: bool,
}

impl Lox {
    pub fn new() -> Self {
        Lox { had_err: false }
    }
    
    pub fn run_file(&mut self, file: String) -> Result<()> {
        let mut file = File::open(file)?;
        let mut code = String::new();
        file.read_to_string(&mut code)?;
        self.run(code);
    
        Ok(())
    }

    pub fn run_prompt(&mut self) -> Result<()> {
        let stdin = stdin();
        let buf = &mut String::new();
    
        loop {
            buf.clear();
            print!("> ");
            stdout().flush().unwrap();
            if let Ok(c) = stdin.read_line(buf) {
                if c == 0 {
                    println!();
                    break;
                }
            }
    
            self.run(buf.clone().trim_end().to_string());
            self.had_err = false;
        }
    
        Ok(())
    }
    
    fn run(&mut self, code: String) {
        let tkns = {
            let mut scr = Scanner::new(code, self);
            scr.scan_tokens()
        };

        let expr = {
             let mut prs = Parser::new(tkns, self);
             prs.parse()
        };

        if self.had_err { 
            return;
        }
        
        match expr {
            Some(expr) => println!("{}", expr),
            None       => println!("{}", Expr::Literal(LitVal::Nil)) 
        }
    }

    pub fn error_simple(&mut self, ln: usize, msg: &str) {
        self.report(ln, "", msg);
    }

    pub fn error_parse(&mut self, tkn: &Token, msg: &str) {
        match tkn.token_t {
            TokenT::EOF => self.report(tkn.ln, " at end", msg),
            _           => self.report(tkn.ln, &format!(" at '{}'", tkn.lex), msg),
        }
    }

    fn report(&mut self, ln: usize, loc: &str, msg: &str) {
        eprintln!("[line {}] Error{}: {}", ln, loc, msg);
        self.had_err = true;
    }
}