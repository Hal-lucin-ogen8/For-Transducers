use crate::bexpr_evaluator;
///
/// In this file we define what is a quantifier
/// free interpretation of words.
///
use crate::Bexpr;
use std::collections::HashMap;

use itertools::Itertools;

/// A type alias to represent an input position
pub type InputPosition = usize;
/// A type alias to represent a label
pub type Label = usize;
/// A type alias to represent an arity
pub type Arity = usize;
/// Type alias to represent output letters
pub type Letter = String;

/// Type alias to represent a position in the output word
#[derive(Debug, Clone)]
pub struct OutputPosition {
    label: Label,
    vars: Vec<InputPosition>,
}

/// A quantifier free interpretation of words
#[derive(Debug, Clone)]
pub struct QfInterpretation {
    /// Display information for the labels
    pub labels: Vec<String>,
    /// arities of the labels (number of free variables)
    pub arities: Vec<usize>,
    /// you have the guarantee that the formula
    /// for label L has free variables ranging in
    /// x1, x2, ..., x_arity(L)
    pub universe: Vec<(Label, Bexpr)>,
    /// you have the guarantee that the formula
    /// for a pair of labels L,L' has free variables ranging in
    /// x1, x2, ..., x_arity(L)
    /// y1, y2, ..., y_arity(L')
    pub order: Vec<(Label, Label, Bexpr)>,
    /// you have the guarantee that the formula
    /// for label L has free variables ranging in
    /// x1, x2, ..., x_arity(L)
    pub letters: Vec<(Label, Letter, Bexpr)>,
}

/// Evaluate a formula with a given word and variables
/// You should provide a vector of
/// variable name (e.g. x) + position in the output word (e.g. ("print1", [0,1,3]))
/// and the formula will be evaluated with variables
/// x1, x2, x3, ... replaced by the corresponding positions (0,1,3)
fn evaluate_formula(
    formula: &Bexpr,
    word: String,
    variables: &[(String, &OutputPosition)],
) -> bool {
    let variables_environment: HashMap<String, InputPosition> = variables
        .iter()
        .map(|(variable_name, pos)| {
            pos.vars
                .iter()
                .enumerate()
                .map(|(i, v)| (format!("{variable_name}{}", i+1), *v))
                .collect::<Vec<(String, InputPosition)>>()
        })
        .flatten()
        .collect();

    let mut evaluator = bexpr_evaluator::Evaluator {
        word: word.clone(),
        variables: variables_environment,
    };

    evaluator.eval(formula) // unimplemented!
}

#[derive(Debug)]
pub enum QfInterpretationError {
    MissingUniverseFormula {
        label: Label,
    },
    MissingOrderFormula {
        label_a: Label,
        label_b: Label,
    },
    NoLetter {
        word: String,
        position: OutputPosition,
    },
    TooManyLetters {
        word: String,
        position: OutputPosition,
        values: Vec<Letter>,
    },
}

impl QfInterpretation {
    pub fn new() -> Self {
        Self {
            labels: Vec::new(),
            arities: Vec::new(),
            universe: Vec::new(),
            order: Vec::new(),
            letters: Vec::new(),
        }
    }

