use simplified_transducer::{tokenize, Parser};
use std::env;
use std::fs;
//use simplified_transducer::parser::print_ast; // Import the print_ast function
mod label;
mod order;
use label::traverse_and_label;
use order::generate_order_formula;

fn main() {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() <= 2 {
        println!("Usage: {} <script> <string>", args[0]);
        return;
    }

    // Read the script file
    let script = fs::read_to_string(&args[1]).expect("Unable to read script file");

    // Get the string
    //let _input_string = &args[2];

    // Tokenize the script
    let tokens = tokenize(&script);

    // Parse the tokens into an AST
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse();

    let mut path = Vec::new();
    let mut labels = Vec::new();
    let mut universe_formulas = Vec::new();
    let mut for_vars = Vec::new();
    let mut for0_or_for1 = Vec::new();
    let mut label_formulas = Vec::new();

    // Traverse the AST and label print statements and generate universe formulas and label formulas
    traverse_and_label(
        &stmts,
        &mut path,
        &mut labels,
        None,
        &mut universe_formulas,
        &mut for_vars,
        &mut for0_or_for1,
        &mut label_formulas,
    );

    // //print paths
    // for path in labels.iter() {
    //     println!("pathh{:?}", path);
    // }

    // //print for_vars
    // for for_var in for_vars.iter() {
    //     println!("forr{:?}", for_var);
    // }
    
    // //print labels
    // for label in labels.iter() {
    //     println!("labell{:?}", label);
    // }

    //print for0_or_for1
    // for for0_or_for1 in for0_or_for1.iter() {
    //     println!("for0_or_for1{:?}", for0_or_for1);
    // }

    // Print the labels, corresponding universe formulas, and label formulas
    for (i, label) in labels.iter().enumerate() {
        let (vars, universe_formula) = &universe_formulas[i];
        let vars_str = if vars.is_empty() {
            "".to_string()
        } else {
            vars.join(", ")
        };
        let formula_str = if universe_formula.to_string() == "T" {
            "T".to_string()
        } else {
            universe_formula.to_string()
        };

        let (label_formula_a, label_formula_b, label_formula_hash) = &label_formulas[i];
        println!(
            "Label: {:?}, Universe Formula({}): {}",
            label, vars_str, formula_str
        );
        println!("    Label Formula(a)({}): {}", vars_str, label_formula_a);
        println!("    Label Formula(b)({}): {}", vars_str, label_formula_b);
        println!("    Label Formula(#)({}): {}", vars_str, label_formula_hash);
    }

    //run order.rs
    let mut order_formulas = Vec::new();
    generate_order_formula(&stmts, &mut path, &mut labels, &mut universe_formulas, &mut order_formulas);
    //print_ast(&stmts,0);
    // Interpret the AST
    // let mut interpreter = Interpreter::new(input_string);
    // interpreter.interpret(stmts);
}
