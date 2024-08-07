use simplified_transducer::ast::Bexpr;
use simplified_transducer::bexpr_evaluator::*;
use simplified_transducer::interpreter::Interpreter;
use simplified_transducer::label::*;
use simplified_transducer::order::*;
use simplified_transducer::qf_interpretation;
use simplified_transducer::qf_pullback::{pullback, FoFormula, FoFormulaR};
use simplified_transducer::two_sorted_formulas::{FormulaR, SMTResult, SMTSolver, Sort};
use simplified_transducer::{tokenize, Parser};

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

    // Remap variable indices and update formulas
    let mut remapped_universe_formulas = vec![];
    for (vars, universe_formula) in &universe_formulas {
        let (remapped_vars, remapped_formula) = remap_variables(vars, universe_formula);
        remapped_universe_formulas.push((remapped_vars, remapped_formula));
    }

    let mut remapped_label_formulas = vec![];
    for (i, (label_formula_a, label_formula_b, label_formula_hash)) in
        label_formulas.iter().enumerate()
    {
        let (vars, _) = &universe_formulas[i];
        let (remapped_a, remapped_b, remapped_hash) = (
            remap_formula_string(label_formula_a, vars),
            remap_formula_string(label_formula_b, vars),
            remap_formula_string(label_formula_hash, vars),
        );
        remapped_label_formulas.push((remapped_a, remapped_b, remapped_hash));
    }

    // Print the labels, corresponding universe formulas, and label formulas
    for (i, _label) in labels.iter().enumerate() {
        let (vars, universe_formula) = &remapped_universe_formulas[i];
        let _vars_str = if vars.is_empty() {
            "".to_string()
        } else {
            vars.join(", ")
        };
        let _formula_str = if universe_formula.to_string() == "T" {
            "T".to_string()
        } else {
            universe_formula.to_string()
        };

        let (_label_formula_a, _label_formula_b, _label_formula_hash) = &remapped_label_formulas[i];
        // println!(
        //     "Label: {:?}, Universe Formula({}): {}",
        //     label, vars_str, formula_str
        // );
        // println!("    Label Formula(a)({}): {}", vars_str, label_formula_a);
        // println!("    Label Formula(b)({}): {}", vars_str, label_formula_b);
        // println!("    Label Formula(#)({}): {}", vars_str, label_formula_hash);
    }

    let for_vars: Vec<Vec<i32>> = remapped_universe_formulas
        .iter()
        .map(|(vars, _)| {
            vars.iter()
                .map(|var| var[1..].parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    // Calculate the order formulas
    let mut order_formulas = Vec::new();
    generate_order_formula(
        &mut remapped_universe_formulas,
        &for0_or_for1,
        &mut order_formulas,
    );

    // println!("\nOrder Formulas:");
    // Print the order formulas
    for (i, j, _formula) in order_formulas.iter() {
        let mut vec = Vec::new();

        for a in 0..for_vars[*i].len() {
            vec.push(format!("x{}", for_vars[*i][a]));
        }

        for a in 0..for_vars[*j].len() {
            vec.push(format!("y{}", for_vars[*j][a]));
        }

        // Separate the variables by commas
        let _vars_str = vec.join(", ");

        let _label_i = &labels[*i];
        let _label_j = &labels[*j];
        // println!(
        //     "print{:?} <= print{:?} ({}): {}",
        //     label_i, label_j, vars_str, formula
        // );
    }

    // Fit the interpretation
    let qf = qf_interpretation::fit_interpretation(
        remapped_universe_formulas,
        order_formulas,
        for_vars.clone(),
        labels.clone(),
        remapped_label_formulas,
    );

    // Print the interpretation
    qf_interpretation::print_interpretation(&qf, &for_vars);

    // have a formula
    // that says, that the last letter of the output should be
    // an a.
    //
    // φ = exists x. (forall y. y <= x) and a(x)

    let last_letter_is_a: FoFormula = FoFormula {
        inside: FoFormulaR::Exists(
            "x".into(),
            Box::new(FoFormula {
                inside: FoFormulaR::And(
                    Box::new(FoFormula {
                        inside: FoFormulaR::Forall(
                            "y".into(),
                            Box::new(FoFormula {
                                inside: FoFormulaR::PosLessEqual("y".into(), "x".into()),
                            }),
                        ),
                    }),
                    Box::new(FoFormula {
                        inside: FoFormulaR::PosLetter("x".into(), "a".into()),
                    }),
                ),
            }),
        ),
    };

    let first_letter_is_a: FormulaR<String, String> = FormulaR::less_equal("x".into(), "y".into())
        .forall("y".into(), Sort::Position)
        .and(FormulaR::letter_at_pos("x".into(), "a".into()))
        .exists("x".into(), Sort::Position);

    let new_formula = pullback(&last_letter_is_a, &qf);

    println!("New formula: {:?}", new_formula);
    let solvers = vec![
        SMTSolver::Mona,
        SMTSolver::Z3,
        SMTSolver::CVC5,
        SMTSolver::AltErgo,
    ];
    let alphabet = vec!["a".into(), "b".into()];
    let labels: Vec<String> = qf
        .labels
        .iter()
        .enumerate()
        .map(|(i, _)| format!("l{i}"))
        .collect();

    for solver in &solvers {
        println!("Checking formula with solver: {:?}", solver);
        println!(
            "Model: {}\n\n",
            solver.produce_output(&new_formula, &alphabet, &labels)
        );
        let result: SMTResult = solver.solve(&new_formula, &alphabet, &labels);
        println!("Result: {:?}\n\n", result);
    }

    // simplified_transducer::two_sorted_formulas::example();

    // ask for an input string
    let mut input = String::new();
    println!("Enter a string to evaluate the formula: ");
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    //give iterator to the interpreter
    let qf_output = qf_interpretation::evaluate(&qf, input.to_string());
    println!("QF output: {}", qf_output);
    print!("TR output: ");
    let mut interpreter = Interpreter::new(input);
    interpreter.interpret(stmts);
    println!("");
    //let original_output: String = unimplemented!(); // TODO (for later) directly evaluate the transducer
    //println!("TR output: {}", original_output);
}

fn remap_variables(vars: &[String], formula: &Bexpr) -> (Vec<String>, Bexpr) {
    let mut index_map = std::collections::HashMap::new();
    let mut new_vars = vec![];
    for (new_index, var) in vars.iter().enumerate() {
        index_map.insert(var.clone(), format!("x{}", new_index + 1));
        new_vars.push(format!("x{}", new_index + 1));
    }
    let remapped_formula = remap_bexpr_with_map(formula, &index_map);
    (new_vars, remapped_formula)
}

fn remap_bexpr_with_map(expr: &Bexpr, map: &std::collections::HashMap<String, String>) -> Bexpr {
    match expr {
        Bexpr::Var(var) => Bexpr::Var(map.get(var).cloned().unwrap_or_else(|| var.clone())),
        Bexpr::Str(s) => Bexpr::Str(s.clone()),
        Bexpr::Less(lhs, rhs) => Bexpr::Less(
            Box::new(remap_bexpr_with_map(lhs, map)),
            Box::new(remap_bexpr_with_map(rhs, map)),
        ),
        Bexpr::LessEqual(lhs, rhs) => Bexpr::LessEqual(
            Box::new(remap_bexpr_with_map(lhs, map)),
            Box::new(remap_bexpr_with_map(rhs, map)),
        ),
        Bexpr::Equal(lhs, rhs) => Bexpr::Equal(
            Box::new(remap_bexpr_with_map(lhs, map)),
            Box::new(remap_bexpr_with_map(rhs, map)),
        ),
        Bexpr::NotEqual(lhs, rhs) => Bexpr::NotEqual(
            Box::new(remap_bexpr_with_map(lhs, map)),
            Box::new(remap_bexpr_with_map(rhs, map)),
        ),
        Bexpr::GreaterEqual(lhs, rhs) => Bexpr::GreaterEqual(
            Box::new(remap_bexpr_with_map(lhs, map)),
            Box::new(remap_bexpr_with_map(rhs, map)),
        ),
        Bexpr::Greater(lhs, rhs) => Bexpr::Greater(
            Box::new(remap_bexpr_with_map(lhs, map)),
            Box::new(remap_bexpr_with_map(rhs, map)),
        ),
        Bexpr::Not(expr) => Bexpr::Not(Box::new(remap_bexpr_with_map(expr, map))),
        Bexpr::And(lhs, rhs) => Bexpr::And(
            Box::new(remap_bexpr_with_map(lhs, map)),
            Box::new(remap_bexpr_with_map(rhs, map)),
        ),
        Bexpr::Or(lhs, rhs) => Bexpr::Or(
            Box::new(remap_bexpr_with_map(lhs, map)),
            Box::new(remap_bexpr_with_map(rhs, map)),
        ),
        Bexpr::Label(label) => {
            Bexpr::Label(map.get(label).cloned().unwrap_or_else(|| label.clone()))
        }
    }
}

fn remap_formula_string(formula: &str, vars: &[String]) -> String {
    let mut index_map = std::collections::HashMap::new();
    for (new_index, var) in vars.iter().enumerate() {
        index_map.insert(var.clone(), format!("x{}", new_index + 1));
    }
    //println!("{:?}",index_map);
    let mut remapped_formula = formula.to_string();
    for (old_var, new_var) in &index_map {
        remapped_formula = remapped_formula.replace(old_var, new_var);
    }
    //println!("{:?}",remapped_formula);
    remapped_formula
}
