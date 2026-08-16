use std::process::exit;
use std::{fs::File, io::Read};
use std::io::{Write, stdin, stdout};

use anyhow::Result;

use crate::scanner::Scanner;

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
        self.run(code)?;
    
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
    
            self.run(buf.clone().trim_end().to_string())?;
            self.had_err = false;
        }
    
        Ok(())
    }
    
    fn run(&mut self, code: String) -> Result<()> {
        if self.had_err { 
            exit(65); 
        }
        
        let mut scr = Scanner::new(code);
        let tkns = scr.scan_tokens(self);
    
        for tkn in tkns {
            println!("{:?}", tkn);
        }

        Ok(())
    }

    pub fn error(&mut self, ln: usize, msg: &str) {
        self.report(ln, "", msg);
    }

    fn report(&mut self, ln: usize, loc: &str, msg: &str) {
        eprintln!("[line {}] Error{}: {}", ln, loc, msg);
        self.had_err = true;
    }
}