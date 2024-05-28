use crate::ast::{Stmt, Expr};
use std::collections::HashMap;

pub struct Interpreter {
    variables: HashMap<String, i32>,
}

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
                    Value::Number(n) => println!("{}", n),
                    Value::Str(s) => println!("{}", s),
                }
            }
            Stmt::For(var, start, end, body) => { // Prefix `var` with `_`
                for i in *start..*end { // Prefix `i`
                    //add variables to hashmap
                    self.variables.insert(var.clone(), i);
                    // println!("{}: {}", var, i);
                    self.execute_block(body);
                
                }
                // Remove the variable from the map after the loop
                self.variables.remove(var);
            }
            Stmt::If(condition, then_branch) => {
                if self.evaluate_condition(condition) {
                    self.execute_block(then_branch);
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
