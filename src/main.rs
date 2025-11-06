use std::env;
use std::fs;
use std::io::{self, Read};

mod lexer;
mod parser;
mod value;

fn main() {
    // Step 1: Get command line argument
    let args: Vec<String> = env::args().collect();

    let file_path = match args.get(1) {
        Some(path) => path,
        None => {
            eprintln!("Usage: {} <file>", args[0]);
            std::process::exit(1);
        }
    };

    // Step 2: Read file contents
    let contents = match fs::read_to_string(file_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to read file '{}': {}", file_path, e);
            std::process::exit(1);
        }
    };

    // Step 3: Lexing
    let tokens = match lexer::lex(&contents) {
        Ok(token) => token,
        Err(e) => {
            eprintln!("Lexer error at line {}, column {}. error - {}",e.line,e.column, e.message);
            std::process::exit(1);
        }
    };

    // Step 4: Parsing
    match parser::parse(tokens) {
        Ok(value) => {
            println!("Valid JSON: {:?}", value);
            std::process::exit(0);
        }
        Err(e) => {
            eprintln!("Invalid JSON: {}", e);
            std::process::exit(1);
        }
    }
}
