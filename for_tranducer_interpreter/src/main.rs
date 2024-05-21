// Declare modules
mod lexer;
mod parser;
mod ast;
mod interpreter;

// Import necessary functions and structs
use lexer::tokenize;
use parser::Parser;
use interpreter::Interpreter;

fn main() {
    // The input program to interpret
    let input = r#"
    for i in 0..3 {
        print(i)
    }
    "#;

    // Tokenize the input
    let tokens = tokenize(input);
    // Create a new parser with the tokens
    let mut parser = Parser::new(tokens);
    // Parse the tokens into statements (AST)
    let stmts = parser.parse();

    // Create a new interpreter
    let mut interpreter = Interpreter::new();
    // Interpret the parsed statements
    interpreter.interpret(stmts);
}
