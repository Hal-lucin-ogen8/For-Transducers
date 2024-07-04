use crate::ast::Bexpr;
use crate::qf_interpretation::QfInterpretation;
use crate::two_sorted_formulas::{FormulaF, FormulaR};

pub fn bexpr_to_formula_s(bexpr: &Bexpr) -> FormulaS {
    match bexpr {
        Bexpr::Var(var_name) => {
            if var_name == "T" {
                FormulaS::const_true()
            } else if var_name == "F" {
                FormulaS::const_false()
            } else {
                // Handle variable expressions; assume Sort::Position for variables.
                FormulaS {
                    inside: FormulaF::Equal(
                        crate::two_sorted_formulas::Sort::Position,
                        var_name.clone(),
                        var_name.clone(),
                    ),
                }
            }
        }

        Bexpr::Str(s) => {
            // Handle string expressions; "T" for true and "F" for false.
            match s.as_str() {
                "T" => FormulaS {
                    inside: FormulaF::True,
                },
                "F" => FormulaS {
                    inside: FormulaF::False,
                },
                _ => {
                    // Handle letter at position cases, assuming format is letter(var_name).
                    if let Some((letter, var_name)) = parse_letter_at_pos(s) {
                        FormulaS {
                            inside: FormulaF::LetterAtPos(var_name.to_string(), letter.to_string()),
                        }
                    } else {
                        unimplemented!("Unexpected string value");
                    }
                }
            }
        }

        Bexpr::LessEqual(lhs, rhs) => {
            // Handle less than or equal to comparison.
            FormulaS {
                inside: FormulaF::LessEqual(extract_var_name(lhs), extract_var_name(rhs)),
            }
        }

        Bexpr::Less(lhs, rhs) => {
            // Handle less than comparison.
            // Convert to: not (rhs <= lhs)
            FormulaS {
                inside: FormulaF::Not(Box::new(FormulaS {
                    inside: FormulaF::LessEqual(extract_var_name(rhs), extract_var_name(lhs)),
                })),
            }
        }

        Bexpr::Equal(lhs, rhs) => {
            let label_str: Option<(String, String)> = try_extract_label_variable(lhs)
                .and_then(|label| try_extract_str(rhs).map(|letter| (label, letter)));

            let str_label: Option<(String, String)> = try_extract_str(lhs)
                .and_then(|letter| try_extract_label_variable(rhs).map(|label| (label, letter)));

            let var_var: Option<(String, String)> = try_extract_position_variable(lhs)
                .and_then(|var1| try_extract_position_variable(rhs).map(|var2| (var1, var2)));

            if let Some((label, letter)) = label_str {
                FormulaS::letter_at_pos(label, letter)
            } else if let Some((label, letter)) = str_label {
                FormulaS::letter_at_pos(label, letter)
            } else if let Some((var1, var2)) = var_var {
                FormulaS::equal(crate::two_sorted_formulas::Sort::Position, var1, var2)
            } else {
                unimplemented!("Unexpected value for lhs and rhs");
            }
        }

        Bexpr::NotEqual(lhs, rhs) => {
            let label_str: Option<(String, String)> = try_extract_label_variable(lhs)
                .and_then(|label| try_extract_str(rhs).map(|letter| (label, letter)));

            let str_label: Option<(String, String)> = try_extract_str(lhs)
                .and_then(|letter| try_extract_label_variable(rhs).map(|label| (label, letter)));

            let var_var: Option<(String, String)> = try_extract_position_variable(lhs)
                .and_then(|var1| try_extract_position_variable(rhs).map(|var2| (var1, var2)));

            if let Some((label, letter)) = label_str {
                FormulaS::letter_at_pos(label, letter).not()
            } else if let Some((label, letter)) = str_label {
                FormulaS::letter_at_pos(label, letter).not()
            } else if let Some((var1, var2)) = var_var {
                FormulaS::equal(crate::two_sorted_formulas::Sort::Position, var1, var2).not()
            } else {
                unimplemented!("Unexpected value for lhs and rhs");
            }
        }

        Bexpr::GreaterEqual(lhs, rhs) => {
            // Handle greater than or equal to comparison.
            // Convert to: rhs <= lhs
            FormulaS {
                inside: FormulaF::LessEqual(extract_var_name(rhs), extract_var_name(lhs)),
            }
        }

        Bexpr::Greater(lhs, rhs) => {
            // Handle greater than comparison.
            // Convert to: not (lhs <= rhs)
            FormulaS {
                inside: FormulaF::Not(Box::new(FormulaS {
                    inside: FormulaF::LessEqual(extract_var_name(lhs), extract_var_name(rhs)),
                })),
            }
        }

        Bexpr::Not(inner) => {
            // Handle negation.
            FormulaS {
                inside: FormulaF::Not(Box::new(bexpr_to_formula_s(inner))),
            }
        }

        Bexpr::Label(label) => {
            // Handle label expressions.
            unimplemented!("Labels shouldn't be directly part of Bexpr");
        }

        Bexpr::And(lhs, rhs) => {
            // Handle logical AND.
            FormulaS {
                inside: FormulaF::And(
                    Box::new(bexpr_to_formula_s(lhs)),
                    Box::new(bexpr_to_formula_s(rhs)),
                ),
            }
        }

        Bexpr::Or(lhs, rhs) => {
            // Handle logical OR.
            FormulaS {
                inside: FormulaF::Or(
                    Box::new(bexpr_to_formula_s(lhs)),
                    Box::new(bexpr_to_formula_s(rhs)),
                ),
            }
        }
    }
}

