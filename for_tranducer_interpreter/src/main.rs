mod lexer;
mod parser;
mod ast;
mod interpreter;

use std::env;
use std::fs;

fn main() {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <script>", args[0]);
        return;
    }

    // Read the script file
    let script = fs::read_to_string(&args[1]).expect("Unable to read script file");

    // Tokenize the script
    let tokens = lexer::tokenize(&script);

    // Parse the tokens into an AST
    let mut parser = parser::Parser::new(tokens);
    let stmts = parser.parse();

    // Interpret the AST
    let mut interpreter = interpreter::Interpreter::new();
    interpreter.interpret(stmts);
}
