// Define the possible expressions in the language
#[derive(Debug)]
pub enum Expr {
    Number(i32),     // Numeric literal
    Var(String),     // Variable
}

// Define the possible statements in the language
#[derive(Debug)]
pub enum Stmt {
    Print(Expr),                    // Print statement
    For(String, i32, i32, Vec<Stmt>), // For loop
}
