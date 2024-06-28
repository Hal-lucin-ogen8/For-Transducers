use crate::Bexpr;

#[derive(Debug, Clone)]
pub enum Value {
    Number(usize),
    Str(String),
}

pub struct Evaluator {
    pub variables: std::collections::HashMap<String, usize>,
    pub word: String,
}

impl Evaluator {
    // TODO: implement this function
    pub fn eval(&mut self, expr: &Bexpr) -> bool {
        self.evaluate_condition(expr)
    }

    fn evaluate_bexpr(&mut self, expr: &Bexpr) -> Value {

        //println!("{:?}", expr);
        match expr {
            Bexpr::Var(name) => {
                if name == "T" {
                    Value::Number(1)
                } else if name == "F" {
                    Value::Number(0)
                } else {
                    match self.variables.get(name) {
                        Some(value) => Value::Number(*value),
                        None => panic!("Variable {} not defined", name),
                    }
                }
            }
            Bexpr::Str(s) => {
                // Check for a(var_name), b(var_name), or #(var_name)

                if s == "T" {
                    Value::Number(1)
                } else if s == "F" {
                    Value::Number(0)
                }

                else if s.starts_with('a') && s.ends_with(')') && s.len() > 2 && s.chars().nth(1) == Some('(') {
                    let var_name = &s[2..s.len()-1];
                    match self.variables.get(var_name) {
                        Some(value) => {
                            if let Some(character) = self.word.chars().nth(*value) {
                                if character == 'a' {
                                    Value::Number(1)
                                } else {
                                    Value::Number(0)
                                }
                            } else {
                                panic!("Index out of bounds");
                            }
                        }
                        None => panic!("Variable {} not defined", var_name),
                    }
                } else if s.starts_with('b') && s.ends_with(')') && s.len() > 2 && s.chars().nth(1) == Some('(') {
                    let var_name = &s[2..s.len()-1];
                    match self.variables.get(var_name) {
                        Some(value) => {
                            if let Some(character) = self.word.chars().nth(*value) {
                                if character == 'b' {
                                    Value::Number(1)
                                } else {
                                    Value::Number(0)
                                }
                            } else {
                                panic!("Index out of bounds");
                            }
                        }
                        None => panic!("Variable {} not defined", var_name),
                    }
                } else if s.starts_with('#') && s.ends_with(')') && s.len() > 2 && s.chars().nth(1) == Some('(') {
                    let var_name = &s[2..s.len()-1];
                    match self.variables.get(var_name) {
                        Some(value) => {
                            if let Some(character) = self.word.chars().nth(*value) {
                                if character == '#' {
                                    Value::Number(1)
                                } else {
                                    Value::Number(0)
                                }
                            } else {
                                panic!("Index out of bounds");
                            }
                        }
                        None => panic!("Variable {} not defined", var_name),
                    }
                } else {
                    Value::Str(s.clone())
                }
            }

            Bexpr::Label(name) => match self.variables.get(name) {
                Some(value) => {
                    if let Some(character) = self.word.chars().nth(*value) {
                        Value::Str(character.to_string())
                    } else {
                        panic!("Index out of bounds");
                    }
                }
                None => panic!("Variable {} not defined", name),
            },
            Bexpr::LessEqual(left, right)
            | Bexpr::Less(left, right)
            | Bexpr::Equal(left, right)
            | Bexpr::NotEqual(left, right)
            | Bexpr::Greater(left, right)
            | Bexpr::GreaterEqual(left, right) => {
                let left_val = self.evaluate_bexpr(left);
                let right_val = self.evaluate_bexpr(right);
    
                match (left_val, right_val) {
                    (Value::Number(lv), Value::Number(rv)) => match expr {
                        Bexpr::LessEqual(_, _) => Value::Number((lv <= rv) as usize),
                        Bexpr::Less(_, _) => Value::Number((lv < rv) as usize),
                        Bexpr::Equal(_, _) => Value::Number((lv == rv) as usize),
                        Bexpr::NotEqual(_, _) => Value::Number((lv != rv) as usize),
                        Bexpr::Greater(_, _) => Value::Number((lv > rv) as usize),
                        Bexpr::GreaterEqual(_, _) => Value::Number((lv >= rv) as usize),
                        _ => panic!("Unexpected comparison"),
                    },
                    (Value::Str(ls), Value::Str(rs)) => match expr {
                        Bexpr::Equal(_, _) => Value::Number((ls == rs) as usize),
                        Bexpr::NotEqual(_, _) => Value::Number((ls != rs) as usize),
                        _ => panic!(
                            "Invalid comparison: only equality comparison with labels is allowed"
                        ),
                    },
                    _ => panic!("Invalid comparison types"),
                }
            }
            Bexpr::Not(inner) => {
                let inner_val = self.evaluate_bexpr(inner);
                match inner_val {
                    Value::Number(n) => Value::Number((n == 0) as usize),
                    _ => panic!("Invalid type for Not operation"),
                }
            }
            Bexpr::And(left, right) => {
                let left_val = self.evaluate_bexpr(left);
                let right_val = self.evaluate_bexpr(right);
    
                match (left_val, right_val) {
                    (Value::Number(lv), Value::Number(rv)) => {
                        Value::Number(((lv != 0) && (rv != 0)) as usize)
                    }
                    _ => panic!("Invalid types for And operation"),
                }
            }
            Bexpr::Or(left, right) => {
                let left_val = self.evaluate_bexpr(left);
                let right_val = self.evaluate_bexpr(right);
    
                match (left_val, right_val) {
                    (Value::Number(lv), Value::Number(rv)) => {
                        Value::Number(((lv != 0) || (rv != 0)) as usize)
                    }
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