    pub fn get_letter(
        &self,
        word: String,
        position: &OutputPosition,
    ) -> Result<Letter, QfInterpretationError> {
        let possible_letters = self
            .letters
            .iter()
            .filter_map(|(l, letter, phi)| {
                if *l == position.label {
                    let variables = vec![("x".to_string(), position)];
                    if evaluate_formula(phi, word.clone(), &variables) {
                        Some(letter.clone())
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<Letter>>();
    
        match possible_letters.len() {
            0 => Err(QfInterpretationError::NoLetter {
                word,
                position: position.clone(),
            }),
            1 => Ok(possible_letters[0].clone()),
            _ => Err(QfInterpretationError::TooManyLetters {
                word,
                position: position.clone(),
                values: possible_letters,
            }),
        }
    }

    pub fn get_order(
        &self,
        word: String,
        position_a: &OutputPosition,
        position_b: &OutputPosition,
    ) -> Result<bool, QfInterpretationError> {
        let formula = self
            .order
            .iter()
            .find(|(i, j, _)| *i == position_a.label && *j == position_b.label)
            .map(|(_, _, phi)| phi)
            .ok_or(QfInterpretationError::MissingOrderFormula {
                label_a: position_a.label,
                label_b: position_b.label,
            })?;

        let variables = vec![("x".to_string(), position_a), ("y".to_string(), position_b)];
        Ok(evaluate_formula(formula, word.clone(), &variables))
    }

    pub fn get_universe(
        &self,
        word: String,
        position: &OutputPosition,
    ) -> Result<bool, QfInterpretationError> {
        let formula = self
            .universe
            .iter()
            .find(|(i, _)| *i == position.label)
            .map(|(_, phi)| phi)
            .ok_or(QfInterpretationError::MissingUniverseFormula {
                label: position.label,
            })?;

        let variables = vec![("x".to_string(), position)];

        Ok(evaluate_formula(formula, word.clone(), &variables))
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

fn create_example_interpretation() -> QfInterpretation {
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

/// Should produce a "valid" QfInterpretation
/// meaning that
///
/// 1. size of arities = size of labels
/// 2. for every label, the number of free variables in the formula
///   is equal to the arity of the label
pub fn fit_interpretation(
    universe_formulas: Vec<(Vec<String>, Bexpr)>,
    order_formulas: Vec<(usize, usize, Bexpr)>,
    for_vars: Vec<Vec<i32>>,
    labels: Vec<Vec<usize>>,
    label_formulas: Vec<(String, String, String)>,
) -> QfInterpretation {
    //define the new interpretation
    let mut qf = QfInterpretation::new();

    //define the arities
    qf.arities = for_vars.iter().map(|vars| vars.len()).collect();

    for label in labels.iter() {
        // Convert each usize to String and join them with ", "
        let joined: String = label
            .iter()
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
        i += 1;
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
        qf.letters
            .push((i, "a".to_string(), Bexpr::Str(label_formula_a)));
        qf.letters
            .push((i, "b".to_string(), Bexpr::Str(label_formula_b)));
        qf.letters
            .push((i, "#".to_string(), Bexpr::Str(label_formula_hash)));
        i += 1;
    }

    qf
}

/// input: qf : QfInterpretation, w : String
///
/// 1. produce all the tuples of positions in the string
///     based on the arities of the labels in the qf interpretation
///
/// 2. for each tuple of position, evaluate the corresponding domain
/// formula, and if it is false, remove this tuple from the list.
///
/// 3.  Sort the list of tuples of positions according to the formula <=
///
/// 4. For each tuple of positions, evaluate the corresponding letter
/// formula and replace the position by the result of the evaluation.
///
pub fn evaluate(qf: &QfInterpretation, w: String) -> String {
    // the size of the universe
    let word_size = w.len();

    // Generate the universe of all possible positions
    // the universe (all tuples of positions for all labels)
    // let universe: Vec<OutputPosition> = qf
    //     .arities
    //     .iter()
    //     .enumerate()
    //     .map(|(label, arity)| {
    //         (0..word_size)
    //             .combinations(*arity)
    //             .map(|vars| OutputPosition { label, vars })
    //             .collect::<Vec<OutputPosition>>()
    //     })
    //     .flatten()
    //     .collect();

    // Generate the universe of all possible positions
    let universe: Vec<OutputPosition> = qf
        .arities
        .iter()
        .enumerate()
        .flat_map(|(label, &arity)| {
            std::iter::repeat(0..word_size)
                .take(arity)
                .multi_cartesian_product()
                .map(move |vars| OutputPosition { label, vars })
        })
        .collect();

    eprintln!("Universe: {:?}", universe);

    // filter the universe based on the universe formulas
    let universe: Vec<OutputPosition> = universe
        .into_iter()
        .filter(|pos| qf.get_universe(w.clone(), &pos).unwrap())
        .collect();
    eprintln!("Universe [filtered]: {:?}", universe);

    // sort the universe based on the order formulas
    // update so that we detect equal positions
    // FIXME: if equal then say equal
    let universe: Vec<OutputPosition> = universe
        .into_iter()
        .sorted_by(|a, b| match qf.get_order(w.clone(), &a, &b).unwrap() {
            true => std::cmp::Ordering::Less,
            false => std::cmp::Ordering::Greater,
        })
        .collect();
    eprintln!("Universe [sorted]: {:?}", universe);

    // evaluate the letter formulas and replace the positions
    // by the result of the evaluation
    let result: String = universe
        .iter()
        .map(|pos| qf.get_letter(w.clone(), pos).unwrap())
        .collect();
    eprintln!("Universe [replaced]: {:?}", universe);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate() {
        let qf = create_example_interpretation();
        let result = evaluate(&qf, "abab".to_string());
        eprintln!("Result: {}", result);
        assert_eq!(result, "ab");
        let result = evaluate(&qf, "acab".to_string());
        assert_eq!(result, "ab");
        let result = evaluate(&qf, "aaaa".to_string());
        assert_eq!(result, "ab");
        let result = evaluate(&qf, "".to_string());
        assert_eq!(result, "");
        let result = evaluate(&qf, "a".to_string());
        assert_eq!(result, "a");
    }
}