// Helper function to extract variable names from Bexpr.
fn extract_var_name(bexpr: &Bexpr) -> String {
    eprintln!("bexpr: {:?}", bexpr);
    match bexpr {
        Bexpr::Var(var_name) => var_name.clone(),
        Bexpr::Label(var_name) => var_name.clone(),
        _ => unimplemented!("Expected a variable"),
    }
}

fn try_extract_position_variable(bexpr: &Bexpr) -> Option<String> {
    match bexpr {
        Bexpr::Var(var_name) => Some(var_name.clone()),
        _ => None,
    }
}

fn try_extract_label_variable(bexpr: &Bexpr) -> Option<String> {
    match bexpr {
        Bexpr::Label(var_name) => Some(var_name.clone()),
        _ => None,
    }
}

fn try_extract_str(bexpr: &Bexpr) -> Option<String> {
    match bexpr {
        Bexpr::Str(s) => Some(s.clone()),
        _ => None,
    }
}

fn parse_letter_at_pos(s: &str) -> Option<(String, String)> {
    // This function expects the format to be letter(var_name), like a(x), b(y), #(z)
    if let Some(open_paren_index) = s.find('(') {
        if let Some(close_paren_index) = s.find(')') {
            if close_paren_index > open_paren_index {
                let letter = &s[..open_paren_index];
                let var_name = &s[open_paren_index + 1..close_paren_index];
                return Some((letter.to_string(), var_name.to_string()));
            }
        }
    }
    None
}

//
//
// INPUT
// formula: x1 /\ x2 \/ y3
// name_x : bla
// name_y : blub
// OUTPUT
// bla1 /\ bla2 \/ blub3
//
/// TODO: implement
// Function to substitute variables
pub fn substitute_variables(formula: &Bexpr, name_x: &str, name_y: &str) -> Bexpr {
    match formula {
        Bexpr::Var(var_name) if var_name.starts_with("x") => {
            let num_part = &var_name[1..]; // Extract numeric part after "x"
            Bexpr::Var(format!("{}{}", name_x, num_part))
        }
        Bexpr::Var(var_name) if var_name.starts_with("y") => {
            let num_part = &var_name[1..]; // Extract numeric part after "y"
            Bexpr::Var(format!("{}{}", name_y, num_part))
        }
        Bexpr::Var(var_name) => Bexpr::Var(var_name.clone()), // for other variables
        Bexpr::Label(var_name) if var_name.starts_with("x") => {
            let num_part = &var_name[1..]; // Extract numeric part after "x"
            Bexpr::Label(format!("{}{}", name_x, num_part))
        }
        Bexpr::Label(var_name) if var_name.starts_with("y") => {
            let num_part = &var_name[1..]; // Extract numeric part after "y"
            Bexpr::Label(format!("{}{}", name_y, num_part))
        }
        Bexpr::Label(var_name) => Bexpr::Label(var_name.clone()), // for other variables
        Bexpr::Str(_) => formula.clone(), // assuming Str does not need substitution
        Bexpr::LessEqual(left, right) => Bexpr::LessEqual(
            Box::new(substitute_variables(left, name_x, name_y)),
            Box::new(substitute_variables(right, name_x, name_y)),
        ),
        Bexpr::Less(left, right) => Bexpr::Less(
            Box::new(substitute_variables(left, name_x, name_y)),
            Box::new(substitute_variables(right, name_x, name_y)),
        ),
        Bexpr::Equal(left, right) => Bexpr::Equal(
            Box::new(substitute_variables(left, name_x, name_y)),
            Box::new(substitute_variables(right, name_x, name_y)),
        ),
        Bexpr::NotEqual(left, right) => Bexpr::NotEqual(
            Box::new(substitute_variables(left, name_x, name_y)),
            Box::new(substitute_variables(right, name_x, name_y)),
        ),
        Bexpr::GreaterEqual(left, right) => Bexpr::GreaterEqual(
            Box::new(substitute_variables(left, name_x, name_y)),
            Box::new(substitute_variables(right, name_x, name_y)),
        ),
        Bexpr::Greater(left, right) => Bexpr::Greater(
            Box::new(substitute_variables(left, name_x, name_y)),
            Box::new(substitute_variables(right, name_x, name_y)),
        ),
        Bexpr::Not(subexpr) => Bexpr::Not(Box::new(substitute_variables(subexpr, name_x, name_y))),
        Bexpr::And(left, right) => Bexpr::And(
            Box::new(substitute_variables(left, name_x, name_y)),
            Box::new(substitute_variables(right, name_x, name_y)),
        ),
        Bexpr::Or(left, right) => Bexpr::Or(
            Box::new(substitute_variables(left, name_x, name_y)),
            Box::new(substitute_variables(right, name_x, name_y)),
        ),
    }
}

