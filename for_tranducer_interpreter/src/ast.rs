pub enum Stmt {
    Print(Expr),              // Print statement
    For(String, i32, i32, Vec<Stmt>), // For loop statement
}

pub enum Expr {
    Number(i32),       // Numeric literals
    Var(String),       // Variable expressions
    Str(String),       // String literals
}
