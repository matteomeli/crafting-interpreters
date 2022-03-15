use std::{
    fs::File,
    io::{self, BufRead, Read, Write},
    process::exit,
};

use rlox::{scanner::Scanner, Result};

fn main() {
    let mut args = std::env::args();
    if args.len() > 2 {
        println!("Usage: rlox [script]");
        exit(64);
    } else if args.len() == 2 {
        run_file(args.nth(1).unwrap());
    } else {
        run_prompt();
    }
}

fn run_file(path: String) {
    let mut file = File::open(path).unwrap();
    let mut source = String::new();
    file.read_to_string(&mut source).unwrap();
    if let Err(e) = run(source) {
        eprintln!("{}", e);
    }
}

fn run_prompt() {
    print!("> ");
    io::stdout().flush().unwrap();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Ok(line) => {
                if let Err(e) = run(line) {
                    eprintln!("{}", e);
                }
            }
            Err(_) => break,
        }
        print!("> ");
        io::stdout().flush().unwrap();
    }
}

fn run(source: String) -> Result<()> {
    let mut scanner = Scanner::new(source);
    scanner.scan_tokens()?;

    // For now just print the tokens
    for token in scanner.tokens() {
        println!("{:?}", token);
    }

    Ok(())
}
