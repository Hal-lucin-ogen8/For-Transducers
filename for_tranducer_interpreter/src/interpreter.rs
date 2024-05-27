use crate::ast::{Stmt, Expr};

pub struct Interpreter;

impl Interpreter {
    // Create a new interpreter
    pub fn new() -> Self {
        Interpreter
    }

    // Interpret a vector of statements (AST)
    pub fn interpret(&mut self, stmts: Vec<Stmt>) -> String {
        for stmt in stmts {
            self.execute(&stmt);
        }
        "".to_string()
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

#[test]
fn test_for_loops() {
    let stmt = Stmt::For(
        "i".to_string(),
        0, 10,
        vec![Stmt::Print(Expr::Number(42))],
    );
    let mut interpreter = Interpreter::new();
    let output = interpreter.interpret(vec![stmt]);
    assert_eq!(output, "42".to_string().repeat(10));
}

