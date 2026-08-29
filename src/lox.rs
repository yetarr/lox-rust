use std::process::exit;
use std::{fs::File, io::Read};
use std::io::{Write, stdin, stdout};

use anyhow::Result;

use crate::backend::interpreter::Interpreter;
use crate::error::{LoxErr, RuntimeError};
use crate::frontend::parser::Parser;
use crate::frontend::lexer::Scanner;
use crate::frontend::lexer::token::{Token, TokenT};

const DATA_FORMAT_ERROR: i32 = 65;
const INTERNAL_SOFTWARE_ERROR: i32 = 70;

pub struct Lox {
    errors: Vec<LoxErr>,
    had_parse_err: bool,
    had_runtime_err: bool,
}

impl Lox {
    pub fn new() -> Self {
        Lox { errors: Vec::new(), had_parse_err: false, had_runtime_err: false }
    }

    fn reset(&mut self) {
        self.had_parse_err = false;
        self.had_runtime_err = false;
        self.errors.clear();
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
            self.reset();
        }
    
        Ok(())
    }

    
    fn run(&mut self, code: String) {
        let tkns = {
            let mut scr = Scanner::new(code, self);
            scr.scan_tokens()
        };
        self.report_errors();

        let stmts = {
            let mut prs = Parser::new(&tkns, self);
            prs.parse()
        };

        match stmts {
            Some(stmts) => {
                if self.had_parse_err {
                    self.report_errors();
                    exit(DATA_FORMAT_ERROR)
                }

                let mut intr = Interpreter::new(self, &stmts);
                intr.interpret();

                if self.had_runtime_err {
                    self.report_errors();
                    exit(INTERNAL_SOFTWARE_ERROR)
                }
            }
            None => {    
                let expr = { 
                    let mut parser = Parser::new(&tkns, self);
                    parser.expression()
                };
                if let Ok(expr) = expr {
                    let mut intr = Interpreter::empty(self);
                    if let Ok(val) = intr.eval(&expr) {
                        println!("{}", val.to_string());
                    } else {
                        self.report_errors();
                    }
                } else {
                    self.report_errors();
                }
            }
        }
    }

    pub fn error_simple(&mut self, ln: usize, msg: &str) {
        let msg = format!("[line {}] Error{}: {}", ln, "", msg);
        self.errors.push(LoxErr::new(msg));
    }

    pub fn error_parse(&mut self, tkn: &Token, msg: &str) {
        match tkn.token_t {
            TokenT::EOF => {
                let msg = format!("[line {}] Error{}: {}", tkn.ln, " at end", msg);
                self.errors.push(LoxErr::new(msg));
            }
            _           => {
                let msg = format!("[line {}] Error at {}: {}", tkn.ln, tkn.lex, msg);
                self.errors.push(LoxErr::new(msg));
            },
        }
        self.had_parse_err = true;
    }

    pub fn error_runtime(&mut self, err: &RuntimeError) {
        let msg = format!("{}\n[line {}]", err.msg, err.tkn.ln);
        self.errors.push(LoxErr::new(msg));
        self.had_runtime_err = true;
    }

    pub fn report_errors(&mut self) {
        for err in &self.errors {
            eprintln!("{}", err.msg)
        }
        self.errors.clear();
    }
}