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
                    Value::Number(n) => {
                        if let Some(character) = WORD.chars().nth(n as usize) {
                            println!("{}", character);
                        } else {
                            println!("Index out of bounds");
                        }
                    }
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
            Expr::LessEqual(left, right) => {
                let left_val = self.evaluate_expr(left);
                let right_val = self.evaluate_expr(right);
                Value::Number((left_val <= right_val) as i32)
            }
            Expr::Less(left, right) => {
                let left_val = self.evaluate_expr(left);
                let right_val = self.evaluate_expr(right);
                Value::Number((left_val < right_val) as i32)
            }
            Expr::Equal(left, right) => {
                let left_val = self.evaluate_expr(left);
                let right_val = self.evaluate_expr(right);
                Value::Number((left_val == right_val) as i32)
            }
        }
    }

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
