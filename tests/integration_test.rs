use for_transducer_interpreter::{tokenize, Parser, Interpreter};
use std::fs;

#[test]
fn test_example1() {
    let script = fs::read_to_string("examples/example1.txt").expect("Unable to read example1.txt");

    // Tokenize the script
    let tokens = tokenize(&script);

    // Parse the tokens into an AST
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse();

    // Interpret the AST
    let mut interpreter = Interpreter::new();
    interpreter.interpret(stmts);
}
