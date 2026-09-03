use anyhow::Result;
use std::{env, process::exit};

use lox_rust::lox;

const COMMAND_USAGE_ERROR: i32 = 64;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut lox = lox::Lox::new();
    if args.len() > 2 {
        eprintln!("Usage: jlox [script]");
        exit(COMMAND_USAGE_ERROR)
    } else if args.len() == 2 {
        let file = args[1].clone();
        lox.run_file(file)?;
    } else {
        lox.run_prompt()?;
    }

    Ok(())
}
