use simplified_transducer::ast::Bexpr;

pub fn generate_order_formula(
    universe_formulas: &[(Vec<String>, Bexpr)],
    for0_or_for1: &[usize],
    order_formulas: &mut Vec<(usize, usize, Bexpr)>,
) {
    // Convert vector of vector of strings to vector of vector of i32
    let for_vars: Vec<Vec<i32>> = universe_formulas.iter()
        .map(|(vars, _)| vars.iter().map(|var| var[1..].parse::<i32>().unwrap()).collect())
        .collect();

    // Logic for generating order formulas
    for i in 0..universe_formulas.len() {
        for j in i + 1..universe_formulas.len() {
            
            //store the largest common prefix array for each of the arrays for_vars[i] and for_vars[j]
            let mut lcp = Vec::new();
            let min_len = std::cmp::min(for_vars[i].len(), for_vars[j].len());
            for k in 0..min_len {
                if for_vars[i][k] == for_vars[j][k] {
                    lcp.push(for_vars[i][k]);
                } else {
                    break;
                }
            }

            // Generate the order formula by iterating backwards through the LCP array
            let mut order_formula = Bexpr::Var("T".to_string()); // Initial condition

            for k in (0..lcp.len()).rev() {
                let lhs = Bexpr::Var(format!("X{}", lcp[k])).clone(); // Cloning to avoid move
                let rhs = Bexpr::Var(format!("x{}", lcp[k])).clone(); // Cloning to avoid move

                let less = Bexpr::Less(Box::new(lhs.clone()), Box::new(rhs.clone()));
                let greater = Bexpr::Greater(Box::new(lhs.clone()), Box::new(rhs.clone()));
                let greater_equal = Bexpr::GreaterEqual(Box::new(lhs.clone()), Box::new(rhs.clone()));
                let less_equal = Bexpr::LessEqual(Box::new(lhs.clone()), Box::new(rhs.clone()));
                let equal = Bexpr::Equal(Box::new(lhs), Box::new(rhs));

                let combined_condition = if k == lcp.len() - 1 {
                    // Last element of LCP: combine with just the less condition
                    if for0_or_for1[k] == 0 {
                        less_equal
                    }

                    else {
                        greater_equal
                    }
                    
                } else {
                    // Not the last element: combine with the OR of less and equal conditions
                    if for0_or_for1[k] == 0 {
                        Bexpr::Or(Box::new(less), Box::new(Bexpr::And(Box::new(equal), Box::new(order_formula.clone()))))
                    }

                    else {
                        Bexpr::Or(Box::new(greater), Box::new(Bexpr::And(Box::new(equal), Box::new(order_formula.clone()))))
                    }
                };

                order_formula = combined_condition;
            }

            // Now order_formula contains the complete nested order conditions


            order_formulas.push((i, j, order_formula));
            

        }
    }
}