/// TODO: implement
pub fn universe_formula(qf: &QfInterpretation, label: usize, var_name: &str) -> FormulaS {
    // 1. find the correct formula (qf.letter.find (...))
    // 2. substitute the variables in the formula with x -> var
    // 3. return the formula
    for (label_num, expr) in qf.universe.iter() {
        if *label_num == label {
            let mut substituted_formula = expr.clone();
            eprintln!("\n\n expr: {:?}", expr);
            substituted_formula = substitute_variables(&substituted_formula, var_name, "");
            eprintln!(
                "\n\n substituted_formula: x->{var_name} {:?}",
                substituted_formula
            );

            let final_formula = bexpr_to_formula_s(&substituted_formula);

            return final_formula;
        }
    }

    unimplemented!("No matching univ formula found");
}

/// TODO: implement
pub fn order_formula(
    qf: &QfInterpretation,
    lx: usize,
    ly: usize,
    var_x: &str,
    var_y: &str,
) -> FormulaS {
    // 1. find the correct formula (qf.letter.find (...)) based on the labels
    // 2. substitute the variables in the formula with x -> var_x, y -> var_y
    // 3. return the formula

    //parse through all order formulas in qf and check if the labels match
    for (label1, label2, formula) in qf.order.iter() {
        if *label1 == lx && *label2 == ly {
            //substitute the variables in the formula with x -> var_x, y -> var_y
            let mut substituted_formula = formula.clone();
            substituted_formula = substitute_variables(&substituted_formula, var_x, var_y);

            let final_formula = bexpr_to_formula_s(&substituted_formula);

            return final_formula;
        }
    }

    unimplemented!("No matching order formula found");
}

/// TODO: implement
// Function to find the letter formula and substitute variables
pub fn letter_formula(
    qf: &QfInterpretation,
    l: usize,
    var: &str,
    letter: &str,
) -> Option<FormulaS> {
    // Find the correct formula from the `letters` vector
    let letter_entry = qf
        .letters
        .iter()
        .find(|(label, letter_find, _)| *label == l && letter == letter_find);

    if let Some((_, _, formula)) = letter_entry {
        // Substitute variables in the formula
        let substituted_formula = substitute_variables(formula, var, "");

        // Return the modified formula
        Some(bexpr_to_formula_s(&substituted_formula))
    } else {
        None
    }
}

//
// Huge disjunction
//
// \/[ l1 in L] φ(l)
//
//
pub fn disjunction(vec: Vec<FormulaS>) -> FormulaS {
    // disjunction
    if vec.len() == 0 {
        return FormulaS {
            inside: FormulaF::False,
        };
    } else {
        let mut v_iter = vec.into_iter();
        let mut f = v_iter.next().unwrap();
        for i in v_iter {
            f = f.or(i);
        }
        f
    }
}

// TODO: check the range of the for loop ! [Number, ...., 1]
pub fn quantify_exists(var: &str, number: usize, formula: FormulaS) -> FormulaS {
    let mut f = formula;
    for i in (1..number + 1).rev() {
        f = f.exists(
            format!("{}{}", var, i),
            crate::two_sorted_formulas::Sort::Position,
        );
    }
    f = f.exists(format!("l{var}"), crate::two_sorted_formulas::Sort::Label);
    f
}

