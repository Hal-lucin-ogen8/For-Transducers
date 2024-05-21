use std::collections::HashMap; // Import HashMap
use crate::ast::{Stmt, Expr};  // Import AST definitions

// Define the interpreter struct
pub struct Interpreter {
    variables: HashMap<String, i32>, // HashMap to store variable values
}

impl Interpreter {
    // Create a new interpreter
    pub fn new() -> Self {
        Interpreter {
            variables: HashMap::new(),
        }
    }

    // Interpret a vector of statements (AST)
    pub fn interpret(&mut self, stmts: Vec<Stmt>) {
        for stmt in &stmts {
            self.execute(stmt); // Execute each statement
        }
    }

    // Execute a single statement
    fn execute(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Print(expr) => {
                let value = self.evaluate(expr); // Evaluate the expression
                println!("{}", value); // Print the result
            }
            Stmt::For(var, start, end, body) => {
                for i in *start..*end {
                    self.variables.insert(var.clone(), i); // Set the loop variable
                    for stmt in body {
                        self.execute(stmt); // Execute each statement in the loop body
                    }
                }
            }
        }
    }

    // Evaluate an expression and return its value
    fn evaluate(&self, expr: &Expr) -> i32 {
        match expr {
            Expr::Number(n) => *n, // Return the numeric value
            Expr::Var(name) => *self.variables.get(name).expect("Undefined variable"), // Get the variable value
        }
    }
}
