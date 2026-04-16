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

fn check(source: &str) {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize();

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    let mut compiler = Compiler::new();
    compiler.compile(&ast);
}

fn try_execute<F: FnOnce()>(label: &str, action: F) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(action));
    if result.is_err() {
        eprintln!("Error while {}.", label);
        std::process::exit(1);
    }
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

        let result = std::panic::catch_unwind(|| run(input));
        if result.is_err() { println!("Error: invalid expression"); }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => repl(),
        2 => {
            let filename = &args[1];
            match fs::read_to_string(filename) {
                Ok(source) => try_execute(&format!("running '{}'", filename), || run(&source)),
                Err(e) => {
                    eprintln!("Cannot read '{}': {}", filename, e);
                    std::process::exit(1);
                }
            }
        }
        3 if args[1] == "run" => {
            let filename = &args[2];
            match fs::read_to_string(filename) {
                Ok(source) => try_execute(&format!("running '{}'", filename), || run(&source)),
                Err(e) => {
                    eprintln!("Cannot read '{}': {}", filename, e);
                    std::process::exit(1);
                }
            }
        }
        3 if args[1] == "check" => {
            let filename = &args[2];
            match fs::read_to_string(filename) {
                Ok(source) => {
                    try_execute(&format!("checking '{}'", filename), || check(&source));
                    println!("OK: {} passed lex/parse/compile checks", filename);
                }
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
            eprintln!("  metal check <file.mt> -- lex/parse/compile check without execution");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_script(source: &str) {
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse();
        let mut compiler = Compiler::new();
        compiler.compile(&ast);
        let mut vm = VM::new(compiler.chunk);
        vm.run();
    }

    #[test]
    fn break_works_inside_loop() {
        let src = r#"
let i = 0
loop
  i = i + 1
  if i == 3
    break
  end
end
"#;
        run_script(src);
    }

    #[test]
    fn next_works_inside_loop() {
        let src = r#"
let i = 0
let acc = 0
loop
  i = i + 1
  if i < 3
    next
  end
  acc = acc + i
  break
end
"#;
        run_script(src);
    }

    #[test]
    fn numeric_mixed_comparisons_work() {
        let src = r#"
say 1 < 2.5
say 3.0 >= 3
"#;
        run_script(src);
    }
}
