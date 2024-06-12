use std::env;
use std::fs;
use simplified_transducer::{tokenize, Parser};
//use simplified_transducer::parser::print_ast; // Import the print_ast function
mod label;
use label::traverse_and_label;

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
    let _input_string = &args[2];

    // Tokenize the script
    let tokens = tokenize(&script);

    // Parse the tokens into an AST
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse();

    // Print the AST
    let mut path = Vec::new();
    let mut labels = Vec::new();

    // Traverse the AST and label print statements
    traverse_and_label(&stmts, &mut path, &mut labels);

    // Print the labels
    for label in labels {
        println!("{:?}", label);
    }
    // Interpret the AST
    // let mut interpreter = Interpreter::new(input_string);
    // interpreter.interpret(stmts);
}
