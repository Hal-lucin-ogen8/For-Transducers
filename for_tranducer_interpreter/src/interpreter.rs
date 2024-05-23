use crate::ast::{Stmt, Expr};

pub struct Interpreter;

impl Interpreter {
    // Create a new interpreter
    pub fn new() -> Self {
        Interpreter
    }

    // Interpret a vector of statements (AST)
    pub fn interpret(&mut self, stmts: Vec<Stmt>) {
        for stmt in stmts {
            self.execute(&stmt);
        }
    }

    // Execute a single statement
    fn execute(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Print(expr) => {
                let value = self.evaluate(expr);
                println!("{}", value);
            }
            Stmt::For(_, start, end, body) => {
                for _ in *start..*end {
                    for stmt in body {
                        self.execute(stmt);
                    }
                }
            }
        }
    }

    // Evaluate an expression
    fn evaluate(&mut self, expr: &Expr) -> String {
        match expr {
            Expr::Number(n) => n.to_string(),
            Expr::Var(name) => panic!("Variables are not supported yet: {}", name),
            Expr::Str(s) => s.clone(),
        }
    }
}