pub fn quantify_forall(var: &str, number: usize, formula: FormulaS) -> FormulaS {
    let mut f = formula;
    for i in (1..number + 1).rev() {
        f = f.forall(
            format!("{}{}", var, i),
            crate::two_sorted_formulas::Sort::Position,
        );
    }

    f = f.forall(format!("l{var}"), crate::two_sorted_formulas::Sort::Label);
    f
}

type Letter = String;
type Variable = String;

#[derive(Debug, Clone)]
pub enum FoFormulaR<T> {
    And(T, T),
    Or(T, T),
    Not(T),
    Iff(T, T),
    Implies(T, T),
    Exists(Variable, T),
    Forall(Variable, T),
    PosLessEqual(Variable, Variable),
    PosLetter(Variable, Letter),
}

#[derive(Debug, Clone)]
pub struct FoFormula {
    pub inside: FoFormulaR<Box<FoFormula>>,
}

pub fn map_fo_formula<F, S, T>(formula: &FoFormulaR<T>, f: &F) -> FoFormulaR<S>
where
    F: Fn(&T) -> S,
{
    match formula {
        FoFormulaR::And(left, right) => {
            let left = f(left);
            let right = f(right);
            FoFormulaR::And(left, right)
        }
        FoFormulaR::Or(left, right) => {
            let left = f(left);
            let right = f(right);
            FoFormulaR::Or(left, right)
        }
        FoFormulaR::Not(inner) => {
            let inner = f(inner);
            FoFormulaR::Not(inner)
        }
        FoFormulaR::Iff(left, right) => {
            let left = f(left);
            let right = f(right);
            FoFormulaR::Iff(left, right)
        }
        FoFormulaR::Implies(left, right) => {
            let left = f(left);
            let right = f(right);
            FoFormulaR::Implies(left, right)
        }
        FoFormulaR::Exists(var, inner) => {
            let inner = f(inner);
            FoFormulaR::Exists(var.clone(), inner)
        }
        FoFormulaR::Forall(var, inner) => {
            let inner = f(inner);
            FoFormulaR::Forall(var.clone(), inner)
        }
        FoFormulaR::PosLessEqual(var1, var2) => {
            FoFormulaR::PosLessEqual(var1.clone(), var2.clone())
        }
        FoFormulaR::PosLetter(var, letter) => FoFormulaR::PosLetter(var.clone(), letter.clone()),
    }
}

pub fn fold_fo_formula<F, T>(formula: &FoFormula, f: &F) -> T
where
    F: Fn(FoFormulaR<T>) -> T,
{
    let head_formula: &FoFormulaR<Box<FoFormula>> = &formula.inside;

    let induction: FoFormulaR<T> = map_fo_formula(head_formula, &|inner| fold_fo_formula(inner, f));
    f(induction)
}

type FormulaS = FormulaR<String, String>;

pub fn pullback(post_condition: &FoFormula, qf: &QfInterpretation) -> FormulaS {
    fold_fo_formula(post_condition, &|inner| pullback_unrec(inner, qf))
}

