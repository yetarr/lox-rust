use std::{env, process::exit};
use anyhow::Result;

use lox_rust::lox;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut lox = lox::Lox::new();
    if args.len() > 2 {
        eprintln!("Usage: jlox [script]");
        exit(64)
    } else if args.len() == 2 {
        let file = args[1].clone();
        lox.run_file(file)?;
    } else {
        lox.run_prompt()?;
    }

    Ok(())
}