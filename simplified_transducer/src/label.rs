use simplified_transducer::ast::{Stmt};

pub fn traverse_and_label(stmts: &[Stmt], path: &mut Vec<usize>, labels: &mut Vec<Vec<usize>>) {
    for (index, stmt) in stmts.iter().enumerate() {
        match stmt {
            Stmt::Print(_) => {
                let mut current_path = path.clone();
                current_path.push(index);
                labels.push(current_path);
            }
            Stmt::For0(_, inner_stmts) | Stmt::For1(_, inner_stmts) | Stmt::If(_, inner_stmts) => {
                path.push(index);
                traverse_and_label(inner_stmts, path, labels);
                path.pop();
            }
        }
    }
}
