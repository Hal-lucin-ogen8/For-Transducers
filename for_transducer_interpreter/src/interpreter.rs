use crate::ast::{Stmt, Expr};
use std::collections::HashMap;

pub struct Interpreter {
    variables: HashMap<String, i32>,
}

static WORD: &str = "hellloooo";

impl Interpreter {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, stmts: Vec<Stmt>) {
        for stmt in stmts {
            self.execute(&stmt);
        }
    }

    fn execute(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Print(expr) => {
                let value = self.evaluate_expr(expr);
                match value {
                    Value::Number(n) => panic!("Expected a string or variable, found a number: {}", n),
                    Value::Str(s) => {
                        let parts: Vec<&str> = s.split('.').collect();
                        if parts.len() == 2 && parts[1] == "label" {
                            if let Some(i) = self.variables.get(parts[0]) {
                                if let Some(character) = WORD.chars().nth(*i as usize) {
                                    println!("{}", character);
                                } else {
                                    println!("Index out of bounds");
                                }
                            } else {
                                println!("Variable {} not defined", parts[0]);
                            }
                        } else {
                            println!("{}", s);
                        }
                    }
                }
            }
            
            Stmt::For(var, start, end, body) => { // Prefix `var` with `_`
                // println!("{}: {} to {}", var, start, end);
                if start <= end {
                    for i in *start..*end { 
                        //add variables to hashmap
                        self.variables.insert(var.clone(), i);
                        // println!("{}: {}", var, i);
                        self.execute_block(body);
                    
                    }
                    // Remove the variable from the map after the loop
                    self.variables.remove(var);
                }

                else {
                    for i in *end..*start { 
                        //add variables to hashmap
                        self.variables.insert(var.clone(), start - i - 1);
                        // println!("{}: {}", var, i);
                        self.execute_block(body);
                    
                    }
                    // Remove the variable from the map after the loop
                    self.variables.remove(var);
                }
                
            }
            
            Stmt::If(condition, then_branch, else_branch) => {
                if self.evaluate_condition(condition) {
                    self.execute_block(then_branch);        // Prefix `then_branch`
                } 
                else {                                    // Prefix `else_branch`
                    self.execute_block(else_branch);
                }
            }
        }
    }

    fn execute_block(&mut self, stmts: &[Stmt]) {
        for stmt in stmts {
            self.execute(stmt);
        }
    }

    
fn evaluate_expr(&mut self, expr: &Expr) -> Value {
    match expr {
        Expr::Number(n) => Value::Number(*n),
        Expr::Str(s) => Value::Str(s.clone()),
        Expr::Var(name) => {
            match self.variables.get(name) {
                Some(value) => Value::Number(*value),
                None => panic!("Variable {} not defined", name),
            }
        }
        Expr::Label(name) => {
            match self.variables.get(name) {
                Some(value) => {
                    if let Some(character) = WORD.chars().nth(*value as usize) {
                        Value::Str(character.to_string())
                    } else {
                        panic!("Index out of bounds");
                    }
                }
                None => panic!("Variable {} not defined", name),
            }
        }
        Expr::LessEqual(left, right) | Expr::Less(left, right) | Expr::Equal(left, right) | Expr::NotEqual(left, right) => {
            let left_val = self.evaluate_expr(left);
            let right_val = self.evaluate_expr(right);
    
            match (left_val, right_val) {
                // Comparison between two variables that were in the hashmap
                (Value::Number(lv), Value::Number(rv)) => {
                    if self.is_variable(left) && self.is_variable(right) {
                        match expr {
                            Expr::LessEqual(_, _) => Value::Number((lv <= rv) as i32),
                            Expr::Less(_, _) => Value::Number((lv < rv) as i32),
                            Expr::Equal(_, _) => Value::Number((lv == rv) as i32),
                            Expr::NotEqual(_, _) => Value::Number((lv != rv) as i32), // Add this line
                            _ => panic!("Unexpected comparison"),
                        }
                    } else {
                        panic!("Invalid comparison: comparison between two labels are disallowed");
                    }
                }
                // Comparison of the type i.label == "some_char"
                (Value::Str(ls), Value::Str(rs)) => {
                    if self.is_label(left) && self.is_literal(right) || self.is_literal(left) && self.is_label(right) {
                        match expr {
                            Expr::Equal(_, _) => Value::Number((ls == rs) as i32),
                            Expr::NotEqual(_, _) => Value::Number((ls != rs) as i32), // Add this line
                            _ => panic!("Invalid comparison: only equality comparison with labels is allowed"),
                        }
                    } else {
                        panic!("Invalid comparison: label can only be compared to a string literal");
                    }
                }
                _ => panic!("Invalid comparison types"),
            }
        }
    }
}


    fn is_variable(&self, expr: &Expr) -> bool {
        match expr {
            Expr::Var(name) => self.variables.contains_key(name),
            _ => false,
        }
    }

    fn is_label(&self, expr: &Expr) -> bool {
        matches!(expr, Expr::Label(_))
    }

    fn is_literal(&self, expr: &Expr) -> bool {
        matches!(expr, Expr::Str(_))
    }
}

    impl Interpreter {
        fn evaluate_condition(&mut self, expr: &Expr) -> bool {
            match self.evaluate_expr(expr) {
                Value::Number(n) => n != 0,
                Value::Str(_) => panic!("String in condition"),
            }
        }
    }


#[derive(Debug, PartialEq, PartialOrd)]
enum Value {
    Number(i32),
    Str(String),
}
