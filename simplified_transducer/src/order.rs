use simplified_transducer::ast::{Stmt, Pexpr, Bexpr};

pub fn print_vec_string(vec: &Vec<String>) {
    for i in 0..vec.len() {
        print!("{}", vec[i]);
        if i != vec.len() - 1 {
            print!(", ");
        }
        else {
            print!("\n");
        }
    }
}

pub fn generate_order_formula(
    stmts: &[Stmt],
    path: &mut Vec<usize>,
    labels: &mut Vec<Vec<usize>>,
    universe_formulas: &mut Vec<(Vec<String>, Bexpr)>,
    order_formulas: &mut Vec<(usize, usize, Bexpr)>,
) {
    //for each print statement pair accessible from universe_formula, print the vec<string> associated with it
    for i in 0..universe_formulas.len() {
        
        let mut vec_i = universe_formulas[i].0.clone();

        for j in i+1..universe_formulas.len() {
            
            let mut vec_j = universe_formulas[j].0.clone();
            
            println!("Comparing {i}: {:?} and {j}: {:?}", vec_i, vec_j);
            
        }
    }
}
