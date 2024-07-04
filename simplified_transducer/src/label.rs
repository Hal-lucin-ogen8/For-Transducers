use crate::ast::{Bexpr, Pexpr, Stmt};

pub fn traverse_and_label(
    stmts: &[Stmt],
    path: &mut Vec<usize>,
    labels: &mut Vec<Vec<usize>>,
    current_if: Option<Bexpr>,
    universe_formulas: &mut Vec<(Vec<String>, Bexpr)>,
    for_vars: &mut Vec<String>,
    for0_or_for1: &mut Vec<usize>,
    label_formulas: &mut Vec<(String, String, String)>,
) {
    for (index, stmt) in stmts.iter().enumerate() {
        match stmt {
            Stmt::Print(expr) => {
                let mut current_path = path.clone();
                current_path.push(index);
                labels.push(current_path.clone());

                let universe_formula = if let Some(ref if_expr) = current_if {
                    if_expr.clone()
                } else {
                    Bexpr::Var("T".to_string())
                };
                universe_formulas.push((for_vars.clone(), universe_formula));

                let label_formula_a = generate_label_formula(expr, 'a');
                let label_formula_b = generate_label_formula(expr, 'b');
                let label_formula_hash = generate_label_formula(expr, '#');
                label_formulas.push((label_formula_a, label_formula_b, label_formula_hash));
            }
            Stmt::For0(var, inner_stmts) => {
                for0_or_for1.push(0);
                path.push(index);
                for_vars.push(var.clone());
                traverse_and_label(
                    inner_stmts,
                    path,
                    labels,
                    current_if.clone(),
                    universe_formulas,
                    for_vars,
                    for0_or_for1,
                    label_formulas,
                );
                for_vars.pop();
                path.pop();
            }

            Stmt::For1(var, inner_stmts) => {
                for0_or_for1.push(1);
                path.push(index);
                for_vars.push(var.clone());
                traverse_and_label(
                    inner_stmts,
                    path,
                    labels,
                    current_if.clone(),
                    universe_formulas,
                    for_vars,
                    for0_or_for1,
                    label_formulas,
                );
                for_vars.pop();
                path.pop();
            }

            Stmt::If(if_expr, inner_stmts) => {
                path.push(index);
                let new_if_expr = if let Some(ref existing_if) = current_if {
                    Bexpr::And(Box::new(existing_if.clone()), Box::new(if_expr.clone()))
                } else {
                    if_expr.clone()
                };
                traverse_and_label(
                    inner_stmts,
                    path,
                    labels,
                    Some(new_if_expr),
                    universe_formulas,
                    for_vars,
                    for0_or_for1,
                    label_formulas,
                );
                path.pop();
            }
        }
    }
}

fn generate_label_formula(expr: &Pexpr, ch: char) -> String {
    match expr {
        Pexpr::Label(label) => format!("{}({})", ch, label),
        Pexpr::Str(s) => {
            if s.contains(ch) {
                "T".to_string()
            } else {
                "F".to_string()
            }
        }
    }
}
