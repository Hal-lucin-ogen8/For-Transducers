use simplified_transducer::{tokenize, Parser};
use simplified_transducer::ast::Bexpr;
use std::env;
use std::fs;
mod label;
mod order;
mod bexpr_evaluator;
mod qf_interpretation;
use label::traverse_and_label;
use order::generate_order_formula;

fn main() {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
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


    let for_vars: Vec<Vec<i32>> = universe_formulas.iter()
        .map(|(vars, _)| vars.iter().map(|var| var[1..].parse::<i32>().unwrap()).collect())
        .collect();


    //calculate the order formulas
    let mut order_formulas = Vec::new();
    generate_order_formula(&mut universe_formulas, &for0_or_for1, &mut order_formulas);

    println!("\nOrder Formulas:");
    // Print the order formulas
    for (i, j, formula) in order_formulas.iter() {
        let mut vec = Vec::new();

        for a in 0..for_vars[*i].len() {
            vec.push(format!("X{}", for_vars[*i][a]));
        }

        for a in 0..for_vars[*j].len() {
            vec.push(format!("x{}", for_vars[*j][a]));
        }

        //separate the variables by commas
        let vars_str = vec.join(", ");

        let label_i = &labels[*i];
        let label_j = &labels[*j];
        println!("print{:?} <= print{:?} ({}): {}", label_i, label_j, vars_str, formula);

    }

    // Fit the interpretation
    let qf = qf_interpretation::fit_interpretation(universe_formulas, order_formulas, for_vars.clone(), labels.clone(), label_formulas);
    
    // Print the interpretation
    qf_interpretation::print_interpretation(&qf, &for_vars);

    //give iterator to the interpreter
    qf_interpretation::evaluate(&qf, "abcd".to_string());

    

}
