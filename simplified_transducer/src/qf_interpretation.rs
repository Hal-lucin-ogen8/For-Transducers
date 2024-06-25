use std::vec::Vec;
///
/// In this file we define what is a quantifier
/// free interpretation of words.
///
use simplified_transducer::ast::Bexpr;
use simplified_transducer::bexpr_evaluator;

//define new struct QfInterpretation
impl QfInterpretation {
    // Associated function to create a new instance of QfInterpretation
    pub fn new() -> Self {
        Self {
            labels: Vec::new(),
            arities: Vec::new(),
            universe: Vec::new(),
            order: Vec::new(),
            letters: Vec::new(),
        }
    }
}

pub fn print_interpretation(qf: &QfInterpretation, for_vars: &Vec<Vec<i32>>) {
    //print the labels
    println!("Labels: {:?}", qf.labels);

    //print the arities
    println!("Arities: {:?}", qf.arities);

    // print the universe formulas
    println!("\nUniverse Formulas:");
    for (i, formula) in qf.universe.iter() {
        println!("{:?}, Formula: {}", i, formula);
    }

    // print the order formulas
    println!("\nOrder Formulas:");
    for (i, j, formula) in qf.order.iter() {
        let mut vec = Vec::new();

        for a in 0..for_vars[*i].len() {
            vec.push(format!("X{}", for_vars[*i][a]));
        }

        for a in 0..for_vars[*j].len() {
            vec.push(format!("x{}", for_vars[*j][a]));
        }

        println!("{i} <= {j}: {formula}")
    }
    
    //print the letter formulas
    for (i, str, expr) in qf.letters.iter() {
        println!("Letter: {:?}, Formula({}): {}", i, str, expr);
    }
}

#[derive(Debug, Clone)]
pub struct QfInterpretation {
    pub labels: Vec<String>,
    pub arities: Vec<usize>,
    pub universe: Vec<(usize, Bexpr)>,
    pub order: Vec<(usize, usize, Bexpr)>,
    pub letters: Vec<(usize, String, Bexpr)>,
}

fn create_default_interpretation() -> QfInterpretation {
    QfInterpretation {
        labels: vec!["l1".to_string(), "l2".to_string()],
        arities: vec![1, 2],
        universe: vec![
            (0, Bexpr::Str("true".into())),
            (
                1,
                Bexpr::LessEqual(
                    Box::new(Bexpr::Var("x1".into())),
                    Box::new(Bexpr::Var("x2".into())),
                ),
            ),
        ],
        letters: vec![
            (0, "a".to_string(), Bexpr::Str("true".into())),
            (0, "b".to_string(), Bexpr::Str("false".into())),
            (
                1,
                "a".to_string(),
                Bexpr::Equal(
                    Box::new(Bexpr::Var("x1".into())),
                    Box::new(Bexpr::Var("x2".into())),
                ),
            ),
            (
                1,
                "b".to_string(),
                Bexpr::NotEqual(
                    Box::new(Bexpr::Var("x1".into())),
                    Box::new(Bexpr::Var("x2".into())),
                ),
            ),
        ],
        order: vec![
            (
                0,
                0,
                Bexpr::LessEqual(
                    Box::new(Bexpr::Var("x1".into())),
                    Box::new(Bexpr::Var("y1".into())),
                ),
            ),
            (0, 1, Bexpr::Str("true".into())),
            (1, 0, Bexpr::Str("false".into())),
            (
                1,
                1,
                // lexicographic order "x1 < x2 or x1 = x2 and y1 >= y2"
                Bexpr::Or(
                    Box::new(Bexpr::LessEqual(
                        Box::new(Bexpr::Var("x1".into())),
                        Box::new(Bexpr::Var("x2".into())),
                    )),
                    Box::new(Bexpr::And(
                        Box::new(Bexpr::Equal(
                            Box::new(Bexpr::Var("x1".into())),
                            Box::new(Bexpr::Var("x2".into())),
                        )),
                        Box::new(Bexpr::LessEqual(
                            Box::new(Bexpr::Var("y2".into())),
                            Box::new(Bexpr::Var("y1".into())),
                        )),
                    )),
                ),
            ),
        ],
    }
}

pub fn fit_interpretation(universe_formulas: Vec<(Vec<String>, Bexpr)> , order_formulas: Vec<(usize, usize, Bexpr)>, for_vars: Vec<Vec<i32>>, labels: Vec<Vec<usize>>, label_formulas: Vec<(String, String, String)>)-> QfInterpretation {
    //define the new interpretation
    let mut qf = QfInterpretation::new();

    //define the arities
    qf.arities = for_vars.iter().map(|vars| vars.len()).collect();

    for label in labels.iter() {
        // Convert each usize to String and join them with ", "
        let joined: String = label.iter()
            .map(|num| num.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        
        // Format the final string
        let formatted_label = format!("[{}]", joined);
        
        // Push the formatted label
        qf.labels.push(formatted_label);
    }

    //define the universe formulas
    qf.universe = vec![];
    let mut i = 0;
    for (_vars, formula) in universe_formulas {
        //push bexpr of the universe formulas
        qf.universe.push((i, formula));
        i+=1;
    }

    //define the order formulas
    qf.order = vec![];
    for (i, j, formula) in order_formulas {
        qf.order.push((i, j, formula));
    }

    //define the letter formulas
    qf.letters = vec![];
    i = 0;

    for (label_formula_a, label_formula_b, label_formula_hash) in label_formulas {
        //push bexpr of the label formulas
        qf.letters.push((i, "a".to_string(), Bexpr::Str(label_formula_a)));
        qf.letters.push((i, "b".to_string(), Bexpr::Str(label_formula_b)));
        qf.letters.push((i, "#".to_string(), Bexpr::Str(label_formula_hash)));
        i+=1;
    }

    qf
    
}

/// TODO: implement this
pub fn evaluate(qf: &QfInterpretation, w: String) -> String {
    
    let n = w.len();

    //define a vector that will store tuples of strings
    let mut iterators = Vec::<(String, String)>::new();

    for i in 0..qf.labels.len() {
        let arity = qf.arities[i];
        let label = &qf.labels[i];

        // Generate tuples for current label and arity
        let mut tuple_strings = Vec::<(String, String)>::new();

        // Generate all combinations of positions for this arity
        let max_value = n.pow(arity as u32);
        for num in 0..max_value {
            let tuple = label.clone(); // Start with the label

            let mut positions = String::new();
            let mut temp = num;
            for j in 0..arity {
                if j > 0 {
                    positions.push('x'); // Add 'x' between the positions
                }

                let pos = temp % n;
                temp /= n;
                positions.push_str(&format!("{}", pos));
            }

            tuple_strings.push((tuple.clone(), positions.clone().chars().rev().collect::<String>()));
        }

        // Add generated tuples for current label and arity to iterators
        iterators.extend(tuple_strings);
    }

    // Print the iterators
    // println!("{:?}", iterators);

    "hello".to_string()
}



