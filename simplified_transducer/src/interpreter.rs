use crate::ast::{Stmt, Bexpr, Pexpr};
use std::collections::HashMap;

// Interpreter structure
pub struct Interpreter<'a> {
    variables: HashMap<String, i32>,
    word: &'a str,
    n: i32,
}

impl<'a> Interpreter<'a> {
    // Constructor for Interpreter
    pub fn new(word: &'a str) -> Self {
        let n = word.len() as i32;
        Self {
            variables: HashMap::new(),
            word,
            n,
        }
    }

    // Main function to interpret and execute a list of statements
    pub fn interpret(&mut self, stmts: Vec<Stmt>) {
        for stmt in stmts {
            self.execute(&stmt);
        }
    }

    // Function to execute a single statement
    fn execute(&mut self, stmt: &Stmt) {
        match stmt {
            // Handle Print statements
            Stmt::Print(expr) => {
                let value = self.evaluate_pexpr(expr);
                match value {
                    Value::Number(n) => panic!("Expected a string or variable, found a number: {}", n),
                    Value::Str(s) => {
                        let parts: Vec<&str> = s.split('.').collect();
                        if parts.len() == 2 && parts[1] == "label" {
                            if let Some(i) = self.variables.get(parts[0]) {
                                if let Some(character) = self.word.chars().nth(*i as usize) {
                                    print!("{}", character);
                                } else {
                                    println!("Index out of bounds");
                                }
                            } else {
                                println!("Variable {} not defined", parts[0]);
                            }
                        } else {
                            print!("{}", s.trim_end());
                        }
                    }
                }
            }
            // Handle For loops
            Stmt::For0(var, body) => {
                
                    for i in 0..self.n {
                        self.variables.insert(var.clone(), i);
                        self.execute_block(body);
                    }
                    self.variables.remove(var);
                } 
                
            Stmt::For1(var, body) =>{
                    for i in 0..self.n {
                        self.variables.insert(var.clone(), self.n - i - 1);
                        self.execute_block(body);
                    }
                    self.variables.remove(var);
                }
            
            // Handle If statements
            Stmt::If(condition, then_branch) => {
                if self.evaluate_condition(condition) {
                    self.execute_block(then_branch);
                }
            }
        }   
        }


    // Execute a block of statements
    fn execute_block(&mut self, stmts: &[Stmt]) {
        for stmt in stmts {
            self.execute(stmt);
        }
    }

    // Evaluate an expression and return a Value
    // Evaluate an expression and return a Value
fn evaluate_bexpr(&mut self, expr: &Bexpr) -> Value {
    match expr {
        //Bexpr::Number(n) => Value::Number(*n),
        Bexpr::Str(s) => Value::Str(s.clone()),
        Bexpr::Var(name) => {
            match self.variables.get(name) {
                Some(value) => Value::Number(*value),
                None => panic!("Variable {} not defined", name),
            }
        }
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
        // Evaluate comparison expressions
        Bexpr::LessEqual(left, right) | Bexpr::Less(left, right) | Bexpr::Equal(left, right) | Bexpr::NotEqual(left, right) |
        Bexpr::Greater(left, right) | Bexpr::GreaterEqual(left, right) => {
            let left_val = self.evaluate_bexpr(left);
            let right_val = self.evaluate_bexpr(right);

            match (left_val, right_val) {
                // Comparison between two variables that were in the hashmap
                (Value::Number(lv), Value::Number(rv)) => {
                    if self.is_variable(left) && self.is_variable(right) {
                        match expr {
                            Bexpr::LessEqual(_, _) => Value::Number((lv <= rv) as i32),
                            Bexpr::Less(_, _) => Value::Number((lv < rv) as i32),
                            Bexpr::Equal(_, _) => Value::Number((lv == rv) as i32),
                            Bexpr::NotEqual(_, _) => Value::Number((lv != rv) as i32),
                            Bexpr::Greater(_, _) => Value::Number((lv > rv) as i32),
                            Bexpr::GreaterEqual(_, _) => Value::Number((lv >= rv) as i32),
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
                            Bexpr::Equal(_, _) => Value::Number((ls == rs) as i32),
                            Bexpr::NotEqual(_, _) => Value::Number((ls != rs) as i32),
                            _ => panic!("Invalid comparison: only equality comparison with labels is allowed"),
                        }
                    } else {
                        panic!("Invalid comparison: label can only be compared to a string literal");
                    }
                }
                _ => panic!("Invalid comparison types"),
            }
        }
        Bexpr::Not(expr) => {
            let value = self.evaluate_bexpr(expr);
            match value {
                Value::Number(n) => Value::Number(if n == 0 { 1 } else { 0 }),
                Value::Str(_) => panic!("String in condition"),
            }
        }
        Bexpr::And(lhs, rhs) => {
            let left = self.evaluate_bexpr(lhs);
            let right = self.evaluate_bexpr(rhs);
            match (left, right) {
                (Value::Number(lv), Value::Number(rv)) => Value::Number(if lv != 0 && rv != 0 { 1 } else { 0 }),
                _ => panic!("Invalid comparison types"),
            }
        }
        Bexpr::Or(lhs, rhs) => {
            let left = self.evaluate_bexpr(lhs);
            let right = self.evaluate_bexpr(rhs);
            match (left, right) {
                (Value::Number(lv), Value::Number(rv)) => Value::Number(if lv != 0 || rv != 0 { 1 } else { 0 }),
                _ => panic!("Invalid comparison types"),
            }
        }
    }
}

fn evaluate_pexpr(&mut self, expr: &Pexpr) -> Value {
    match expr{
        Pexpr::Label(name) => {
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
        Pexpr::Str(s) => Value::Str(s.clone())
    }
}
    // Check if an expression is a variable
    fn is_variable(&self, expr: &Bexpr) -> bool {
        match expr {
            Bexpr::Var(name) => self.variables.contains_key(name),
            _ => false,
        }
    }

    // Check if an expression is a label
    fn is_label(&self, expr: &Bexpr) -> bool {
        matches!(expr, Bexpr::Label(_))
    }

    // Check if an expression is a string literal
    fn is_literal(&self, expr: &Bexpr) -> bool {
        matches!(expr, Bexpr::Str(_))
    }

    // Evaluate a condition expression and return a boolean
    fn evaluate_condition(&mut self, expr: &Bexpr) -> bool {
        match self.evaluate_bexpr(expr) {
            Value::Number(n) => n != 0,
            Value::Str(_) => panic!("String in condition"),
        }
    }
}

// Enum to represent the value of an expression
#[derive(Debug, PartialEq, PartialOrd)]
enum Value {
    Number(i32),
    Str(String),
}