/// TODO implement
fn pullback_unrec(post_condition: FoFormulaR<FormulaS>, qf: &QfInterpretation) -> FormulaS {
    match post_condition {
        FoFormulaR::And(left, right) => FormulaR {
            inside: FormulaF::And(Box::new(left), Box::new(right)),
        },
        FoFormulaR::Or(left, right) => FormulaR {
            inside: FormulaF::Or(Box::new(left), Box::new(right)),
        },
        FoFormulaR::Not(inner) => FormulaR {
            inside: FormulaF::Not(Box::new(inner)),
        },
        FoFormulaR::Iff(left, right) => FormulaR {
            inside: FormulaF::Iff(Box::new(left), Box::new(right)),
        },
        FoFormulaR::Implies(left, right) => FormulaR {
            inside: FormulaF::Implies(Box::new(left), Box::new(right)),
        },
        FoFormulaR::Exists(var, inner) => {
            // TODO.
            // exists x. φ
            //
            // existsLabel lx.
            // existsVar x1.
            // existsVar x2.
            // ...
            // existsVar xn. (n = max arity)
            // universe_formula(x1, x2, ..., xn, lx, qf)
            // /\
            // φ
            let max_arity = qf.arities.iter().cloned().max().unwrap_or(0);

            let mut universe_formulas = Vec::new();
            for (i, expr) in qf.universe.iter() {
                let temp_formula_1 = universe_formula(qf, *i, &var);
                let temp_formula_2 = FormulaR {
                    inside: FormulaF::Equal(
                        crate::two_sorted_formulas::Sort::Label,
                        format!("l{}", var),
                        format!("l{}", i),
                    ),
                };

                universe_formulas.push(FormulaR {
                    inside: FormulaF::And(Box::new(temp_formula_1), Box::new(temp_formula_2)),
                });
            }

            eprintln!("universe_formulas: {:?}", universe_formulas);

            let disjunction_univs = disjunction(universe_formulas);

            // let conjunction = FormulaF::And(inner, disjunction_univs);
            let conjuction = disjunction_univs.and(inner);

            let final_form = quantify_exists(&var, max_arity, conjuction);

            return final_form;
        }
        FoFormulaR::Forall(var, inner) => {
            // TODO.
            // forall x. φ
            //
            // forall_label lx.
            // forall_position x1.
            // forall_position x2.
            // ...
            // forall_position xn. (n = max arity)
            // universe_formula(x1, x2, ..., xn, lx, qf)
            // ->
            // φ
            let max_arity = qf.arities.iter().cloned().max().unwrap_or(0);

            let mut universe_formulas = Vec::new();
            for (i, expr) in qf.universe.iter() {
                let temp_formula_1 = universe_formula(qf, *i, &var);
                eprintln!("temp_formula_1: {var} {:?}", temp_formula_1);
                let temp_formula_2 = FormulaR {
                    inside: FormulaF::Equal(
                        crate::two_sorted_formulas::Sort::Label,
                        format!("l{}", var),
                        format!("l{}", i),
                    ),
                };

                universe_formulas.push(FormulaR {
                    inside: FormulaF::And(Box::new(temp_formula_1), Box::new(temp_formula_2)),
                });
            }

            let disjunction_univs = disjunction(universe_formulas);

            let implication = disjunction_univs.implies(inner);

            let final_form = quantify_forall(&var, max_arity, implication);

            return final_form;
        }
        FoFormulaR::PosLessEqual(var1, var2) => {
            // TODO.
            //
            // z <= p
            //
            // lz (variable)
            // lp (variable)
            //
            // \/[l1, l2 labels]
            // ((l1 = lz /\ l2 = lp) /\ order_formula(z, p, l1, l2, qf))
            //
            let mut disjunctions = Vec::new();
            for (label1_number, label1_string) in qf.labels.iter().enumerate() {
                for (label2_number, label2_string) in qf.labels.iter().enumerate() {
                    let order_formula = order_formula(
                        qf,
                        label1_number,
                        label2_number,
                        var1.as_str(),
                        var2.as_str(),
                    );
                    let conjunction = FormulaR {
                        inside: FormulaF::And(
                            Box::new(FormulaR {
                                inside: FormulaF::Equal(
                                    crate::two_sorted_formulas::Sort::Label,
                                    format!("l{var1}"),
                                    format!("l{label1_number}"),
                                ),
                            }),
                            Box::new(FormulaR {
                                inside: FormulaF::Equal(
                                    crate::two_sorted_formulas::Sort::Label,
                                    format!("l{var2}"),
                                    format!("l{label2_number}"),
                                ),
                            }),
                        ),
                    };
                    let conjunction_with_order = FormulaR {
                        inside: FormulaF::And(Box::new(conjunction), Box::new(order_formula)),
                    };
                    disjunctions.push(conjunction_with_order);
                }
            }

            disjunction(disjunctions)
        }
        FoFormulaR::PosLetter(var, letter) => {
            // TODO.
            //
            // a(z)
            //
            //
            // lz          (variable for labels)
            // z1, ..., zn (variables for positions)
            //
            // a  (letter)
            //
            // L = { print1, print2, print3 }
            //
            // (lz = print1 /\ letter_formula(qf, print1, a, z))
            // \/
            // (lz = print2 /\ letter_formula(qf, print2, a, z))
            // \/
            // (lz = print3 /\ letter_formula(qf, print3, a, z))
            //
            let mut disjunctions = Vec::new();
            for (index, label) in qf.labels.iter().enumerate() {
                if let Some(letter_formula) =
                    letter_formula(qf, index, var.as_str(), letter.as_str())
                {
                    let conjunction = FormulaR {
                        inside: FormulaF::Equal(
                            crate::two_sorted_formulas::Sort::Label,
                            format!("l{var}"),
                            format!("l{index}"),
                        ),
                    };
                    let conjunction_with_letter = FormulaR {
                        inside: FormulaF::And(Box::new(conjunction), Box::new(letter_formula)),
                    };
                    disjunctions.push(conjunction_with_letter);
                }
            }

            disjunction(disjunctions)
        }
    }
}
