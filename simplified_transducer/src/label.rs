use simplified_transducer::ast::{Bexpr, Stmt};

pub fn traverse_and_label(
    stmts: &[Stmt],
    path: &mut Vec<usize>,
    labels: &mut Vec<Vec<usize>>,
    current_if: Option<Bexpr>,
    universe_formulas: &mut Vec<(Vec<String>, Bexpr)>,
    for_vars: &mut Vec<String>,
) {
    for (index, stmt) in stmts.iter().enumerate() {
        match stmt {
            Stmt::Print(_) => {
                let mut current_path = path.clone();
                current_path.push(index);
                labels.push(current_path.clone());

                if let Some(ref if_expr) = current_if {
                    universe_formulas.push((for_vars.clone(), if_expr.clone()));
                } else {
                    universe_formulas.push((for_vars.clone(), Bexpr::Var("T".to_string())));
                }
            }
            Stmt::For0(var, inner_stmts) | Stmt::For1(var, inner_stmts) => {
                path.push(index);
                for_vars.push(var.clone());
                traverse_and_label(inner_stmts, path, labels, current_if.clone(), universe_formulas, for_vars);
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
                traverse_and_label(inner_stmts, path, labels, Some(new_if_expr), universe_formulas, for_vars);
                path.pop();
            }
        }
    }
}
