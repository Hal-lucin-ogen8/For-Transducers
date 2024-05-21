mod lexer;
mod parser;
mod ast;
mod interpreter;

use lexer::tokenize;
use parser::Parser;
use interpreter::Interpreter;

fn main() {
    let input = r#"
    for i in 0..3 {
        print(i)
    }
    "#;

    let tokens = tokenize(input);
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse();

    let mut interpreter = Interpreter::new();
    interpreter.interpret(stmts);
}

