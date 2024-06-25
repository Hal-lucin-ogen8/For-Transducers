use crate::Bexpr;

#[derive(Debug, Clone)]
pub enum Value {
    Number(i32),
    Str(String),
}

pub struct Evaluator {
    variables: std::collections::HashMap<String, i32>,
    word: String,
}

impl Evaluator {
    fn evaluate_bexpr(&mut self, expr: &Bexpr) -> Value {
        match expr {
            Bexpr::Var(name) => {
                match self.variables.get(name) {
                    Some(value) => Value::Number(*value),
                    None => panic!("Variable {} not defined", name),
                }
            }
            Bexpr::Str(s) => Value::Str(s.clone()),
            Bexpr::Label(name) => {
                match self.variables.get(name) {
                    Some(value) => {
                        if let Some(character) = self.word.chars().nth(*value as usize) {
                            Value::Str(character.to_string())
                        } else {
                            panic!("Index out of bounds");
                        }
                    }
                    None => panic!("Variable {} not defined", name),
                }
            }
            Bexpr::LessEqual(left, right) |
            Bexpr::Less(left, right) |
            Bexpr::Equal(left, right) |
            Bexpr::NotEqual(left, right) |
            Bexpr::Greater(left, right) |
            Bexpr::GreaterEqual(left, right) => {
                let left_val = self.evaluate_bexpr(left);
                let right_val = self.evaluate_bexpr(right);

                match (left_val, right_val) {
                    (Value::Number(lv), Value::Number(rv)) => {
                        match expr {
                            Bexpr::LessEqual(_, _) => Value::Number((lv <= rv) as i32),
                            Bexpr::Less(_, _) => Value::Number((lv < rv) as i32),
                            Bexpr::Equal(_, _) => Value::Number((lv == rv) as i32),
                            Bexpr::NotEqual(_, _) => Value::Number((lv != rv) as i32),
                            Bexpr::Greater(_, _) => Value::Number((lv > rv) as i32),
                            Bexpr::GreaterEqual(_, _) => Value::Number((lv >= rv) as i32),
                            _ => panic!("Unexpected comparison"),
                        }
                    }
                    (Value::Str(ls), Value::Str(rs)) => {
                        match expr {
                            Bexpr::Equal(_, _) => Value::Number((ls == rs) as i32),
                            Bexpr::NotEqual(_, _) => Value::Number((ls != rs) as i32),
                            _ => panic!("Invalid comparison: only equality comparison with labels is allowed"),
                        }
                    }
                    _ => panic!("Invalid comparison types"),
                }
            }
            Bexpr::Not(inner) => {
                let inner_val = self.evaluate_bexpr(inner);
                match inner_val {
                    Value::Number(n) => Value::Number((n == 0) as i32),
                    _ => panic!("Invalid type for Not operation"),
                }
            }
            Bexpr::And(left, right) => {
                let left_val = self.evaluate_bexpr(left);
                let right_val = self.evaluate_bexpr(right);

                match (left_val, right_val) {
                    (Value::Number(lv), Value::Number(rv)) => Value::Number(((lv != 0) && (rv != 0)) as i32),
                    _ => panic!("Invalid types for And operation"),
                }
            }
            Bexpr::Or(left, right) => {
                let left_val = self.evaluate_bexpr(left);
                let right_val = self.evaluate_bexpr(right);

                match (left_val, right_val) {
                    (Value::Number(lv), Value::Number(rv)) => Value::Number(((lv != 0) || (rv != 0)) as i32),
                    _ => panic!("Invalid types for Or operation"),
                }
            }
        }
    }

    fn evaluate_condition(&mut self, expr: &Bexpr) -> bool {
        match self.evaluate_bexpr(expr) {
            Value::Number(n) => n != 0,
            Value::Str(_) => panic!("String in condition"),
        }
    }
}