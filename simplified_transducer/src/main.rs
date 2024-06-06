use std::env;
use std::fs;
use simplified_transducer::{tokenize, Parser};
use simplified_transducer::parser::print_ast; // Import the print_ast function

fn main() {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} <script> <string>", args[0]);
        return;
    }

    // Read the script file
    let script = fs::read_to_string(&args[1]).expect("Unable to read script file");

    // Get the string
    let input_string = &args[2];

    // Tokenize the script
    let tokens = tokenize(&script);

    // Parse the tokens into an AST
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse();

    // Print the AST
    print_ast(&stmts, 0);

    // Interpret the AST
    // let mut interpreter = Interpreter::new(input_string);
    // interpreter.interpret(stmts);
}
