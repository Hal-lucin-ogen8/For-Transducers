use crate::ast::{Bexpr, Pexpr, Stmt};
use crate::qf_interpretation::QfInterpretation;
use crate::two_sorted_formulas::{FormulaF, FormulaR};

pub fn bexpr_to_formula_s(bexpr: &Bexpr) -> FormulaS {
    // unimplemented!() // Replace this with the actual implementation
    FormulaS {
        inside: FormulaF::False,
    }
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
        },
        Bexpr::Var(var_name) if var_name.starts_with("y") => {
            let num_part = &var_name[1..]; // Extract numeric part after "y"
            Bexpr::Var(format!("{}{}", name_y, num_part))
        },
        Bexpr::Var(var_name) => Bexpr::Var(var_name.clone()), // for other variables
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
        Bexpr::Label(_) => formula.clone(), // assuming Label does not need substitution
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
pub fn universe_formula(qf: &QfInterpretation, var_name: &str) -> FormulaS {
    // 1. find the correct formula (qf.letter.find (...))
    // 2. substitute the variables in the formula with x -> var
    // 3. return the formula
    unimplemented!()
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
            
            let mut final_formula = bexpr_to_formula_s(&substituted_formula);
            
            return final_formula; 
            
        }
    }
    
    unimplemented!()

}

/// TODO: implement
pub fn letter_formula(qf: &QfInterpretation, l: usize, var: &str, letter: &str) -> FormulaS {
    // 1. find the correct formula (qf.letter.find (...))
    // 2. substitute the variables in the formula with x -> var
    // 3. return the formula
    
    //parse through all letter formulas in qf and check if the labels match
    unimplemented!();
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
    for i in number..0 {
        f = f.exists(
            format!("{}{}", var, i),
            crate::two_sorted_formulas::Sort::Position,
        );
    }
    f
}

pub fn quantify_forall(var: &str, number: usize, formula: FormulaS) -> FormulaS {
    let mut f = formula;
    for i in number..0 {
        f = f.forall(
            format!("{}{}", var, i),
            crate::two_sorted_formulas::Sort::Position,
        );
    }
    f
}

type Letter = String;
type Variable = String;

#[derive(Debug, Clone)]
enum FoFormulaR<T> {
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
struct FoFormula {
    inside: FoFormulaR<Box<FoFormula>>,
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
            unimplemented!()
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
            unimplemented!()
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
            unimplemented!()
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
            unimplemented!()
        }
    }
}
