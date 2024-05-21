use std::collections::HashMap;
use crate::ast::{Stmt, Expr};

pub struct Interpreter {
    variables: HashMap<String, i32>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            variables: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, stmts: Vec<Stmt>) {
        for stmt in &stmts {
            self.execute(stmt);
        }
    }

    fn execute(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Print(expr) => {
                let value = self.evaluate(expr);
                println!("{}", value);
            }
            Stmt::For(var, start, end, body) => {
                for i in *start..*end {
                    self.variables.insert(var.clone(), i);
                    for stmt in body {
                        self.execute(stmt);
                    }
                }
            }
        }
    }

    fn evaluate(&self, expr: &Expr) -> i32 {
        match expr {
            Expr::Number(n) => *n,
            Expr::Var(name) => *self.variables.get(name).expect("Undefined variable"),
        }
    }
}
