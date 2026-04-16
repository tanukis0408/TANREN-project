mod token;
mod lexer;
mod ast;
mod parser;
mod bytecode;
mod compiler;
mod value;
mod vm;

use std::env;
use std::fs;
use std::io::{self, Write};

use lexer::Lexer;
use parser::Parser;
use compiler::Compiler;
use vm::VM;

fn run(source: &str) {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize();

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    let mut compiler = Compiler::new();
    compiler.compile(&ast);

    let mut vm = VM::new(compiler.chunk);
    vm.run();
}

fn repl() {
    println!("Metal 0.2.0");
    println!("Type 'quit' to exit.\n");

    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "quit" || input == "exit" {
            println!("Goodbye!");
            break;
        }

        if input.is_empty() {
            continue;
        }

        let result = std::panic::catch_unwind(|| {
            run(input);
        });

        if result.is_err() {
            println!("Error: invalid expression");
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => repl(),
        2 => {
            let filename = &args[1];
            match fs::read_to_string(filename) {
                Ok(source) => run(&source),
                Err(e) => {
                    eprintln!("Cannot read '{}': {}", filename, e);
                    std::process::exit(1);
                }
            }
        }
        3 if args[1] == "run" => {
            let filename = &args[2];
            match fs::read_to_string(filename) {
                Ok(source) => run(&source),
                Err(e) => {
                    eprintln!("Cannot read '{}': {}", filename, e);
                    std::process::exit(1);
                }
            }
        }
        _ => {
            eprintln!("Usage:");
            eprintln!("  metal               -- REPL");
            eprintln!("  metal <file.mt>     -- run file");
            eprintln!("  metal run <file.mt> -- run file");
        }
    }
}